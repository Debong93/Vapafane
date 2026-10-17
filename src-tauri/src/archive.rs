//! ZIP-Backups des ganzen Server-Ordners, Entpacken von ZIP/tar und Wiederherstellen.

use std::fs;
use std::io::{self, BufReader, BufWriter, Read};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

use crate::config::{Config, STAGING_NAME};
use crate::server;
use crate::util::{self, Result, err};

/// Fortschritt: Anteil 0..1 und kurzer Text.
pub type Progress<'a> = &'a (dyn Fn(f64, &str) + Sync);

/// Bereits komprimierte Dateien werden im ZIP nur abgelegt, nicht nochmal gepackt.
const STORED_EXT: &[&str] = &[
    "mca", "jar", "zip", "gz", "zst", "png", "sqlite", "mcc", "xz", "7z", "mrpack", "jpg", "ogg",
];

fn stored(path: &Path) -> bool {
    path.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .map(|e| STORED_EXT.contains(&e.as_str()))
        .unwrap_or(false)
}

// ---------------------------------------------------------------- Backup

/// Packt den ganzen Server-Ordner in `<Backups>/<Name>/<Name>_JJJJ-MM-TT_HH-MM.zip`.
pub fn backup_server(dir: &Path, name: &str, cfg: &Config, progress: Progress) -> Result<PathBuf> {
    use zip::CompressionMethod;
    use zip::write::SimpleFileOptions;

    let dest_dir = cfg.backup_dir_for(name);
    fs::create_dir_all(&dest_dir)?;
    let safe = util::safe_name(name);
    let mut final_path = dest_dir.join(format!("{safe}_{}.zip", util::now_stamp()));
    if final_path.exists() {
        let sec = chrono::Local::now().format("%S");
        final_path = dest_dir.join(format!("{safe}_{}-{sec}.zip", util::now_stamp()));
    }
    let tmp = dest_dir.join(format!(".{}.part", final_path.file_name().unwrap().to_string_lossy()));

    let root_name = util::canonical(dir)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| safe.clone());
    let files: Vec<(PathBuf, u64)> = walkdir::WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| e.file_name() != STAGING_NAME)
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let len = e.metadata().map(|m| m.len()).unwrap_or(0);
            (e.into_path(), len)
        })
        .collect();
    let total: u64 = files.iter().map(|(_, l)| *l).sum::<u64>().max(1);

    let result = (|| -> Result<()> {
        let out = BufWriter::with_capacity(1 << 20, fs::File::create(&tmp)?);
        let mut z = zip::ZipWriter::new(out);
        let mut done = 0u64;
        let mut buf = vec![0u8; 1 << 20];
        for (path, len) in &files {
            let rel = path.strip_prefix(dir).unwrap_or(path);
            let mut name_in_zip = root_name.clone();
            for c in rel.components() {
                name_in_zip.push('/');
                name_in_zip.push_str(&c.as_os_str().to_string_lossy());
            }
            // Datei kann während des Backups verschwinden (z. B. Lock-Dateien)
            let Ok(mut f) = fs::File::open(path) else {
                continue;
            };
            let method = if stored(path) {
                CompressionMethod::Stored
            } else {
                CompressionMethod::Deflated
            };
            let mut opts = SimpleFileOptions::default()
                .compression_method(method)
                .large_file(*len >= 0xF000_0000);
            if method == CompressionMethod::Deflated {
                opts = opts.compression_level(Some(6));
            }
            if let Ok(meta) = f.metadata() {
                if let Ok(modified) = meta.modified()
                    && let Some(dt) = zip_time(modified)
                {
                    opts = opts.last_modified_time(dt);
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    opts = opts.unix_permissions(meta.permissions().mode() & 0o777);
                }
            }
            z.start_file(name_in_zip, opts)?;
            loop {
                let n = match f.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e.into()),
                };
                io::Write::write_all(&mut z, &buf[..n])?;
            }
            done += len;
            progress(
                done as f64 / total as f64,
                &format!("Sichere {}", short(&rel.to_string_lossy())),
            );
        }
        let mut out = z.finish()?;
        io::Write::flush(&mut out)?;
        out.get_ref().sync_all()?;
        Ok(())
    })();
    if let Err(e) = result {
        let _ = fs::remove_file(&tmp);
        return Err(e);
    }
    fs::rename(&tmp, &final_path)?;
    prune_backups(cfg, name)?;
    Ok(final_path)
}

