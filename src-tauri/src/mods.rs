//! Mod- und Datapack-Liste mit An/Aus-Schalter.
//!
//! Ausschalten heißt umbenennen in `<name>.jar.disabled` bzw. `<name>.zip.disabled`
//! (wie bei der Modrinth-App und Prism Launcher) – Fabric und Minecraft laden diese
//! Dateien nicht. Warum etwas aus ist (z. B. „gibt es nicht für 26.3“), steht in
//! `.redcraftserver.json` im Server-Ordner und wandert so auch mit ins Backup.

use serde_json::{Map, Value, json};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::server;
use crate::util::{self, Result, err};

pub const DISABLED: &str = ".disabled";
const META_FILE: &str = ".redcraftserver.json";

// ---------------------------------------------------------------- Gründe merken

fn read_meta(dir: &Path) -> Map<String, Value> {
    fs::read_to_string(dir.join(META_FILE))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
}

fn write_meta(dir: &Path, meta: &Map<String, Value>) -> Result<()> {
    util::write_atomic(&dir.join(META_FILE), serde_json::to_string_pretty(meta)?.as_bytes())
}

/// Grund zu einer Datei (Schlüssel: Pfad relativ zum Server, ohne `.disabled`).
#[cfg(test)]
pub fn note(dir: &Path, rel: &str) -> Option<String> {
    read_meta(dir)
        .get("mod_notes")
        .and_then(|n| n.get(rel))
        .and_then(|r| r.as_str())
        .map(str::to_string)
}

pub fn set_note(dir: &Path, rel: &str, reason: Option<&str>) -> Result<()> {
    let mut meta = read_meta(dir);
    let notes = meta.entry("mod_notes").or_insert_with(|| json!({}));
    if !notes.is_object() {
        *notes = json!({});
    }
    let notes = notes.as_object_mut().unwrap();
    match reason {
        Some(r) => {
            notes.insert(rel.to_string(), json!(r));
        }
        None => {
            notes.remove(rel);
        }
    }
    write_meta(dir, &meta)
}

/// Datei ausschalten (umbenennen) und den Grund merken.
pub fn disable(dir: &Path, rel: &str, reason: &str) -> Result<()> {
    let src = dir.join(rel);
    let dest = PathBuf::from(format!("{}{DISABLED}", src.display()));
    if dest.exists() {
        fs::remove_file(&dest)?; // ältere ausgeschaltete Kopie derselben Datei
    }
    fs::rename(&src, &dest)?;
    set_note(dir, rel, Some(reason))
}

// ---------------------------------------------------------------- Liste

/// Name und Version aus fabric.mod.json in der Jar.
fn mod_info(jar: &Path) -> (Option<String>, Option<String>, Option<String>) {
    let read = || -> Option<Value> {
        let mut z = zip::ZipArchive::new(fs::File::open(jar).ok()?).ok()?;
        let mut e = z.by_name("fabric.mod.json").ok()?;
        let mut s = String::new();
        e.read_to_string(&mut s).ok()?;
        // Manche Mods haben Steuerzeichen in Beschreibungen – grob säubern
        let s: String = s
            .chars()
            .map(|c| if c.is_control() && c != '\n' { ' ' } else { c })
            .collect();
        serde_json::from_str(&s).ok()
    };
    match read() {
        Some(v) => (
            v.get("name").and_then(|n| n.as_str()).map(str::to_string),
            v.get("version").and_then(|n| n.as_str()).map(str::to_string),
            v.get("environment").and_then(|n| n.as_str()).map(str::to_string),
        ),
        None => (None, None, None),
    }
}

fn level_name(dir: &Path) -> String {
    server::read_properties(dir)
        .get("level-name")
        .cloned()
        .filter(|l| !l.is_empty())
        .unwrap_or_else(|| "world".into())
}

