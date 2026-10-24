//! Aufräumen: Reste alter Versionen aus dem Server-Ordner entfernen.
//!
//! Gelöscht wird nur, was zur alten Version gehört oder von Minecraft/Fabric beim
//! nächsten Start ohnehin neu erzeugt bzw. geladen wird. Welt, Einstellungen und
//! ausgeschaltete Mods (`.disabled`) bleiben immer erhalten.

use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::ALT_DIR_NAME;
use crate::server;
use crate::util::{self, cmp_versions};

#[derive(Clone, Debug, Serialize)]
pub struct Candidate {
    pub rel: String,
    pub what: String,
    pub size: u64,
    #[serde(skip)]
    pub path: PathBuf,
}

fn size_of(p: &Path) -> u64 {
    if p.is_dir() {
        util::dir_size(p)
    } else {
        fs::metadata(p).map(|m| m.len()).unwrap_or(0)
    }
}

fn cand(dir: &Path, path: PathBuf, what: &str) -> Candidate {
    Candidate {
        rel: path
            .strip_prefix(dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/"),
        what: what.into(),
        size: size_of(&path),
        path,
    }
}

fn subdirs(p: &Path) -> Vec<PathBuf> {
    fs::read_dir(p)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect()
        })
        .unwrap_or_default()
}

/// Bibliotheken im Maven-Aufbau (`gruppe/artefakt/version/*.jar`): von jedem Artefakt
/// nur die neueste Version behalten. Fehlt doch etwas, lädt der Server es beim Start nach.
fn old_libraries(dir: &Path, mc_version: &str, loader: &str) -> Vec<Candidate> {
    let libs = dir.join("libraries");
    let mut by_artifact: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
    for e in walkdir::WalkDir::new(&libs)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let has_jar = fs::read_dir(e.path())
            .map(|rd| {
                rd.filter_map(|x| x.ok())
                    .any(|x| x.file_name().to_string_lossy().ends_with(".jar"))
            })
            .unwrap_or(false);
        if has_jar && let Some(parent) = e.path().parent() {
            by_artifact
                .entry(parent.to_path_buf())
                .or_default()
                .push(e.path().to_path_buf());
        }
    }
    let mut out = vec![];
    for (artifact, mut versions) in by_artifact {
        let name = |p: &PathBuf| {
            p.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default()
        };
        // Fabric-Zwischen-Mappings gehören genau zu einer Minecraft-Version
        if artifact.ends_with("net/fabricmc/intermediary") && !mc_version.is_empty() {
            for v in versions.iter().filter(|v| name(v) != mc_version) {
                out.push(cand(dir, v.clone(), "Fabric-Mappings einer alten Minecraft-Version"));
            }
            continue;
        }
        // NeoForge: genau die installierte Version behalten
        if artifact.ends_with("net/neoforged/neoforge") && versions.iter().any(|v| name(v) == loader) {
            for v in versions.iter().filter(|v| name(v) != loader) {
                out.push(cand(dir, v.clone(), &crate::sprache::t!("cln-alte-neoforge")));
            }
            continue;
        }
        if versions.len() < 2 {
            continue;
        }
        versions.sort_by(|a, b| cmp_versions(&name(a), &name(b)));
        versions.pop(); // neueste behalten
        for v in versions {
            out.push(cand(dir, v, &crate::sprache::t!("cln-alte-bibliothek")));
        }
    }
    out
}