fn zip_time(t: SystemTime) -> Option<zip::DateTime> {
    use chrono::{Datelike, Timelike};
    let dt: chrono::DateTime<chrono::Local> = t.into();
    zip::DateTime::from_date_and_time(
        dt.year().clamp(1980, 2107) as u16,
        dt.month() as u8,
        dt.day() as u8,
        dt.hour() as u8,
        dt.minute() as u8,
        dt.second().min(58) as u8,
    )
    .ok()
}

fn short(s: &str) -> String {
    let n = s.chars().count();
    if n <= 60 {
        s.to_string()
    } else {
        format!("…{}", s.chars().skip(n - 59).collect::<String>())
    }
}

pub fn list_backups(cfg: &Config, name: &str) -> Vec<PathBuf> {
    let Ok(rd) = fs::read_dir(cfg.backup_dir_for(name)) else {
        return vec![];
    };
    let mut items: Vec<(SystemTime, PathBuf)> = rd
        .filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            n.ends_with(".zip") && !n.starts_with('.')
        })
        .filter_map(|e| Some((e.metadata().ok()?.modified().ok()?, e.path())))
        .collect();
    items.sort_by_key(|a| std::cmp::Reverse(a.0));
    items.into_iter().map(|(_, p)| p).collect()
}

pub fn prune_backups(cfg: &Config, name: &str) -> Result<()> {
    let keep = cfg.backup_keep.max(1);
    for old in list_backups(cfg, name).into_iter().skip(keep) {
        fs::remove_file(old)?;
    }
    // Reste abgebrochener Backups (älter als einen Tag) aufräumen
    if let Ok(rd) = fs::read_dir(cfg.backup_dir_for(name)) {
        for e in rd.filter_map(|e| e.ok()) {
            let n = e.file_name().to_string_lossy().into_owned();
            let old = e
                .metadata()
                .and_then(|m| m.modified())
                .map(|t| t.elapsed().map(|d| d.as_secs() > 86_400).unwrap_or(false))
                .unwrap_or(false);
            if n.starts_with('.') && n.ends_with(".part") && old {
                let _ = fs::remove_file(e.path());
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- Entpacken

/// Nur normale Pfadteile erlauben (kein `..`, kein `/` am Anfang, kein Laufwerk).
fn safe_relative(name: &Path) -> Option<PathBuf> {
    let mut out = PathBuf::new();
    for c in name.components() {
        match c {
            Component::Normal(p) => out.push(p),
            Component::CurDir => {}
            _ => return None,
        }
    }
    if out.as_os_str().is_empty() { None } else { Some(out) }
}

fn is_archive_name(lower: &str) -> Option<&'static str> {
    if lower.ends_with(".zip") || lower.ends_with(".mrpack") {
        Some("zip")
    } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
        Some("tgz")
    } else if lower.ends_with(".tar.zst") || lower.ends_with(".tzst") {
        Some("tzst")
    } else if lower.ends_with(".tar") {
        Some("tar")
    } else {
        None
    }
}

/// Entpackt ZIP oder tar(.gz/.zst) nach `dest` (darf noch nicht existieren).
/// Gibt den eigentlichen Server-Ordner darin zurück.
pub fn extract_archive(archive: &Path, dest: &Path, progress: Progress) -> Result<PathBuf> {
    fs::create_dir_all(dest)?;
    let lower = archive.to_string_lossy().to_lowercase();
    match is_archive_name(&lower) {
        Some("zip") => extract_zip(archive, dest, progress)?,
        Some(kind) => {
            let f = BufReader::new(fs::File::open(archive)?);
            let total = fs::metadata(archive)?.len().max(1);
            let counter = CountingReader { inner: f, read: 0 };
            let reader: Box<dyn Read> = match kind {
                "tgz" => Box::new(flate2::read::GzDecoder::new(counter)),
                "tzst" => {
                    Box::new(ruzstd::decoding::StreamingDecoder::new(counter).map_err(|e| util::Error(e.to_string()))?)
                }
                _ => Box::new(counter),
            };
            extract_tar(reader, dest, total, progress)?;
        }
        None => return err("Unbekanntes Archivformat (erlaubt: .zip, .tar, .tar.gz, .tar.zst)"),
    }
    Ok(server::find_server_root(dest))
}

struct CountingReader<R> {
    inner: R,
    read: u64,
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.read += n as u64;
        Ok(n)
    }
}

fn extract_zip(archive: &Path, dest: &Path, progress: Progress) -> Result<()> {
    let mut z = zip::ZipArchive::new(BufReader::new(fs::File::open(archive)?))?;
    let mut total = 0u64;
    for i in 0..z.len() {
        total += z.by_index_raw(i).map(|f| f.size()).unwrap_or(0);
    }
    let total = total.max(1);
    let mut done = 0u64;
    for i in 0..z.len() {
        let mut f = z.by_index(i)?;
        let raw_name = f.name().to_string();
        let Some(rel) = safe_relative(Path::new(&raw_name.replace('\\', "/"))) else {
            return err(format!("Unsichere Pfadangabe im Archiv: {raw_name}"));
        };
        let target = dest.join(&rel);
        if f.is_dir() {
            fs::create_dir_all(&target)?;
            continue;
        }
        if f.is_symlink() {
            continue; // Verknüpfungen werden nicht übernommen
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut out = BufWriter::new(fs::File::create(&target)?);
        io::copy(&mut f, &mut out)?;
        drop(out);
        #[cfg(unix)]
        if let Some(mode) = f.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            let mode = (mode & 0o777) | 0o600;
            let _ = fs::set_permissions(&target, fs::Permissions::from_mode(mode));
        }
        done += f.size();
        progress(done as f64 / total as f64, &format!("Entpacke {}", short(&raw_name)));
    }
    Ok(())
}

fn extract_tar(reader: Box<dyn Read>, dest: &Path, _total: u64, progress: Progress) -> Result<()> {
    let mut ar = tar::Archive::new(reader);
    let mut count = 0u64;
    for entry in ar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        let Some(rel) = safe_relative(&path) else {
            return err(format!("Unsichere Pfadangabe im Archiv: {}", path.display()));
        };
        let target = dest.join(&rel);
        match entry.header().entry_type() {
            tar::EntryType::Directory => fs::create_dir_all(&target)?,
            tar::EntryType::Regular | tar::EntryType::Continuous | tar::EntryType::GNUSparse => {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut out = BufWriter::new(fs::File::create(&target)?);
                io::copy(&mut entry, &mut out)?;
                drop(out);
                #[cfg(unix)]
                if let Ok(mode) = entry.header().mode() {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = fs::set_permissions(&target, fs::Permissions::from_mode((mode & 0o777) | 0o600));
                }
            }
            _ => continue, // Links und Sonderdateien werden übersprungen
        }
        count += 1;
        if count.is_multiple_of(50) {
            progress(0.5, &format!("Entpacke {}", short(&rel.to_string_lossy())));
        }
    }
    Ok(())
}

