//! Einstellungen aus einer anderen Minecraft-Installation übernehmen: Optionen, Mod-Einstellungen,
//! Shader, Ressourcenpakete, Server-Liste und auf Wunsch Welten. Mods nie – die kommen passend
//! vom Server.
//!
//! Quellen sind Instanzen der Modrinth-App, der offizielle Launcher (Standard-Ordner und Profile
//! mit eigenem Ordner) und andere Vapafane-Clients. Die Quelle wird nur gelesen, nie verändert –
//! auch an den Dateien der Modrinth-App (Datenbank, Instanzen) ändert Vapafane nichts.

use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::client;
use crate::config;
use crate::util::{self, Result, err};

/// Was übernommen werden kann: (Schlüssel, Bezeichnung).
pub const PARTS: [(&str, &str); 6] = [
    ("options", "Optionen & Tastenbelegung"),
    ("config", "Mod-Einstellungen"),
    ("shaders", "Shader"),
    ("resourcepacks", "Ressourcenpakete"),
    ("servers", "Server-Liste"),
    ("worlds", "Welten"),
];

/// Dateien und Ordner (relativ zum Spielordner), die zu einem Teil gehören.
fn members(part: &str) -> (&'static [&'static str], &'static [&'static str]) {
    match part {
        "options" => (
            &["options.txt", "optionsof.txt", "hotbar.nbt", "command_history.txt"],
            &[],
        ),
        "config" => (&[], &["config"]),
        // Die Shader-Auswahl steht bei Iris/Oculus im config-Ordner
        "shaders" => (
            &[
                "optionsshaders.txt",
                "config/iris.properties",
                "config/oculus.properties",
            ],
            &["shaderpacks"],
        ),
        "resourcepacks" => (&[], &["resourcepacks"]),
        "servers" => (&["servers.dat"], &[]),
        "worlds" => (&[], &["saves"]),
        _ => (&[], &[]),
    }
}

/// Nie kopieren: Sperrdatei einer gerade offenen Welt.
const SKIP: [&str; 1] = ["session.lock"];

/// Gewählte Quelle und Teile (kommt so aus der Oberfläche).
#[derive(Clone, Debug, Deserialize)]
pub struct Take {
    pub dir: PathBuf,
    pub parts: Vec<String>,
}

impl Take {
    pub fn has(&self, part: &str) -> bool {
        self.parts.iter().any(|p| p == part)
    }

    /// Server-Liste der Quelle, falls sie übernommen werden soll.
    pub fn servers(&self) -> Option<Vec<u8>> {
        self.has("servers")
            .then(|| fs::read(self.dir.join("servers.dat")).ok())
            .flatten()
    }
}

// ---------------------------------------------------------------- Quellen finden

/// Ordner „profiles“ der Modrinth-App (Linux, Flatpak, Windows; auch der frühere Name).
/// Einen in der App selbst gewählten Speicherort kennt nur deren Datenbank – den wählt man
/// dann über „Anderer Ordner …“.
pub fn modrinth_profile_dirs() -> Vec<PathBuf> {
    let h = config::home();
    let mut bases = vec![];
    if config::test_home().is_none()
        && let Some(d) = dirs::data_dir()
    {
        // Windows: %APPDATA%, Linux: $XDG_DATA_HOME bzw. ~/.local/share
        bases.push(d.join("ModrinthApp"));
        bases.push(d.join("com.modrinth.theseus"));
    }
    bases.push(h.join(".local/share/ModrinthApp"));
    bases.push(h.join(".local/share/com.modrinth.theseus"));
    bases.push(h.join(".var/app/com.modrinth.ModrinthApp/data/ModrinthApp"));
    let mut seen = HashSet::new();
    bases
        .into_iter()
        .map(|b| b.join("profiles"))
        .filter(|p| p.is_dir())
        .filter(|p| seen.insert(util::canonical(p)))
        .collect()
}

