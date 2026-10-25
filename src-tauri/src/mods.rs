//! Mod- und Datapack-Liste: ein-/ausschalten, entfernen, Abhängigkeiten und Zusatz-Infos.
//!
//! Ausschalten heißt umbenennen in `<name>.jar.disabled` bzw. `<name>.zip.disabled`
//! (wie bei der Modrinth-App und Prism Launcher) – Fabric und Minecraft laden diese
//! Dateien nicht. Warum etwas aus ist (z. B. „gibt es nicht für 26.3“), steht in
//! `.vapafane.json` im Server-Ordner und wandert so auch mit ins Backup – als
//! Sprach-Schlüssel, damit der Grund nach einem Sprachwechsel mitwechselt. Dort liegen
//! auch die nachgeschlagenen Infos (Symbol, Kurzbeschreibung, Link zur Mod-Seite).

use serde_json::{Map, Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Cursor, Read, Seek};
use std::path::{Path, PathBuf};

use crate::server;
use crate::util::{self, Result, err};
use crate::{curseforge, net, updater};

pub const DISABLED: &str = ".disabled";
const META_FILE: &str = ".vapafane.json";
/// Früherer Dateiname – wird beim Start einmalig übernommen (siehe convert_old_meta).
const OLD_META_FILE: &str = ".redcraftserver.json";
const MODRINTH_API: &str = "https://api.modrinth.com/v2";

// ---------------------------------------------------------------- Notizen in .vapafane.json

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

/// Einmalig beim Start: Mod-Notizen unter dem früheren Dateinamen in die neue Datei
/// übernehmen (vorhandene Einträge der neuen Datei haben Vorrang) und die alte löschen.
pub fn convert_old_meta(dir: &Path) {
    let old = dir.join(OLD_META_FILE);
    let Some(old_meta) = fs::read_to_string(&old)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_object().cloned())
    else {
        return;
    };
    let mut meta = read_meta(dir);
    for (section, entries) in old_meta {
        match (
            meta.get_mut(&section).and_then(|v| v.as_object_mut()),
            entries.as_object(),
        ) {
            (Some(target), Some(src)) => {
                for (k, v) in src {
                    target.entry(k.clone()).or_insert_with(|| v.clone());
                }
            }
            _ => {
                meta.entry(section).or_insert(entries);
            }
        }
    }
    if write_meta(dir, &meta).is_ok() {
        let _ = fs::remove_file(old);
    }
}

/// Für alle Server aus der Konfiguration.
pub fn convert_old_meta_all() {
    for s in crate::config::Config::load().servers {
        convert_old_meta(Path::new(&s.path));
    }
}

/// Ein Abschnitt aus `.vapafane.json` („mod_notes“, „sources“, „infos“).
fn section(dir: &Path, name: &str) -> Map<String, Value> {
    read_meta(dir)
        .get(name)
        .and_then(|s| s.as_object())
        .cloned()
        .unwrap_or_default()
}

/// Eintrag eines Abschnitts setzen oder (None) löschen.
fn set_entry(dir: &Path, name: &str, rel: &str, value: Option<Value>) -> Result<()> {
    let mut meta = read_meta(dir);
    let all = meta.entry(name).or_insert_with(|| json!({}));
    if !all.is_object() {
        *all = json!({});
    }
    let Some(all) = all.as_object_mut() else {
        return write_meta(dir, &meta);
    };
    match value {
        Some(v) => {
            all.insert(rel.to_string(), v);
        }
        None => {
            all.remove(rel);
        }
    }
    write_meta(dir, &meta)
}

/// Grund als Sprach-Schlüssel mit Werten merken – übersetzt wird erst beim Anzeigen.
pub fn grund(key: &str, args: &[(&str, &str)]) -> Value {
    let mut m = Map::new();
    m.insert("k".into(), json!(key));
    for (name, wert) in args {
        m.insert((*name).to_string(), json!(wert));
    }
    Value::Object(m)
}

/// Gemerkter Grund in der aktuellen Sprache (Einträge älterer Versionen sind fertiger Text).
fn grund_text(v: &Value) -> Option<String> {
    if let Some(s) = v.as_str() {
        return Some(s.to_string());
    }
    let o = v.as_object()?;
    let key = o.get("k")?.as_str()?;
    let mut args = ::vapafane_shared::sprache::FluentArgs::new();
    for (name, wert) in o.iter().filter(|(n, _)| n.as_str() != "k") {
        if let Some(s) = wert.as_str() {
            args.set(name.clone(), ::vapafane_shared::sprache::FluentValue::from(s.to_string()));
        }
    }
    Some(crate::sprache::text(key, Some(&args)))
}

/// Grund zu einer Datei (Schlüssel: Pfad relativ zum Server, ohne `.disabled`).
#[cfg(test)]
pub fn note(dir: &Path, rel: &str) -> Option<String> {
    section(dir, "mod_notes").get(rel).and_then(grund_text)
}

pub fn set_note(dir: &Path, rel: &str, reason: Option<Value>) -> Result<()> {
    set_entry(dir, "mod_notes", rel, reason)
}

