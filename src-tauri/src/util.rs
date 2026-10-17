//! Kleine Helfer: Fehlertyp, Dateinamen, Formatierung.

use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Fehler mit einer verständlichen deutschen Meldung.
#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<E: std::error::Error> From<E> for Error {
    fn from(e: E) -> Self {
        Error(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Fehler aus einem Text.
pub fn err<T>(msg: impl Into<String>) -> Result<T> {
    Err(Error(msg.into()))
}

/// Pfad auflösen (Links folgen), ohne zu scheitern, wenn er fehlt.
pub fn canonical(p: &Path) -> PathBuf {
    let c = fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    strip_unc(c)
}

/// Windows hängt bei canonicalize ein `\\?\` vorne an – das stört Java und die Anzeige.
fn strip_unc(p: PathBuf) -> PathBuf {
    #[cfg(windows)]
    {
        let s = p.to_string_lossy();
        if let Some(rest) = s.strip_prefix(r"\\?\") {
            if !rest.starts_with("UNC\\") {
                return PathBuf::from(rest);
            }
        }
    }
    p
}

/// Name, der sich gefahrlos als Datei- oder Ordnername eignet.
pub fn safe_name(name: &str) -> String {
    let mut out = String::new();
    let mut last_us = false;
    for ch in name.trim().chars() {
        if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '-' {
            out.push(ch);
            last_us = false;
        } else if !last_us {
            out.push('_');
            last_us = true;
        }
    }
    let out = out.trim_matches(|c| c == '.' || c == '_').to_string();
    if out.is_empty() { "Server".into() } else { out }
}

/// Freien Ordnernamen finden: name, name_2, name_3 …
pub fn unique_dir(base: &Path, name: &str) -> PathBuf {
    let mut path = base.join(name);
    let mut i = 2;
    while path.exists() {
        path = base.join(format!("{name}_{i}"));
        i += 1;
    }
    path
}

/// Datei so schreiben, dass sie nie halb fertig ist.
pub fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

pub fn dir_size(path: &Path) -> u64 {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

pub fn free_space(path: &Path) -> u64 {
    use sysinfo::Disks;
    let path = canonical(path);
    let disks = Disks::new_with_refreshed_list();
    disks
        .list()
        .iter()
        .filter(|d| path.starts_with(d.mount_point()))
        .max_by_key(|d| d.mount_point().as_os_str().len())
        .map(|d| d.available_space())
        .unwrap_or(0)
}

/// IP-Adresse im Heimnetz (es wird nichts gesendet).
pub fn local_ip() -> String {
    std::net::UdpSocket::bind("0.0.0.0:0")
        .and_then(|s| {
            s.connect("192.0.2.1:9")?;
            s.local_addr()
        })
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".into())
}

pub fn now_stamp() -> String {
    chrono::Local::now().format("%Y-%m-%d_%H-%M").to_string()
}

pub fn random_hex(bytes: usize) -> String {
    use rand::RngCore;
    let mut buf = vec![0u8; bytes];
    rand::rng().fill_bytes(&mut buf);
    hex::encode(buf)
}

/// Programm ohne aufpoppendes Konsolenfenster vorbereiten (Windows).
pub fn command(program: impl AsRef<std::ffi::OsStr>) -> std::process::Command {
    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

/// Größe lesbar: "6,8 GB".
pub fn fmt_bytes(n: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut v = n as f64;
    let mut i = 0;
    while v >= 1024.0 && i < units.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i >= 3 {
        format!("{v:.1} {}", units[i]).replace('.', ",")
    } else {
        format!("{v:.0} {}", units[i])
    }
}