/// Importiert ein Archiv als neuen Server in `MC_Servers/<Name>`.
pub fn import_archive(archive: &Path, name: &str, progress: Progress) -> Result<PathBuf> {
    let base = crate::config::servers_dir();
    fs::create_dir_all(&base)?;
    let target = util::unique_dir(&base, &util::safe_name(name));
    let staging = PathBuf::from(format!("{}.import", target.display()));
    let _ = fs::remove_dir_all(&staging);
    let res = extract_archive(archive, &staging, progress).and_then(|root| {
        fs::rename(&root, &target)?;
        Ok(())
    });
    let _ = fs::remove_dir_all(&staging);
    res.map(|_| target)
}

/// Stellt ein Backup in `dir` wieder her. Der bisherige Ordner bleibt daneben als
/// `<Ordner>_vor_Wiederherstellung_<Datum>` erhalten (Rückgabewert).
pub fn restore_backup(archive: &Path, dir: &Path, progress: Progress) -> Result<Option<PathBuf>> {
    let dir = util::canonical(dir);
    let staging = PathBuf::from(format!("{}.restore", dir.display()));
    let _ = fs::remove_dir_all(&staging);
    let res = (|| -> Result<Option<PathBuf>> {
        let root = extract_archive(archive, &staging, progress)?;
        let mut aside = None;
        if dir.exists() {
            let parent = dir.parent().unwrap_or(Path::new("."));
            let base = dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let a = util::unique_dir(parent, &format!("{base}_vor_Wiederherstellung_{}", util::now_stamp()));
            fs::rename(&dir, &a)?;
            aside = Some(a);
        }
        if let Err(e) = fs::rename(&root, &dir) {
            // Zurückrollen, damit der Server nicht verschwindet
            if let Some(a) = &aside {
                let _ = fs::rename(a, &dir);
            }
            return Err(e.into());
        }
        Ok(aside)
    })();
    let _ = fs::remove_dir_all(&staging);
    res
}