fn subdirs(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                .map(|e| e.path())
                .collect()
        })
        .unwrap_or_default();
    v.sort();
    v
}

fn folder_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Instanzen der Modrinth-App, die aus einem früheren Vapafane-Paket dieses Servers stammen.
/// Die App benennt den Ordner nach der Paket-Datei: „Name“, bei Doppelten „Name (1)“ usw.
/// Neueste zuerst.
pub fn modrinth_instances_for(stem: &str, profile_dirs: &[PathBuf]) -> Vec<PathBuf> {
    let stem = stem.trim_end_matches(['.', ' ']);
    let matches = |n: &str| {
        n == stem
            || n.strip_prefix(stem)
                .and_then(|r| r.strip_prefix(" ("))
                .and_then(|r| r.strip_suffix(')'))
                .is_some_and(|d| !d.is_empty() && d.chars().all(|c| c.is_ascii_digit()))
    };
    let mut v: Vec<PathBuf> = profile_dirs
        .iter()
        .flat_map(|p| subdirs(p))
        .filter(|d| matches(&folder_name(d)))
        .collect();
    v.sort_by_key(|d| std::cmp::Reverse(last_used(d)));
    v
}

/// Zuletzt benutzt: jüngste Änderung an Optionen, Server-Liste oder Ordner (Unix-Sekunden).
fn last_used(dir: &Path) -> u64 {
    ["options.txt", "servers.dat", "logs/latest.log", ""]
        .iter()
        .filter_map(|f| fs::metadata(dir.join(f)).ok())
        .filter_map(|m| m.modified().ok())
        .filter_map(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .max()
        .unwrap_or(0)
}

/// Größe je Teil, nur Teile mit Inhalt.
pub fn part_sizes(dir: &Path) -> BTreeMap<&'static str, u64> {
    PARTS
        .iter()
        .filter_map(|(key, _)| {
            let (files, dirs) = members(key);
            let mut size = 0u64;
            let (mut file_any, mut dir_any) = (false, false);
            for f in files {
                if let Ok(m) = fs::symlink_metadata(dir.join(f))
                    && m.is_file()
                {
                    size += m.len();
                    file_any = true;
                }
            }
            for d in dirs {
                let p = dir.join(d);
                if p.is_dir() {
                    let s = util::dir_size(&p);
                    // leere Ordner zählen nicht
                    if s > 0 {
                        size += s;
                        dir_any = true;
                    }
                }
            }
            // Teile mit Ordnern zählen nur, wenn darin etwas liegt (Shader: nicht nur die Einstellung)
            let any = if dirs.is_empty() { file_any } else { dir_any };
            any.then_some((*key, size))
        })
        .collect()
}