/// Reste einer alten Minecraft-Version, die nach einem geglückten Start weg können.
fn old_version_files(dir: &Path, version: &str, loader: &str) -> Vec<Candidate> {
    let mut out = vec![];
    if version.is_empty() {
        return out;
    }
    for v in subdirs(&dir.join("versions")) {
        if v.file_name().map(|n| n.to_string_lossy() != version).unwrap_or(false) {
            out.push(cand(dir, v, &crate::sprache::t!("cln-alte-mc-dateien")));
        }
    }
    let fabric = dir.join(".fabric");
    for v in subdirs(&fabric.join("remappedJars")) {
        let n = v.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let current =
            n == format!("minecraft-{version}-{loader}") || (loader.is_empty() && n.contains(&format!("-{version}-")));
        if !current {
            out.push(cand(dir, v, "Fabric-Zwischenspeicher einer alten Version"));
        }
    }
    if let Ok(rd) = fs::read_dir(fabric.join("server")) {
        for e in rd.filter_map(|e| e.ok()) {
            let n = e.file_name().to_string_lossy().into_owned();
            let current = n == format!("{version}-server.jar")
                || (n.ends_with(&format!("-minecraft-{version}.jar")) && (loader.is_empty() || n.contains(loader)));
            if !current {
                out.push(cand(dir, e.path(), &crate::sprache::t!("cln-alte-server-datei")));
            }
        }
    }
    // Paper: von Mojang geladene Original-Jars alter Versionen
    if let Ok(rd) = fs::read_dir(dir.join("cache")) {
        for e in rd.filter_map(|e| e.ok()) {
            let n = e.file_name().to_string_lossy().into_owned();
            if n.starts_with("mojang_") && n.ends_with(".jar") && n != format!("mojang_{version}.jar") {
                out.push(cand(dir, e.path(), &crate::sprache::t!("cln-alte-mc-datei")));
            }
        }
    }
    out.extend(old_libraries(dir, version, loader));
    out
}

/// Liegen im Mod-Zwischenspeicher mehrere Versionen derselben Mod? (Name bis „-<Ziffer>“)
fn has_stale_processed(p: &Path) -> bool {
    let Ok(rd) = fs::read_dir(p) else {
        return false;
    };
    let mut seen = std::collections::HashSet::new();
    for e in rd.filter_map(|e| e.ok()) {
        let n = e.file_name().to_string_lossy().into_owned();
        let b = n.as_bytes();
        let cut = (1..b.len())
            .find(|&i| b[i - 1] == b'-' && b[i].is_ascii_digit())
            .unwrap_or(b.len());
        if !seen.insert(n[..cut].to_string()) {
            return true;
        }
    }
    false
}

/// Alles, was der Knopf „Aufräumen“ löschen würde (Server muss dafür gestoppt sein).
pub fn candidates(dir: &Path) -> Vec<Candidate> {
    let info = server::detect_server(dir);
    let mut out = vec![];
    let old = dir.join("mods").join(".old");
    if old.is_dir() {
        out.push(cand(dir, old, &crate::sprache::t!("cln-alte-mods")));
    }
    for v in subdirs(&dir.join(ALT_DIR_NAME)) {
        if v.file_name()
            .map(|n| n.to_string_lossy().starts_with("vor_Update_"))
            .unwrap_or(false)
        {
            out.push(cand(dir, v, &crate::sprache::t!("cln-vor-update")));
        }
    }
    let processed = dir.join(".fabric").join("processedMods");
    if has_stale_processed(&processed) {
        out.push(cand(dir, processed, &crate::sprache::t!("cln-cache")));
    }
    if matches!(
        info.kind.as_str(),
        "fabric" | "vanilla" | "quilt" | "paper" | "neoforge"
    ) {
        out.extend(old_version_files(dir, &info.version, &info.loader));
    }
    out
}

/// Nach einem geglückten Update: Altes aus diesem Update und Reste der alten Version.
pub fn after_update(dir: &Path, alt: &Path, version: &str, loader: &str) -> Vec<Candidate> {
    let mut out = vec![];
    if alt.exists() {
        out.push(cand(dir, alt.to_path_buf(), &crate::sprache::t!("cln-ersetzt")));
    }
    out.extend(old_version_files(dir, version, loader));
    out
}