/// Alle Mods und Datapacks, ein- und ausgeschaltet.
pub fn list(dir: &Path) -> Value {
    let notes = read_meta(dir).get("mod_notes").cloned().unwrap_or(json!({}));
    let mut mods = vec![];
    if let Ok(rd) = fs::read_dir(dir.join("mods")) {
        for e in rd.filter_map(|e| e.ok()).filter(|e| e.path().is_file()) {
            let name = e.file_name().to_string_lossy().into_owned();
            let (base, enabled) = match name.strip_suffix(DISABLED) {
                Some(b) if b.ends_with(".jar") => (b.to_string(), false),
                _ if name.ends_with(".jar") => (name.clone(), true),
                _ => continue,
            };
            let rel = format!("mods/{base}");
            let (title, version, env) = mod_info(&e.path());
            mods.push(json!({
                "kind": "mod",
                "file": base,
                "rel": rel,
                "enabled": enabled,
                "title": title.unwrap_or_else(|| base.trim_end_matches(".jar").to_string()),
                "version": version,
                "client_only": env.as_deref() == Some("client"),
                "reason": notes.get(&rel),
                "size": e.metadata().map(|m| m.len()).unwrap_or(0),
            }));
        }
    }
    let level = level_name(dir);
    let mut packs = vec![];
    if let Ok(rd) = fs::read_dir(dir.join(&level).join("datapacks")) {
        for e in rd.filter_map(|e| e.ok()) {
            let name = e.file_name().to_string_lossy().into_owned();
            let is_dir = e.path().is_dir();
            let (base, enabled) = match name.strip_suffix(DISABLED) {
                Some(b) if b.to_lowercase().ends_with(".zip") => (b.to_string(), false),
                _ if is_dir || name.to_lowercase().ends_with(".zip") => (name.clone(), true),
                _ => continue,
            };
            let rel = format!("{level}/datapacks/{base}");
            packs.push(json!({
                "kind": "datapack",
                "file": base,
                "rel": rel,
                "enabled": enabled,
                "folder": is_dir,
                "title": base.trim_end_matches(".zip").to_string(),
                "reason": notes.get(&rel),
            }));
        }
    }
    let key = |v: &Value| v["title"].as_str().unwrap_or("").to_lowercase();
    mods.sort_by_key(key);
    packs.sort_by_key(key);
    json!({"mods": mods, "datapacks": packs})
}

/// Schalter umlegen. `rel` ist der Pfad ohne `.disabled`.
pub fn set_enabled(dir: &Path, rel: &str, on: bool) -> Result<()> {
    let ok_prefix = rel.starts_with("mods/") || rel.contains("/datapacks/");
    if !ok_prefix || rel.contains("..") || rel.contains('\\') {
        return err("Ungültiger Pfad");
    }
    let enabled = dir.join(rel);
    let disabled = PathBuf::from(format!("{}{DISABLED}", enabled.display()));
    if on {
        if enabled.exists() {
            return err("Es gibt schon eine eingeschaltete Datei mit diesem Namen.");
        }
        fs::rename(&disabled, &enabled)?;
        set_note(dir, rel, None)?;
    } else {
        if enabled.is_dir() {
            return err("Datapack-Ordner lassen sich nicht ausschalten – nur ZIP-Datapacks.");
        }
        fs::rename(&enabled, &disabled)?;
        set_note(dir, rel, Some("von Hand ausgeschaltet"))?;
    }
    Ok(())
}

/// Ausgeschaltete Mod-Jars (für den Update-Assistenten).
pub fn disabled_mods(dir: &Path) -> Vec<(String, PathBuf)> {
    let Ok(rd) = fs::read_dir(dir.join("mods")) else {
        return vec![];
    };
    rd.filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let base = name.strip_suffix(DISABLED)?;
            base.ends_with(".jar").then(|| (base.to_string(), e.path()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_and_reason() {
        let dir = std::env::temp_dir().join(format!("redcraft-mods-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("mods")).unwrap();
        fs::write(dir.join("mods/a.jar"), "x").unwrap();
        disable(&dir, "mods/a.jar", "gibt es nicht für 26.3").unwrap();
        assert!(dir.join("mods/a.jar.disabled").exists());
        let l = list(&dir);
        assert_eq!(l["mods"][0]["enabled"], false);
        assert_eq!(l["mods"][0]["reason"], "gibt es nicht für 26.3");
        set_enabled(&dir, "mods/a.jar", true).unwrap();
        assert!(dir.join("mods/a.jar").exists());
        assert!(note(&dir, "mods/a.jar").is_none());
        assert!(set_enabled(&dir, "../x.jar", false).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }
}