/// Profile des offiziellen Launchers mit eigenem Spielordner (ohne die von Vapafane).
fn launcher_profile_dirs(launcher: &Path) -> Vec<(String, PathBuf)> {
    let Ok(text) = fs::read_to_string(launcher.join("launcher_profiles.json")) else {
        return vec![];
    };
    let v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    v.get("profiles")
        .and_then(|p| p.as_object())
        .map(|p| {
            p.iter()
                .filter(|(k, _)| !k.starts_with("vapafane-"))
                .filter_map(|(_, p)| {
                    let dir = p.get("gameDir")?.as_str()?.trim();
                    let name = p.get("name").and_then(|n| n.as_str()).unwrap_or("").trim();
                    (!dir.is_empty()).then(|| {
                        let label = if name.is_empty() {
                            folder_name(Path::new(dir))
                        } else {
                            name.to_string()
                        };
                        (label, PathBuf::from(dir))
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Alle Quellen mit dem, was sie zu bieten haben – zuletzt benutzte zuerst.
/// `exclude`: Ordner, in den gerade eingerichtet wird.
pub fn sources(exclude: Option<&Path>) -> Vec<Value> {
    let mut found: Vec<(&str, String, PathBuf)> = vec![];
    for p in modrinth_profile_dirs() {
        for d in subdirs(&p) {
            found.push(("modrinth", format!("Modrinth-App: {}", folder_name(&d)), d));
        }
    }
    for l in client::launcher_dirs() {
        found.push(("launcher", "Minecraft-Launcher (Standard)".into(), l.clone()));
        for (name, d) in launcher_profile_dirs(&l) {
            found.push(("launcher", format!("Minecraft-Launcher: {name}"), d));
        }
    }
    for c in client::list() {
        if c.get("target").and_then(|t| t.as_str()) == Some("modrinth") {
            continue; // da liegt nur das Paket
        }
        if let (Some(dir), Some(name)) = (
            c.get("dir").and_then(|d| d.as_str()),
            c.get("name").and_then(|n| n.as_str()),
        ) {
            found.push(("vapafane", format!("Vapafane: {name}"), PathBuf::from(dir)));
        }
    }
    let skip = exclude.map(util::canonical);
    let mut seen = HashSet::new();
    let mut out: Vec<(u64, Value)> = found
        .into_iter()
        .filter(|(_, _, d)| d.is_dir())
        .filter(|(_, _, d)| {
            let c = util::canonical(d);
            skip.as_ref() != Some(&c) && seen.insert(c)
        })
        .filter_map(|(kind, label, dir)| {
            let parts = part_sizes(&dir);
            (!parts.is_empty()).then(|| {
                (
                    last_used(&dir),
                    json!({ "kind": kind, "label": label, "dir": dir, "parts": parts }),
                )
            })
        })
        .collect();
    out.sort_by_key(|(t, _)| std::cmp::Reverse(*t));
    out.into_iter().map(|(_, v)| v).collect()
}

/// Einen selbst gewählten Ordner als Quelle beschreiben (z. B. Modrinth-App mit eigenem Speicherort).
pub fn describe(dir: &Path) -> Result<Value> {
    let parts = part_sizes(dir);
    if parts.is_empty() {
        return err(crate::sprache::t!("err-nichts-zu-uebernehmen"));
    }
    Ok(json!({ "kind": "folder", "label": crate::sprache::t!("cli-quelle-ordner", ordner = folder_name(dir).to_string()), "dir": dir, "parts": parts }))
}

// ---------------------------------------------------------------- Dateien sammeln und übernehmen

/// Pfad relativ zum Spielordner mit „/“ – nur, wenn er gültiges UTF-8 ist.
fn relative(base: &Path, p: &Path) -> Option<String> {
    let rel = p.strip_prefix(base).ok()?;
    let parts: Option<Vec<&str>> = rel.components().map(|c| c.as_os_str().to_str()).collect();
    let parts = parts?;
    (!parts.is_empty()).then(|| parts.join("/"))
}

/// Dateien der gewählten Teile (ohne servers.dat – die wird zusammengeführt).
/// Ergebnis: relativer Pfad → Datei in der Quelle.
pub fn gather(take: &Take) -> Result<BTreeMap<String, PathBuf>> {
    let src = &take.dir;
    if !src.is_dir() {
        return err(crate::sprache::t!("err-ordner-weg", ordner = src.display().to_string()));
    }
    let mut out = BTreeMap::new();
    for part in &take.parts {
        if !PARTS.iter().any(|(k, _)| k == part) {
            return err(crate::sprache::t!("err-teil-unbekannt", teil = part.to_string()));
        }
        let (files, dirs) = members(part);
        for f in files {
            let p = src.join(f);
            if *f != "servers.dat" && fs::symlink_metadata(&p).map(|m| m.is_file()).unwrap_or(false) {
                out.insert((*f).to_string(), p);
            }
        }
        for d in dirs {
            for e in walkdir::WalkDir::new(src.join(d))
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                if SKIP.iter().any(|s| e.file_name() == *s) {
                    continue;
                }
                if let Some(rel) = relative(src, e.path()) {
                    out.insert(rel, e.path().to_path_buf());
                }
            }
        }
    }
    Ok(out)
}

/// Ergebnis einer Übernahme.
#[derive(Debug, Default, PartialEq)]
pub struct Copied {
    pub copied: usize,
    /// lag im Ziel schon und blieb unverändert
    pub kept: usize,
    /// ließ sich nicht lesen (z. B. Welt gerade in Minecraft geöffnet)
    pub failed: Vec<String>,
}

impl Copied {
    pub fn json(&self, from: &Path, servers: bool) -> Value {
        json!({ "from": label(from), "copied": self.copied, "kept": self.kept, "failed": self.failed, "servers": servers })
    }
}

/// In einen Spielordner übernehmen – was dort schon liegt, bleibt unangetastet.
/// Einzelne unlesbare Dateien brechen nicht alles ab, sie werden gemeldet.
pub fn copy_missing(files: &BTreeMap<String, PathBuf>, target: &Path) -> Result<Copied> {
    let mut r = Copied::default();
    for (rel, from) in files {
        let to = target.join(rel);
        if to.exists() {
            r.kept += 1;
            continue;
        }
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent)?;
        }
        match fs::copy(from, &to) {
            Ok(_) => r.copied += 1,
            Err(_) => {
                let _ = fs::remove_file(&to);
                r.failed.push(rel.clone());
            }
        }
    }
    Ok(r)
}

/// Bereits gepackte Dateien nicht nochmal komprimieren.
fn stored(rel: &str) -> bool {
    let r = rel.to_ascii_lowercase();
    [".zip", ".jar", ".png", ".jpg", ".ogg", ".mca", ".dat", ".nbt"]
        .iter()
        .any(|x| r.ends_with(x))
}

/// Ins Modrinth-Paket legen (Ordner „overrides“ – die App übernimmt ihn beim Import).
/// Unlesbare Dateien werden übersprungen und gemeldet.
pub fn add_overrides<W: Write + std::io::Seek>(
    z: &mut zip::ZipWriter<W>,
    files: &BTreeMap<String, PathBuf>,
) -> Result<Copied> {
    let o = zip::write::SimpleFileOptions::default();
    let mut r = Copied::default();
    for (rel, from) in files {
        // erst öffnen: Was sich nicht lesen lässt, kommt gar nicht erst ins Paket
        let Ok(mut f) = fs::File::open(from) else {
            r.failed.push(rel.clone());
            continue;
        };
        let big = f.metadata().map(|m| m.len() >= u64::from(u32::MAX)).unwrap_or(false);
        let opts = if stored(rel) {
            o.compression_method(zip::CompressionMethod::Stored)
        } else {
            o
        }
        .large_file(big);
        z.start_file(format!("overrides/{rel}"), opts)?;
        std::io::copy(&mut f, z)?;
        r.copied += 1;
    }
    Ok(r)
}

/// Bezeichnung einer Quelle für Meldungen.
pub fn label(dir: &Path) -> String {
    let c = util::canonical(dir);
    if modrinth_profile_dirs()
        .iter()
        .any(|p| c.starts_with(util::canonical(p)))
    {
        return format!("Modrinth-App: {}", folder_name(dir));
    }
    if client::launcher_dirs().iter().any(|l| util::canonical(l) == c) {
        return "Minecraft-Launcher (Standard)".into();
    }
    folder_name(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("vapafane-transfer-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn put(dir: &Path, rel: &str, data: &str) {
        let p = dir.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, data).unwrap();
    }

    /// Eine Instanz, wie sie die Modrinth-App anlegt (mit Mods und Logs, die nie mitkommen).
    fn instance(dir: &Path) {
        put(dir, "options.txt", "version:4189\nguiScale:3\n");
        put(dir, "config/sodium-options.json", "{}");
        put(dir, "config/iris.properties", "shaderPack=Complementary.zip\n");
        put(dir, "shaderpacks/Complementary.zip", "zip");
        put(dir, "resourcepacks/Faithful.zip", "zip");
        put(dir, "saves/Welt/level.dat", "level");
        put(dir, "saves/Welt/session.lock", "lock");
        put(dir, "mods/sodium.jar", "jar");
        put(dir, "logs/latest.log", "log");
        fs::write(
            dir.join("servers.dat"),
            crate::nbt::servers_with(&[], "Freund", "1.2.3.4"),
        )
        .unwrap();
    }

    #[test]
    fn teile_und_groessen() {
        let d = temp("teile");
        instance(&d);
        let s = part_sizes(&d);
        assert_eq!(
            s.keys().copied().collect::<Vec<_>>(),
            vec!["config", "options", "resourcepacks", "servers", "shaders", "worlds"]
        );
        // Welt ohne session.lock wird trotzdem gezählt; Größe > 0
        assert!(s["worlds"] > 0);
        let leer = temp("leer");
        assert!(part_sizes(&leer).is_empty());
        fs::remove_dir_all(&d).unwrap();
        fs::remove_dir_all(&leer).unwrap();
    }

    #[test]
    fn sammeln_ohne_mods_logs_und_sperrdatei() {
        let d = temp("sammeln");
        instance(&d);
        let all = Take {
            dir: d.clone(),
            parts: PARTS.iter().map(|(k, _)| k.to_string()).collect(),
        };
        let files = gather(&all).unwrap();
        let keys: Vec<&str> = files.keys().map(String::as_str).collect();
        assert_eq!(
            keys,
            vec![
                "config/iris.properties",
                "config/sodium-options.json",
                "options.txt",
                "resourcepacks/Faithful.zip",
                "saves/Welt/level.dat",
                "shaderpacks/Complementary.zip",
            ]
        );
        // Nur Shader: Auswahl aus config kommt mit, sonst nichts aus config
        let shaders = gather(&Take {
            dir: d.clone(),
            parts: vec!["shaders".into()],
        })
        .unwrap();
        assert_eq!(
            shaders.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["config/iris.properties", "shaderpacks/Complementary.zip"]
        );
        assert!(
            gather(&Take {
                dir: d.clone(),
                parts: vec!["mods".into()]
            })
            .is_err()
        );
        assert!(
            gather(&Take {
                dir: d.join("fehlt"),
                parts: vec![]
            })
            .is_err()
        );
        fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn vorhandenes_wird_nicht_ueberschrieben() {
        let src = temp("quelle");
        let dst = temp("ziel");
        instance(&src);
        put(&dst, "options.txt", "eigene");
        let files = gather(&Take {
            dir: src.clone(),
            parts: vec!["options".into(), "config".into()],
        })
        .unwrap();
        let r = copy_missing(&files, &dst).unwrap();
        assert_eq!((r.copied, r.kept, r.failed.len()), (2, 1, 0));
        assert_eq!(fs::read_to_string(dst.join("options.txt")).unwrap(), "eigene");
        assert!(dst.join("config/sodium-options.json").is_file());
        // Quelle unverändert
        assert_eq!(
            fs::read_to_string(src.join("options.txt")).unwrap(),
            "version:4189\nguiScale:3\n"
        );
        fs::remove_dir_all(&src).unwrap();
        fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn unlesbare_datei_bricht_nicht_ab() {
        let src = temp("unlesbar");
        let dst = temp("unlesbar-ziel");
        put(&src, "options.txt", "x");
        let mut files = gather(&Take {
            dir: src.clone(),
            parts: vec!["options".into()],
        })
        .unwrap();
        // verschwindet zwischen Sammeln und Kopieren (wie eine gesperrte Datei)
        files.insert("saves/Welt/region/r.0.0.mca".into(), src.join("fehlt.mca"));
        let r = copy_missing(&files, &dst).unwrap();
        assert_eq!(
            (r.copied, r.failed.clone()),
            (1, vec!["saves/Welt/region/r.0.0.mca".to_string()])
        );
        assert!(!dst.join("saves/Welt/region/r.0.0.mca").exists());
        let mut buf = std::io::Cursor::new(vec![]);
        let mut z = zip::ZipWriter::new(&mut buf);
        let r = add_overrides(&mut z, &files).unwrap();
        assert_eq!((r.copied, r.failed.len()), (1, 1));
        fs::remove_dir_all(&src).unwrap();
        fs::remove_dir_all(&dst).unwrap();
    }

    #[test]
    fn server_liste_der_quelle() {
        let d = temp("server");
        instance(&d);
        let t = Take {
            dir: d.clone(),
            parts: vec!["servers".into()],
        };
        let b = crate::nbt::servers_with(&[&t.servers().unwrap()], "E2E", "5.6.7.8:25599");
        assert_eq!(crate::nbt::addresses(&b), vec!["5.6.7.8:25599", "1.2.3.4"]);
        assert!(
            Take {
                dir: d.clone(),
                parts: vec![]
            }
            .servers()
            .is_none()
        );
        fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn eigene_instanzen_in_der_modrinth_app_erkennen() {
        let p = temp("profiles");
        for n in ["E2E", "E2E (1)", "E2E (zwei)", "E2E2", "Andere"] {
            fs::create_dir_all(p.join(n)).unwrap();
        }
        let found: Vec<String> = modrinth_instances_for("E2E", std::slice::from_ref(&p))
            .iter()
            .map(|d| folder_name(d))
            .collect();
        let mut sorted = found.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["E2E", "E2E (1)"]);
        // Punkt am Ende schneidet die App ab
        fs::create_dir_all(p.join("Server")).unwrap();
        assert_eq!(modrinth_instances_for("Server.", std::slice::from_ref(&p)).len(), 1);
        fs::remove_dir_all(&p).unwrap();
    }

    #[test]
    fn eigener_ordner_als_quelle() {
        let d = temp("eigener");
        assert!(describe(&d).is_err());
        put(&d, "options.txt", "x");
        assert_eq!(describe(&d).unwrap()["parts"]["options"], 1);
        fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn launcher_profile_mit_eigenem_ordner() {
        let l = temp("launcher");
        fs::write(
            l.join("launcher_profiles.json"),
            r#"{"profiles":{"a":{"name":"Mods","gameDir":"/spiel/mods"},"b":{"name":"Standard"},
               "vapafane-x":{"name":"Vapafane – X","gameDir":"/v"}}}"#,
        )
        .unwrap();
        assert_eq!(
            launcher_profile_dirs(&l),
            vec![("Mods".to_string(), PathBuf::from("/spiel/mods"))]
        );
        fs::remove_dir_all(&l).unwrap();
    }

    #[test]
    fn paket_bekommt_overrides() {
        let d = temp("paket");
        instance(&d);
        let files = gather(&Take {
            dir: d.clone(),
            parts: vec!["options".into(), "resourcepacks".into()],
        })
        .unwrap();
        let mut buf = std::io::Cursor::new(vec![]);
        {
            let mut z = zip::ZipWriter::new(&mut buf);
            assert_eq!(add_overrides(&mut z, &files).unwrap().copied, 2);
            z.finish().unwrap();
        }
        let mut a = zip::ZipArchive::new(std::io::Cursor::new(buf.into_inner())).unwrap();
        let mut names: Vec<String> = (0..a.len())
            .map(|i| a.by_index(i).unwrap().name().to_string())
            .collect();
        names.sort();
        assert_eq!(
            names,
            vec!["overrides/options.txt", "overrides/resourcepacks/Faithful.zip"]
        );
        let mut s = String::new();
        std::io::Read::read_to_string(&mut a.by_name("overrides/options.txt").unwrap(), &mut s).unwrap();
        assert_eq!(s, "version:4189\nguiScale:3\n");
        fs::remove_dir_all(&d).unwrap();
    }
}