/// Herkunft einer Datei merken: {"source": "modrinth"|"curseforge", "id": …, "dep": bool}.
/// `dep` heißt: kam als Abhängigkeit eines anderen Mods mit.
pub fn set_source(dir: &Path, rel: &str, source: Option<Value>) -> Result<()> {
    set_entry(dir, "sources", rel, source)
}

/// Alle gemerkten Herkünfte (Pfad → {"source", "id"}).
pub fn sources(dir: &Path) -> Map<String, Value> {
    section(dir, "sources")
}

/// Datei ausschalten (umbenennen) und den Grund merken (siehe `grund`).
pub fn disable(dir: &Path, rel: &str, reason: Value) -> Result<()> {
    let src = dir.join(rel);
    let dest = PathBuf::from(format!("{}{DISABLED}", src.display()));
    if dest.exists() {
        fs::remove_file(&dest)?; // ältere ausgeschaltete Kopie derselben Datei
    }
    fs::rename(&src, &dest)?;
    set_note(dir, rel, Some(reason))
}

// ---------------------------------------------------------------- Infos aus der Jar

fn read_entry<R: Read + Seek>(z: &mut zip::ZipArchive<R>, name: &str) -> Option<String> {
    let mut e = z.by_name(name).ok()?;
    let mut s = String::new();
    e.read_to_string(&mut s).ok()?;
    Some(s)
}

/// Wert aus `key: wert` (plugin.yml) oder `key="wert"` (mods.toml).
fn simple_value(text: &str, key: &str, sep: char) -> Option<String> {
    text.lines().find_map(|l| {
        let l = l.trim();
        let (k, v) = l.split_once(sep)?;
        (k.trim() == key)
            .then(|| v.trim().trim_matches(['"', '\'']).to_string())
            .filter(|v| !v.is_empty())
    })
}

/// fabric.mod.json lesen (manche Mods haben Steuerzeichen in Beschreibungen – grob säubern).
fn fabric_json<R: Read + Seek>(z: &mut zip::ZipArchive<R>) -> Option<Value> {
    let s: String = read_entry(z, "fabric.mod.json")?
        .chars()
        .map(|c| if c.is_control() && c != '\n' { ' ' } else { c })
        .collect();
    serde_json::from_str(&s).ok()
}

#[derive(Default)]
struct JarInfo {
    title: Option<String>,
    version: Option<String>,
    /// „client“, wenn die Mod nur auf den Client gehört
    env: Option<String>,
    desc: Option<String>,
}

/// Name, Version, „nur Client?“ und Beschreibung aus der Jar (Fabric, NeoForge, Paper/Bukkit).
fn mod_info(jar: &Path) -> JarInfo {
    let Some(mut z) = fs::File::open(jar).ok().and_then(|f| zip::ZipArchive::new(f).ok()) else {
        return JarInfo::default();
    };
    let s = |v: &Value, k: &str| v.get(k).and_then(|n| n.as_str()).map(str::to_string);
    if let Some(v) = fabric_json(&mut z) {
        return JarInfo {
            title: s(&v, "name"),
            version: s(&v, "version"),
            env: s(&v, "environment"),
            desc: s(&v, "description"),
        };
    }
    for f in ["paper-plugin.yml", "plugin.yml"] {
        if let Some(t) = read_entry(&mut z, f) {
            return JarInfo {
                title: simple_value(&t, "name", ':'),
                version: simple_value(&t, "version", ':'),
                env: None,
                desc: simple_value(&t, "description", ':'),
            };
        }
    }
    if let Some(t) = read_entry(&mut z, "META-INF/neoforge.mods.toml") {
        let mut version = simple_value(&t, "version", '=');
        if version.as_deref().map(|v| v.contains("${")).unwrap_or(true) {
            version = read_entry(&mut z, "META-INF/MANIFEST.MF")
                .and_then(|m| simple_value(&m, "Implementation-Version", ':'));
        }
        return JarInfo {
            title: simple_value(&t, "displayName", '='),
            version,
            env: None,
            // mehrzeilige Beschreibungen (''') lässt die einfache Lesart aus
            desc: simple_value(&t, "description", '=').filter(|d| !d.starts_with("''")),
        };
    }
    JarInfo::default()
}

fn level_name(dir: &Path) -> String {
    server::read_properties(dir)
        .get("level-name")
        .cloned()
        .filter(|l| !l.is_empty())
        .unwrap_or_else(|| "world".into())
}

// ---------------------------------------------------------------- Liste