/// Datum aus dem Backup-Namen lesbar machen: "17.10.2026  12:22 Uhr".
pub fn backup_label(path: &Path) -> String {
    use regex::Regex;
    use std::sync::LazyLock;
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d{4})-(\d{2})-(\d{2})_(\d{2})-(\d{2})").unwrap());
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    if let Some(c) = RE.captures(&name) {
        return format!("{}.{}.{}  {}:{} Uhr", &c[3], &c[2], &c[1], &c[4], &c[5]);
    }
    fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|t| {
            chrono::DateTime::<chrono::Local>::from(t)
                .format("%d.%m.%Y  %H:%M Uhr")
                .to_string()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backup_and_restore_roundtrip() {
        let base = std::env::temp_dir().join(format!("redcraft-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let srv = base.join("Welt Server");
        fs::create_dir_all(srv.join("world/region")).unwrap();
        fs::write(srv.join("server.properties"), "motd=Hallo\n").unwrap();
        fs::write(srv.join("world/region/r.0.0.mca"), vec![7u8; 100_000]).unwrap();
        fs::create_dir_all(srv.join(STAGING_NAME)).unwrap();
        fs::write(srv.join(STAGING_NAME).join("x.jar"), "weg").unwrap();
        let mut cfg = Config::default();
        cfg.backup_dir = base.join("Backups").to_string_lossy().into_owned();
        cfg.backup_keep = 1;
        let noop = |_: f64, _: &str| {};
        let zip1 = backup_server(&srv, "Welt Server", &cfg, &noop).unwrap();
        assert!(zip1.file_name().unwrap().to_string_lossy().starts_with("Welt_Server_"));
        // Zweites Backup in derselben Minute bekommt Sekunden angehängt, das erste wird aufgeräumt
        let zip2 = backup_server(&srv, "Welt Server", &cfg, &noop).unwrap();
        assert_ne!(zip1, zip2);
        assert_eq!(list_backups(&cfg, "Welt Server"), vec![zip2.clone()]);

        fs::write(srv.join("server.properties"), "motd=Kaputt\n").unwrap();
        let aside = restore_backup(&zip2, &srv, &noop).unwrap().unwrap();
        assert_eq!(
            fs::read_to_string(srv.join("server.properties")).unwrap(),
            "motd=Hallo\n"
        );
        assert_eq!(fs::read(srv.join("world/region/r.0.0.mca")).unwrap().len(), 100_000);
        assert!(!srv.join(STAGING_NAME).exists());
        assert_eq!(
            fs::read_to_string(aside.join("server.properties")).unwrap(),
            "motd=Kaputt\n"
        );
        assert!(
            aside
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains("_vor_Wiederherstellung_")
        );
        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn rejects_zip_slip() {
        let base = std::env::temp_dir().join(format!("redcraft-slip-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let zpath = base.join("boese.zip");
        {
            let mut z = zip::ZipWriter::new(fs::File::create(&zpath).unwrap());
            z.start_file("../../ausbruch.txt", zip::write::SimpleFileOptions::default())
                .unwrap();
            io::Write::write_all(&mut z, b"x").unwrap();
            z.finish().unwrap();
        }
        let noop = |_: f64, _: &str| {};
        let res = extract_archive(&zpath, &base.join("ziel"), &noop);
        assert!(res.is_err());
        assert!(!base.join("ausbruch.txt").exists() && !std::env::temp_dir().join("ausbruch.txt").exists());
        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn imports_zip_with_top_folder() {
        let base = std::env::temp_dir().join(format!("redcraft-imp-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let zpath = base.join("Mein Server_2026-10-01_05-00.zip");
        {
            let mut z = zip::ZipWriter::new(fs::File::create(&zpath).unwrap());
            let o = zip::write::SimpleFileOptions::default();
            z.add_directory("MeinServer/", o).unwrap();
            z.start_file("MeinServer/server.properties", o).unwrap();
            io::Write::write_all(&mut z, b"motd=x\n").unwrap();
            z.start_file("MeinServer/world/level.dat", o).unwrap();
            io::Write::write_all(&mut z, b"daten").unwrap();
            z.finish().unwrap();
        }
        let dest = base.join("ziel");
        let noop = |_: f64, _: &str| {};
        let root = extract_archive(&zpath, &dest, &noop).unwrap();
        assert_eq!(root, dest.join("MeinServer"));
        assert_eq!(fs::read_to_string(root.join("world/level.dat")).unwrap(), "daten");
        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn imports_tar_gz() {
        let base = std::env::temp_dir().join(format!("redcraft-tgz-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("src/S")).unwrap();
        fs::write(base.join("src/S/server.properties"), "motd=y\n").unwrap();
        let tgz = base.join("s.tar.gz");
        {
            let enc = flate2::write::GzEncoder::new(fs::File::create(&tgz).unwrap(), flate2::Compression::fast());
            let mut t = tar::Builder::new(enc);
            t.append_dir_all("S", base.join("src/S")).unwrap();
            t.into_inner().unwrap().finish().unwrap();
        }
        let noop = |_: f64, _: &str| {};
        let root = extract_archive(&tgz, &base.join("ziel"), &noop).unwrap();
        assert_eq!(fs::read_to_string(root.join("server.properties")).unwrap(), "motd=y\n");
        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn finds_single_top_folder() {
        let base = std::env::temp_dir().join(format!("redcraft-root-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("MeinServer/world")).unwrap();
        fs::write(base.join("MeinServer/server.properties"), "").unwrap();
        assert_eq!(server::find_server_root(&base), base.join("MeinServer"));
        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn rejects_escaping_paths() {
        assert!(safe_relative(Path::new("../x")).is_none());
        assert!(safe_relative(Path::new("/etc/passwd")).is_none());
        assert!(safe_relative(Path::new("a/../../b")).is_none());
        assert_eq!(safe_relative(Path::new("./a/b.txt")), Some(PathBuf::from("a/b.txt")));
    }
}