/// Löscht die Einträge. Was nicht geht (z. B. unter Windows gerade benutzt), bleibt liegen.
pub fn delete(dir: &Path, items: &[Candidate]) -> (u64, Vec<String>) {
    let mut freed = 0;
    let mut failed = vec![];
    for c in items {
        // Nur innerhalb des Server-Ordners löschen
        if !util::canonical(&c.path).starts_with(util::canonical(dir)) || c.path == dir {
            failed.push(c.rel.clone());
            continue;
        }
        let res = if c.path.is_dir() {
            fs::remove_dir_all(&c.path)
        } else {
            fs::remove_file(&c.path)
        };
        match res {
            Ok(()) => freed += c.size,
            Err(_) => failed.push(c.rel.clone()),
        }
    }
    // Leeren .panel-alt-Ordner mit wegräumen
    let alt = dir.join(ALT_DIR_NAME);
    if fs::read_dir(&alt).map(|mut rd| rd.next().is_none()).unwrap_or(false) {
        let _ = fs::remove_dir(&alt);
    }
    (freed, failed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn version_order() {
        assert_eq!(
            cmp_versions("0.17.4+mixin.0.8.7", "0.16.5+mixin.0.8.7"),
            Ordering::Greater
        );
        assert_eq!(cmp_versions("2.20.0", "2.17.0"), Ordering::Greater);
        assert_eq!(cmp_versions("1.21.11", "26.3"), Ordering::Less);
        assert_eq!(cmp_versions("0.19.5", "0.19.5"), Ordering::Equal);
    }

    #[test]
    fn finds_leftovers() {
        let dir = std::env::temp_dir().join(format!("vapafane-clean-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        for d in [
            "mods/.old",
            "versions/1.21.11",
            "versions/26.3",
            "libraries/commons-io/commons-io/2.17.0",
            "libraries/commons-io/commons-io/2.20.0",
            "libraries/net/fabricmc/intermediary/1.21.11",
            ".fabric/remappedJars/minecraft-1.21.11-0.18.4",
            ".fabric/server",
            ".panel-alt/vor_Update_2026-10-17_12-22",
            "world",
        ] {
            fs::create_dir_all(dir.join(d)).unwrap();
        }
        for f in [
            "mods/.old/a.jar",
            "libraries/commons-io/commons-io/2.17.0/c.jar",
            "libraries/commons-io/commons-io/2.20.0/c.jar",
            "libraries/net/fabricmc/intermediary/1.21.11/i.jar",
            ".fabric/server/26.3-server.jar",
            ".fabric/server/1.21.11-server.jar",
            "world/level.dat",
            "fabric-server-launch.jar",
        ] {
            fs::write(dir.join(f), "x").unwrap();
        }
        fs::write(
            dir.join(".mcpanel.env"),
            "MC_VERSION=26.3\nMC_JAR=fabric-server-launch.jar\n",
        )
        .unwrap();
        let rels: Vec<String> = candidates(&dir).into_iter().map(|c| c.rel).collect();
        for want in [
            "mods/.old",
            ".panel-alt/vor_Update_2026-10-17_12-22",
            "versions/1.21.11",
            "libraries/commons-io/commons-io/2.17.0",
            "libraries/net/fabricmc/intermediary/1.21.11",
            ".fabric/remappedJars/minecraft-1.21.11-0.18.4",
            ".fabric/server/1.21.11-server.jar",
        ] {
            assert!(rels.contains(&want.to_string()), "{want} fehlt in {rels:?}");
        }
        for keep in [
            "versions/26.3",
            "libraries/commons-io/commons-io/2.20.0",
            ".fabric/server/26.3-server.jar",
            "world",
        ] {
            assert!(!rels.iter().any(|r| r == keep), "{keep} darf nicht weg");
        }
        let (freed, failed) = delete(&dir, &candidates(&dir));
        assert!(freed > 0 && failed.is_empty());
        assert!(dir.join("world/level.dat").exists() && !dir.join(".panel-alt").exists());
        fs::remove_dir_all(&dir).unwrap();
    }
}