/// Alle Mods und Datapacks, ein- und ausgeschaltet.
pub fn list(dir: &Path) -> Value {
    let notes = section(dir, "mod_notes");
    let srcs = sources(dir);
    let reason_of = |rel: &str| notes.get(rel).and_then(grund_text);
    let source_of = |rel: &str| srcs.get(rel).and_then(|v| v.get("source")).cloned();
    let sub = crate::flavor::mod_dir_of(dir);
    let mut mods = vec![];
    if let Ok(rd) = fs::read_dir(dir.join(sub)) {
        for e in rd.filter_map(|e| e.ok()).filter(|e| e.path().is_file()) {
            let name = e.file_name().to_string_lossy().into_owned();
            let (base, enabled) = match name.strip_suffix(DISABLED) {
                Some(b) if b.ends_with(".jar") => (b.to_string(), false),
                _ if name.ends_with(".jar") => (name.clone(), true),
                _ => continue,
            };
            let rel = format!("{sub}/{base}");
            let info = mod_info(&e.path());
            mods.push(json!({
                "kind": "mod",
                "file": base,
                "rel": rel,
                "enabled": enabled,
                "title": info.title.unwrap_or_else(|| base.trim_end_matches(".jar").to_string()),
                "version": info.version,
                "desc": info.desc,
                "client_only": info.env.as_deref() == Some("client"),
                "reason": reason_of(&rel),
                "source": source_of(&rel),
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
                "reason": reason_of(&rel),
                "source": source_of(&rel),
            }));
        }
    }
    let key = |v: &Value| v["title"].as_str().unwrap_or("").to_lowercase();
    mods.sort_by_key(key);
    packs.sort_by_key(key);
    json!({"mods": mods, "datapacks": packs, "mod_dir": sub})
}

/// Pfad aus der Liste prüfen: genau eine Datei bzw. ein Datapack direkt in `mods`,
/// `plugins` oder `<welt>/datapacks` – nichts anderes lässt sich schalten oder löschen.
fn pruefe_rel(dir: &Path, rel: &str) -> Result<PathBuf> {
    let p = dir.join(rel);
    let erlaubt = [
        dir.join("mods"),
        dir.join("plugins"),
        dir.join(level_name(dir)).join("datapacks"),
    ];
    let ok = !rel.is_empty()
        && !rel.contains("..")
        && !rel.contains('\\')
        && !rel.ends_with('/')
        && p.file_name().is_some()
        && p.parent().is_some_and(|eltern| erlaubt.iter().any(|e| e == eltern));
    if !ok {
        return err(crate::sprache::t!("err-pfad"));
    }
    Ok(p)
}

/// Schalter umlegen. `rel` ist der Pfad ohne `.disabled`.
pub fn set_enabled(dir: &Path, rel: &str, on: bool) -> Result<()> {
    let enabled = pruefe_rel(dir, rel)?;
    let disabled = PathBuf::from(format!("{}{DISABLED}", enabled.display()));
    if on {
        if enabled.exists() {
            return err(crate::sprache::t!("mods-err-name-belegt"));
        }
        fs::rename(&disabled, &enabled)?;
        set_note(dir, rel, None)?;
    } else {
        if enabled.is_dir() {
            return err(crate::sprache::t!("mods-err-ordner-aus"));
        }
        fs::rename(&enabled, &disabled)?;
        set_note(dir, rel, Some(grund("mods-grund-hand", &[])))?;
    }
    Ok(())
}

/// Datei bzw. Datapack-Ordner endgültig löschen – egal ob ein- oder ausgeschaltet –
/// und alles vergessen, was zu ihr gemerkt war.
pub fn remove(dir: &Path, rel: &str) -> Result<()> {
    let p = pruefe_rel(dir, rel)?;
    let aus = PathBuf::from(format!("{}{DISABLED}", p.display()));
    if p.is_dir() {
        fs::remove_dir_all(&p)?;
    } else if p.is_file() {
        fs::remove_file(&p)?;
    } else if aus.is_file() {
        fs::remove_file(&aus)?;
    } else {
        return err(crate::sprache::t!("mods-err-weg"));
    }
    let mut meta = read_meta(dir);
    for teil in ["mod_notes", "sources", "infos"] {
        if let Some(t) = meta.get_mut(teil).and_then(|t| t.as_object_mut()) {
            t.remove(rel);
        }
    }
    write_meta(dir, &meta)
}

/// Ausgeschaltete Jars in `sub` ("mods" oder "plugins") für den Update-Assistenten.
pub fn disabled_in(dir: &Path, sub: &str) -> Vec<(String, PathBuf)> {
    let Ok(rd) = fs::read_dir(dir.join(sub)) else {
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

// ---------------------------------------------------------------- Abhängigkeiten

/// Was eine Mod-Datei bereitstellt (Mod-IDs bzw. Plugin-Namen) und was sie zwingend braucht.
#[derive(Default, Debug)]
struct Bedarf {
    stellt: Vec<String>,
    braucht: Vec<String>,
}

/// `key = "wert"` aus einer TOML-Zeile.
fn toml_wert(zeile: &str, key: &str) -> Option<String> {
    let (k, v) = zeile.split_once('=')?;
    (k.trim() == key).then(|| v.trim().trim_matches(['"', '\'']).to_string())
}

/// neoforge.mods.toml: `[[mods]] modId` stellt bereit, `[[dependencies.x]]` mit
/// `type="required"` (bzw. früher `mandatory=true`) wird gebraucht.
fn toml_bedarf(text: &str) -> Bedarf {
    let mut b = Bedarf::default();
    let (mut in_mods, mut in_dep) = (false, false);
    let (mut dep_id, mut pflicht) = (None::<String>, false);
    let abschliessen = |b: &mut Bedarf, id: &mut Option<String>, pflicht: bool| {
        if let Some(id) = id.take().filter(|_| pflicht) {
            b.braucht.push(id.to_lowercase());
        }
    };
    for zeile in text.lines().map(str::trim) {
        if zeile.starts_with('[') {
            if in_dep {
                abschliessen(&mut b, &mut dep_id, pflicht);
            }
            in_mods = zeile == "[[mods]]";
            in_dep = zeile.starts_with("[[dependencies");
            pflicht = false;
            continue;
        }
        if in_mods && let Some(id) = toml_wert(zeile, "modId") {
            b.stellt.push(id.to_lowercase());
        }
        if in_dep {
            if let Some(id) = toml_wert(zeile, "modId") {
                dep_id = Some(id);
            }
            if toml_wert(zeile, "type").is_some_and(|t| t == "required")
                || toml_wert(zeile, "mandatory").is_some_and(|t| t == "true")
            {
                pflicht = true;
            }
        }
    }
    if in_dep {
        abschliessen(&mut b, &mut dep_id, pflicht);
    }
    b
}

/// Einrückung einer Zeile (Leerzeichen am Anfang).
fn einrueckung(zeile: &str) -> usize {
    zeile.len() - zeile.trim_start().len()
}

/// YAML-Liste unter `key:` – als `[a, b]` in derselben Zeile oder als `- a`-Zeilen darunter.
fn yml_liste(text: &str, key: &str) -> Vec<String> {
    let zeilen: Vec<&str> = text.lines().collect();
    let Some(i) = zeilen.iter().position(|z| {
        einrueckung(z) == 0 && z.split_once(':').is_some_and(|(k, _)| k.trim() == key)
    }) else {
        return vec![];
    };
    let rest = zeilen[i].split_once(':').map(|(_, r)| r.trim()).unwrap_or("");
    let sauber = |s: &str| s.trim().trim_matches(['"', '\'']).to_string();
    if let Some(innen) = rest.strip_prefix('[') {
        return innen
            .trim_end_matches(']')
            .split(',')
            .map(sauber)
            .filter(|s| !s.is_empty())
            .collect();
    }
    zeilen[i + 1..]
        .iter()
        .take_while(|z| z.trim().is_empty() || einrueckung(z) > 0 || z.trim_start().starts_with('-'))
        .filter_map(|z| z.trim().strip_prefix('-').map(sauber))
        .filter(|s| !s.is_empty())
        .collect()
}

/// paper-plugin.yml: Plugins unter `dependencies: server:` – Pflicht, solange nicht
/// `required: false` dabeisteht.
fn paper_pflicht(text: &str) -> Vec<String> {
    let zeilen: Vec<&str> = text.lines().filter(|z| !z.trim().is_empty()).collect();
    let Some(d) = zeilen.iter().position(|z| einrueckung(z) == 0 && z.trim() == "dependencies:") else {
        return vec![];
    };
    let Some(s) = zeilen[d + 1..]
        .iter()
        .take_while(|z| einrueckung(z) > 0)
        .position(|z| z.trim() == "server:")
        .map(|p| p + d + 1)
    else {
        return vec![];
    };
    let tiefe = einrueckung(zeilen[s]);
    let block: Vec<&str> = zeilen[s + 1..].iter().copied().take_while(|z| einrueckung(z) > tiefe).collect();
    let Some(namen_tiefe) = block.iter().map(|z| einrueckung(z)).min() else {
        return vec![];
    };
    let mut out = vec![];
    for (i, z) in block.iter().enumerate() {
        if einrueckung(z) != namen_tiefe {
            continue;
        }
        let name = z.trim().trim_end_matches(':').trim_matches(['"', '\'']).to_string();
        let optional = block[i + 1..]
            .iter()
            .take_while(|w| einrueckung(w) > namen_tiefe)
            .any(|w| w.replace(' ', "") == "required:false");
        if !optional && !name.is_empty() {
            out.push(name);
        }
    }
    out
}

/// Bedarf aus einer (evtl. verschachtelten) Jar. Mitgelieferte Jars (Fabric `jars`,
/// NeoForge `META-INF/jarjar`) zählen als bereitgestellt – eine Ebene tief.
fn bedarf_zip<R: Read + Seek>(z: &mut zip::ZipArchive<R>, verschachtelt: bool) -> Bedarf {
    let mut b = Bedarf::default();
    let innere = |z: &mut zip::ZipArchive<R>, pfad: &str| -> Option<Bedarf> {
        let mut e = z.by_name(pfad).ok()?;
        let mut bytes = vec![];
        e.read_to_end(&mut bytes).ok()?;
        let mut inner = zip::ZipArchive::new(Cursor::new(bytes)).ok()?;
        Some(bedarf_zip(&mut inner, false))
    };
    if let Some(v) = fabric_json(z) {
        let ids = |k: &str| -> Vec<String> {
            match v.get(k) {
                Some(Value::String(s)) => vec![s.to_lowercase()],
                Some(Value::Array(a)) => a.iter().filter_map(|x| x.as_str()).map(str::to_lowercase).collect(),
                _ => vec![],
            }
        };
        b.stellt.extend(ids("id"));
        b.stellt.extend(ids("provides"));
        if let Some(d) = v.get("depends").and_then(|d| d.as_object()) {
            b.braucht.extend(d.keys().map(|k| k.to_lowercase()));
        }
        if verschachtelt {
            let pfade: Vec<String> = v
                .get("jars")
                .and_then(|j| j.as_array())
                .into_iter()
                .flatten()
                .filter_map(|j| j.get("file").and_then(|f| f.as_str()).map(str::to_string))
                .collect();
            for p in pfade {
                if let Some(inner) = innere(z, &p) {
                    b.stellt.extend(inner.stellt);
                }
            }
        }
        return b;
    }
    if let Some(t) = read_entry(z, "META-INF/neoforge.mods.toml") {
        b = toml_bedarf(&t);
        if verschachtelt {
            let pfade: Vec<String> = z
                .file_names()
                .filter(|n| n.starts_with("META-INF/jarjar/") && n.ends_with(".jar"))
                .map(str::to_string)
                .collect();
            for p in pfade {
                if let Some(inner) = innere(z, &p) {
                    b.stellt.extend(inner.stellt);
                }
            }
        }
        return b;
    }
    if let Some(t) = read_entry(z, "paper-plugin.yml") {
        b.stellt.extend(simple_value(&t, "name", ':').map(|n| n.to_lowercase()));
        b.braucht.extend(paper_pflicht(&t).iter().map(|n| n.to_lowercase()));
    } else if let Some(t) = read_entry(z, "plugin.yml") {
        b.stellt.extend(simple_value(&t, "name", ':').map(|n| n.to_lowercase()));
        b.braucht.extend(yml_liste(&t, "depend").iter().map(|n| n.to_lowercase()));
    }
    b
}

fn bedarf_jar(pfad: &Path) -> Bedarf {
    match fs::File::open(pfad).ok().and_then(|f| zip::ZipArchive::new(f).ok()) {
        Some(mut z) => bedarf_zip(&mut z, true),
        None => Bedarf::default(),
    }
}

/// Wer braucht wen – nur eingeschaltete Mods zählen als „braucht“.
#[derive(Default)]
pub struct Abhaengigkeiten {
    /// Datei → Anzeigename
    titel: HashMap<String, String>,
    /// Datei → Dateien, die sie zwingend braucht
    braucht: HashMap<String, Vec<String>>,
    /// Datei → eingeschaltete Dateien, die sie brauchen
    nutzer: HashMap<String, Vec<String>>,
}

pub fn abhaengigkeiten(dir: &Path) -> Abhaengigkeiten {
    let sub = crate::flavor::mod_dir_of(dir);
    let mut dateien = vec![]; // (rel, eingeschaltet, Bedarf)
    let mut a = Abhaengigkeiten::default();
    if let Ok(rd) = fs::read_dir(dir.join(sub)) {
        for e in rd.filter_map(|e| e.ok()).filter(|e| e.path().is_file()) {
            let name = e.file_name().to_string_lossy().into_owned();
            let (base, an) = match name.strip_suffix(DISABLED) {
                Some(b) if b.ends_with(".jar") => (b.to_string(), false),
                _ if name.ends_with(".jar") => (name.clone(), true),
                _ => continue,
            };
            let rel = format!("{sub}/{base}");
            let titel = mod_info(&e.path()).title.unwrap_or_else(|| base.trim_end_matches(".jar").to_string());
            a.titel.insert(rel.clone(), titel);
            dateien.push((rel, an, bedarf_jar(&e.path())));
        }
    }
    // Wer stellt welche Kennung bereit? Eingeschaltete Dateien haben Vorrang.
    let mut anbieter: HashMap<&str, &str> = HashMap::new();
    for nur_an in [true, false] {
        for (rel, an, b) in &dateien {
            if *an == nur_an {
                for id in &b.stellt {
                    anbieter.entry(id.as_str()).or_insert(rel.as_str());
                }
            }
        }
    }
    for (rel, an, b) in &dateien {
        if !*an {
            continue;
        }
        let mut gesehen = HashSet::new();
        for id in &b.braucht {
            if let Some(p) = anbieter.get(id.as_str()).filter(|p| **p != rel.as_str())
                && gesehen.insert(*p)
            {
                a.braucht.entry(rel.clone()).or_default().push(p.to_string());
                a.nutzer.entry(p.to_string()).or_default().push(rel.clone());
            }
        }
    }
    a
}

impl Abhaengigkeiten {
    fn titel_von(&self, rels: &[String]) -> Vec<String> {
        let mut t: Vec<String> = rels.iter().map(|r| self.titel.get(r).cloned().unwrap_or_else(|| r.clone())).collect();
        t.sort_by_key(|s| s.to_lowercase());
        t
    }

    /// Anzeigenamen der eingeschalteten Mods, die `rel` brauchen.
    pub fn gebraucht_von(&self, rel: &str) -> Vec<String> {
        self.titel_von(self.nutzer.get(rel).map(Vec::as_slice).unwrap_or_default())
    }
}

/// Vor dem Entfernen bzw. Ausschalten: Wer braucht diese Datei noch, und welche Helfer
/// würden danach von keinem anderen eingeschalteten Mod mehr gebraucht?
/// Vorgeschlagen (Häkchen gesetzt) werden nur Helfer, die als Abhängigkeit mitkamen
/// oder Bibliotheken sind.
pub fn check(dir: &Path, rel: &str) -> Value {
    let a = abhaengigkeiten(dir);
    let srcs = sources(dir);
    let infos = section(dir, "infos");
    let mut waisen = vec![];
    for p in a.braucht.get(rel).into_iter().flatten() {
        let andere = a.nutzer.get(p).into_iter().flatten().any(|n| n != rel);
        if andere {
            continue;
        }
        let als_abhaengigkeit = srcs.get(p).and_then(|s| s.get("dep")).and_then(|d| d.as_bool()) == Some(true);
        let bibliothek = infos.get(p).and_then(|i| i.get("lib")).and_then(|l| l.as_bool()) == Some(true);
        waisen.push(json!({
            "rel": p,
            "title": a.titel.get(p).cloned().unwrap_or_else(|| p.clone()),
            "suggest": als_abhaengigkeit || bibliothek,
        }));
    }
    json!({"needed_by": a.gebraucht_von(rel), "orphans": waisen})
}

// ---------------------------------------------------------------- Zusatz-Infos (Symbol, Beschreibung, Link)

fn modrinth_info(p: &Value) -> Value {
    let typ = p.get("project_type").and_then(|t| t.as_str()).unwrap_or("mod");
    let slug = p.get("slug").and_then(|s| s.as_str()).unwrap_or("");
    json!({
        "title": p.get("title"),
        "icon": p.get("icon_url"),
        "desc": p.get("description"),
        "page": (!slug.is_empty()).then(|| format!("https://modrinth.com/{typ}/{slug}")),
        "lib": p.get("categories").and_then(|c| c.as_array()).is_some_and(|c| c.iter().any(|x| x == "library")),
    })
}

fn curseforge_info(m: &Value) -> Value {
    json!({
        "title": m.get("name"),
        "icon": m.pointer("/logo/thumbnailUrl"),
        "desc": m.get("summary"),
        "page": m.pointer("/links/websiteUrl"),
        "lib": m.get("categories").and_then(|c| c.as_array()).is_some_and(|c| {
            c.iter().any(|x| x.get("slug").and_then(|s| s.as_str()).is_some_and(|s| s.contains("library")))
        }),
    })
}

/// Fehlende Infos nachschlagen: bekannte Herkunft direkt, sonst über den
/// Datei-Fingerabdruck bei Modrinth (so erkennt auch der Update-Assistent Mods).
/// Netzfehler lassen den Eintrag offen – beim nächsten Mal wird es wieder versucht.
fn nachschlagen(eintraege: &[(String, PathBuf)], srcs: &Map<String, Value>, infos: &mut Map<String, Value>) {
    let mut modrinth: Vec<(String, String)> = vec![]; // (Datei, Projekt-ID)
    let mut cf: Vec<(String, u64)> = vec![];
    let mut fingerabdruck: Vec<(String, String)> = vec![]; // (Datei, SHA-1)
    for (rel, pfad) in eintraege {
        let s = srcs.get(rel);
        let quelle = s.and_then(|s| s.get("source")).and_then(|q| q.as_str());
        match (quelle, s.and_then(|s| s.get("id"))) {
            (Some("modrinth"), Some(Value::String(id))) => modrinth.push((rel.clone(), id.clone())),
            (Some("curseforge"), Some(id)) if id.as_u64().is_some() => cf.push((rel.clone(), id.as_u64().unwrap_or(0))),
            _ if pfad.is_file() => {
                if let Ok(h) = net::sha1_file(pfad) {
                    fingerabdruck.push((rel.clone(), h));
                }
            }
            _ => {}
        }
    }
    if !fingerabdruck.is_empty() {
        let hashes: Vec<String> = fingerabdruck.iter().map(|(_, h)| h.clone()).collect();
        if let Ok(gefunden) = updater::version_files(&hashes) {
            for (rel, h) in &fingerabdruck {
                match gefunden.get(h).and_then(|v| v.get("project_id")).and_then(|p| p.as_str()) {
                    Some(pid) => modrinth.push((rel.clone(), pid.to_string())),
                    // nicht bei Modrinth – merken, damit nicht jedes Mal neu gefragt wird
                    None => {
                        infos.insert(rel.clone(), json!({"none": true}));
                    }
                }
            }
        }
    }
    let mut ids: Vec<&str> = modrinth.iter().map(|(_, id)| id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    let mut projekte: HashMap<String, Value> = HashMap::new();
    for paket in ids.chunks(100) {
        let Ok(v) = net::get_json_query(&format!("{MODRINTH_API}/projects"), &[("ids", json!(paket).to_string())]) else {
            continue;
        };
        for p in v.as_array().into_iter().flatten() {
            if let Some(id) = p.get("id").and_then(|i| i.as_str()) {
                projekte.insert(id.to_string(), modrinth_info(p));
            }
        }
    }
    for (rel, id) in &modrinth {
        if let Some(info) = projekte.get(id) {
            infos.insert(rel.clone(), info.clone());
        }
    }
    // CurseForge nur mit hinterlegtem Schlüssel
    if !cf.is_empty() && curseforge::key().is_some() {
        let ids: Vec<u64> = cf.iter().map(|(_, id)| *id).collect();
        if let Ok(liste) = curseforge::mods_info(&ids) {
            for (rel, id) in &cf {
                if let Some(m) = liste.iter().find(|m| m.get("id").and_then(|i| i.as_u64()) == Some(*id)) {
                    infos.insert(rel.clone(), curseforge_info(m));
                }
            }
        }
    }
}

/// Zusatz-Infos für die Liste: Datei → Symbol, Kurzbeschreibung, Link zur Mod-Seite,
/// Bibliothek?, gebraucht von. Fehlendes wird nachgeschlagen und in `.vapafane.json` gemerkt.
pub fn details(dir: &Path) -> Value {
    let liste = list(dir);
    let mut eintraege = vec![];
    for e in liste["mods"].as_array().into_iter().chain(liste["datapacks"].as_array()).flatten() {
        let rel = e["rel"].as_str().unwrap_or_default().to_string();
        if rel.is_empty() || e["folder"].as_bool() == Some(true) {
            continue;
        }
        let pfad = if e["enabled"].as_bool() == Some(true) {
            dir.join(&rel)
        } else {
            dir.join(format!("{rel}{DISABLED}"))
        };
        eintraege.push((rel, pfad));
    }
    let mut infos = section(dir, "infos");
    let fehlen: Vec<(String, PathBuf)> = eintraege.iter().filter(|(r, _)| !infos.contains_key(r)).cloned().collect();
    if !fehlen.is_empty() {
        nachschlagen(&fehlen, &sources(dir), &mut infos);
        // nur Einträge zu vorhandenen Dateien behalten
        infos.retain(|rel, _| eintraege.iter().any(|(r, _)| r == rel));
        let mut meta = read_meta(dir);
        meta.insert("infos".into(), Value::Object(infos.clone()));
        let _ = write_meta(dir, &meta);
    }
    let a = abhaengigkeiten(dir);
    let mut out = Map::new();
    for (rel, _) in &eintraege {
        let i = infos.get(rel).cloned().unwrap_or(json!({}));
        out.insert(
            rel.clone(),
            json!({
                "title": i.get("title"),
                "icon": i.get("icon"),
                "desc": i.get("desc"),
                "page": i.get("page"),
                "lib": i.get("lib"),
                "needed_by": a.gebraucht_von(rel),
            }),
        );
    }
    Value::Object(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn converts_old_notes_once() {
        let dir = std::env::temp_dir().join(format!("vapafane-alt-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(OLD_META_FILE),
            r#"{"mod_notes":{"mods/a.jar":"alt","mods/b.jar":"alt"}}"#,
        )
        .unwrap();
        fs::write(dir.join(META_FILE), r#"{"mod_notes":{"mods/a.jar":"neu"}}"#).unwrap();
        convert_old_meta(&dir);
        assert!(!dir.join(OLD_META_FILE).exists());
        assert_eq!(note(&dir, "mods/a.jar").as_deref(), Some("neu"));
        assert_eq!(note(&dir, "mods/b.jar").as_deref(), Some("alt"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn toggle_and_reason() {
        let dir = std::env::temp_dir().join(format!("vapafane-mods-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("mods")).unwrap();
        fs::write(dir.join("mods/a.jar"), "x").unwrap();
        disable(&dir, "mods/a.jar", grund("upd-grund-fehlt", &[("mc", "26.3")])).unwrap();
        assert!(dir.join("mods/a.jar.disabled").exists());
        let l = list(&dir);
        assert_eq!(l["mods"][0]["enabled"], false);
        // Grund wird beim Anzeigen in der aktuellen Sprache aufgebaut
        assert_eq!(l["mods"][0]["reason"], crate::sprache::t!("upd-grund-fehlt", mc = "26.3"));
        set_enabled(&dir, "mods/a.jar", true).unwrap();
        assert!(dir.join("mods/a.jar").exists());
        assert!(note(&dir, "mods/a.jar").is_none());
        set_enabled(&dir, "mods/a.jar", false).unwrap();
        assert_eq!(note(&dir, "mods/a.jar"), Some(crate::sprache::t!("mods-grund-hand")));
        assert!(set_enabled(&dir, "../x.jar", false).is_err());
        assert!(set_enabled(&dir, "mods", false).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn entfernen() {
        let dir = std::env::temp_dir().join(format!("vapafane-weg-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("mods")).unwrap();
        fs::create_dir_all(dir.join("world/datapacks/ordner")).unwrap();
        fs::write(dir.join("mods/a.jar"), "x").unwrap();
        fs::write(dir.join("mods/b.jar.disabled"), "x").unwrap();
        fs::write(dir.join("world/datapacks/p.zip"), "x").unwrap();
        set_source(&dir, "mods/a.jar", Some(json!({"source": "modrinth", "id": "x"}))).unwrap();
        remove(&dir, "mods/a.jar").unwrap();
        remove(&dir, "mods/b.jar").unwrap();
        remove(&dir, "world/datapacks/ordner").unwrap();
        remove(&dir, "world/datapacks/p.zip").unwrap();
        assert!(!dir.join("mods/a.jar").exists() && !dir.join("mods/b.jar.disabled").exists());
        assert!(!dir.join("world/datapacks/ordner").exists() && !dir.join("world/datapacks/p.zip").exists());
        assert!(sources(&dir).is_empty());
        // Nur Einträge direkt in den Mod-/Datapack-Ordnern – nie die Ordner selbst oder Fremdes
        assert!(remove(&dir, "mods/a.jar").is_err());
        assert!(remove(&dir, "mods").is_err());
        assert!(remove(&dir, "world/datapacks").is_err());
        assert!(remove(&dir, "../etwas.jar").is_err());
        assert!(remove(&dir, "server.properties").is_err());
        assert!(dir.join("mods").is_dir() && dir.join("world/datapacks").is_dir());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn abhaengigkeiten_lesen() {
        let t = "[[mods]]\nmodId=\"meinmod\"\n[[dependencies.meinmod]]\nmodId=\"neoforge\"\ntype=\"required\"\n\
                 [[dependencies.meinmod]]\nmodId=\"jei\"\ntype=\"optional\"\n[[dependencies.meinmod]]\nmodId=\"geckolib\"\nmandatory=true\n";
        let b = toml_bedarf(t);
        assert_eq!(b.stellt, ["meinmod"]);
        assert_eq!(b.braucht, ["neoforge", "geckolib"]);
        assert_eq!(yml_liste("name: A\ndepend: [Vault, 'LuckPerms']\n", "depend"), ["Vault", "LuckPerms"]);
        assert_eq!(yml_liste("depend:\n  - Vault\n  - WorldEdit\nsoftdepend: [X]\n", "depend"), ["Vault", "WorldEdit"]);
        let paper = "name: B\ndependencies:\n  server:\n    Vault:\n      load: BEFORE\n    Extra:\n      required: false\n  bootstrap:\n    Z:\n";
        assert_eq!(paper_pflicht(paper), ["Vault"]);
    }

    fn jar_bytes(dateien: &[(&str, &[u8])]) -> Vec<u8> {
        let mut w = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, inhalt) in dateien {
            w.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            w.write_all(inhalt).unwrap();
        }
        w.finish().unwrap().into_inner()
    }

    #[test]
    fn wer_braucht_wen() {
        let dir = std::env::temp_dir().join(format!("vapafane-abh-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("mods")).unwrap();
        let basis = jar_bytes(&[("fabric.mod.json", br#"{"id":"fabric-api-base","name":"Base"}"#)]);
        let api = jar_bytes(&[
            ("fabric.mod.json", br#"{"id":"fabric-api","name":"Fabric API","jars":[{"file":"META-INF/jars/base.jar"}]}"#),
            ("META-INF/jars/base.jar", &basis),
        ]);
        fs::write(dir.join("mods/fabric-api.jar"), api).unwrap();
        let sodium = jar_bytes(&[("fabric.mod.json", br#"{"id":"sodium","name":"Sodium","depends":{"fabric-api-base":"*","minecraft":"*"}}"#)]);
        fs::write(dir.join("mods/sodium.jar"), sodium).unwrap();
        let iris = jar_bytes(&[("fabric.mod.json", br#"{"id":"iris","name":"Iris","depends":{"sodium":"*"}}"#)]);
        fs::write(dir.join("mods/iris.jar"), iris).unwrap();
        set_source(&dir, "mods/fabric-api.jar", Some(json!({"source": "modrinth", "id": "P7dR8mSH", "dep": true}))).unwrap();

        let a = abhaengigkeiten(&dir);
        // Fabric API stellt das mitgelieferte Modul bereit – Sodium braucht es also
        assert_eq!(a.gebraucht_von("mods/fabric-api.jar"), ["Sodium"]);
        assert_eq!(a.gebraucht_von("mods/sodium.jar"), ["Iris"]);
        let c = check(&dir, "mods/sodium.jar");
        assert_eq!(c["needed_by"], json!(["Iris"]));
        assert_eq!(c["orphans"][0]["rel"], "mods/fabric-api.jar");
        assert_eq!(c["orphans"][0]["suggest"], true);
        // Sodium ist kein Helfer, der als Abhängigkeit kam – nicht vorschlagen
        let c = check(&dir, "mods/iris.jar");
        assert_eq!(c["orphans"][0]["rel"], "mods/sodium.jar");
        assert_eq!(c["orphans"][0]["suggest"], false);
        // Ausgeschaltete Mods brauchen nichts
        set_enabled(&dir, "mods/iris.jar", false).unwrap();
        assert!(abhaengigkeiten(&dir).gebraucht_von("mods/sodium.jar").is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }
}
