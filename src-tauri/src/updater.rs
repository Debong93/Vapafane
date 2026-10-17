//! Update-Assistent für Fabric, Paper, NeoForge und Vanilla: prüft Mods bzw. Plugins
//! und Datapacks über Modrinth und stellt einen Server auf eine neue Version um.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::archive::Progress;
use crate::config::{ALT_DIR_NAME, STAGING_NAME};
use crate::flavor::{self, Artifact, Kind};
use crate::java;
use crate::mods;
use crate::net;
use crate::server;
use crate::util::{self, Result, err};

const API: &str = "https://api.modrinth.com/v2";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewFile {
    pub url: String,
    pub filename: String,
    pub sha512: Option<String>,
    pub size: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Item {
    /// "mod" oder "datapack"
    pub kind: String,
    /// Dateiname (bei neuen Abhängigkeiten leer)
    pub file: String,
    /// Pfad relativ zum Server-Ordner
    pub rel: String,
    pub sha1: Option<String>,
    pub project_id: Option<String>,
    pub title: String,
    pub current_version: Option<String>,
    pub new_version: Option<String>,
    pub new_file: Option<NewFile>,
    /// update, current, missing, unknown, new, duplicate
    pub status: String,
    /// Datei ist ausgeschaltet (`.disabled`) – bei „update“ wird die neue Version eingeschaltet
    #[serde(default)]
    pub disabled: bool,
    pub client_side: Option<String>,
    pub server_side: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Plan {
    pub dir: PathBuf,
    /// fabric, vanilla, paper, neoforge
    pub kind: String,
    /// "mods" oder "plugins" (leer bei Vanilla)
    pub mod_dir: String,
    pub target: String,
    pub current: String,
    pub change_version: bool,
    /// Neue Server-Datei (bei neuer Version oder neuerem Paper-Build/NeoForge)
    pub server: Option<Artifact>,
    pub mods: Vec<Item>,
    pub extra: Vec<Item>,
    pub datapacks: Vec<Item>,
    pub java_required: Option<u32>,
    pub java_have: u32,
    pub java_path: String,
    pub java_ok: bool,
    pub server_size: u64,
    pub free_space: u64,
    pub space_ok: bool,
}

impl Plan {
    pub fn count(&self, status: &str) -> usize {
        self.mods
            .iter()
            .chain(&self.datapacks)
            .filter(|m| m.status == status)
            .count()
    }

    /// Gibt es überhaupt etwas zu tun?
    pub fn has_work(&self) -> bool {
        self.change_version
            || self.server.is_some()
            || self.count("update") > 0
            || self.count("duplicate") > 0
            || self.count("reenable") > 0
            || self.extra.iter().any(|e| e.status == "new")
    }
}

pub(crate) fn primary_file(version: &Value) -> Option<NewFile> {
    let files = version.get("files")?.as_array()?;
    let f = files
        .iter()
        .find(|f| f.get("primary").and_then(|p| p.as_bool()).unwrap_or(false))
        .or(files.first())?;
    Some(NewFile {
        url: f.get("url")?.as_str()?.to_string(),
        filename: f.get("filename")?.as_str()?.to_string(),
        sha512: f.pointer("/hashes/sha512").and_then(|h| h.as_str()).map(str::to_string),
        size: f.get("size").and_then(|s| s.as_u64()).unwrap_or(0),
    })
}

fn s(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(|x| x.as_str()).map(str::to_string)
}

/// Nur einfache Dateinamen zulassen (Modrinth-Angaben landen im Server-Ordner).
pub(crate) fn safe_filename(name: &str) -> Result<String> {
    if name.is_empty() || name.contains(['/', '\\']) || name.starts_with('.') || name.contains("..") {
        return err(format!("Ungültiger Dateiname von Modrinth: {name}"));
    }
    Ok(name.to_string())
}

/// Mods bzw. Plugins (Unterordner `sub`, z. B. "mods" oder "plugins").
pub fn installed_mods(dir: &Path) -> Vec<Item> {
    installed_in(dir, flavor::mod_dir_of(dir))
}

fn installed_in(dir: &Path, sub: &str) -> Vec<Item> {
    let Ok(rd) = fs::read_dir(dir.join(sub)) else {
        return vec![];
    };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".jar"))
        .collect();
    names.sort_by_key(|n| n.to_lowercase());
    names
        .into_iter()
        .map(|n| Item {
            kind: "mod".into(),
            sha1: net::sha1_file(&dir.join(sub).join(&n)).ok(),
            rel: format!("{sub}/{n}"),
            title: n.clone(),
            file: n,
            ..Default::default()
        })
        .collect()
}

pub(crate) fn level_name(dir: &Path) -> String {
    server::read_properties(dir)
        .get("level-name")
        .cloned()
        .filter(|l| !l.is_empty())
        .unwrap_or_else(|| "world".into())
}

/// ZIP-Datapacks im Welt-Ordner (Ordner-Datapacks lassen sich nicht prüfen).
pub fn installed_datapacks(dir: &Path) -> Vec<Item> {
    let level = level_name(dir);
    let dp = dir.join(&level).join("datapacks");
    let Ok(rd) = fs::read_dir(&dp) else {
        return vec![];
    };
    let mut items: Vec<Item> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            let p = e.path();
            let is_zip = n.to_lowercase().ends_with(".zip") && p.is_file();
            if !is_zip && !p.is_dir() {
                return None;
            }
            Some(Item {
                kind: "datapack".into(),
                sha1: if is_zip { net::sha1_file(&p).ok() } else { None },
                rel: format!("{level}/datapacks/{n}"),
                title: n.clone(),
                file: n,
                ..Default::default()
            })
        })
        .collect();
    items.sort_by_key(|i| i.file.to_lowercase());
    items
}

pub(crate) fn version_files(hashes: &[String]) -> Result<HashMap<String, Value>> {
    if hashes.is_empty() {
        return Ok(HashMap::new());
    }
    let v = net::post_json(
        &format!("{API}/version_files"),
        &json!({"hashes": hashes, "algorithm": "sha1"}),
    )?;
    Ok(serde_json::from_value(v).unwrap_or_default())
}

fn version_updates(hashes: &[String], loaders: &[&str], target: &str) -> Result<HashMap<String, Value>> {
    if hashes.is_empty() {
        return Ok(HashMap::new());
    }
    let v = net::post_json(
        &format!("{API}/version_files/update"),
        &json!({"hashes": hashes, "algorithm": "sha1", "loaders": loaders, "game_versions": [target]}),
    )?;
    Ok(serde_json::from_value(v).unwrap_or_default())
}

pub(crate) fn newest_for_project(project_id: &str, loaders: &[&str], target: &str) -> Result<Option<Value>> {
    let v = net::get_json_query(
        &format!("{API}/project/{project_id}/version"),
        &[
            ("loaders", json!(loaders).to_string()),
            ("game_versions", json!([target]).to_string()),
        ],
    )?;
    // Neueste Release-Version bevorzugen, sonst die neueste überhaupt (Beta/Alpha)
    let list = v.as_array().cloned().unwrap_or_default();
    Ok(list
        .iter()
        .find(|x| x.get("version_type").and_then(|t| t.as_str()) == Some("release"))
        .or(list.first())
        .cloned())
}

fn project_infos(ids: &HashSet<String>) -> Result<HashMap<String, Value>> {
    let mut out = HashMap::new();
    let ids: Vec<&String> = ids.iter().collect();
    for chunk in ids.chunks(100) {
        let v = net::get_json_query(&format!("{API}/projects"), &[("ids", json!(chunk).to_string())])?;
        for p in v.as_array().cloned().unwrap_or_default() {
            if let Some(id) = s(&p, "id") {
                out.insert(id, p);
            }
        }
    }
    Ok(out)
}

/// Vergleicht installierte Dateien mit Modrinth und trägt den Status ein.
fn classify(items: &mut [Item], known: &HashMap<String, Value>, updates: &HashMap<String, Value>) {
    for m in items.iter_mut() {
        let Some(h) = m.sha1.clone() else {
            m.status = "unknown".into();
            continue;
        };
        let cur = known.get(&h);
        let new = updates.get(&h);
        m.project_id = cur.and_then(|c| s(c, "project_id"));
        m.current_version = cur.and_then(|c| s(c, "version_number"));
        m.status = match (cur, new) {
            (None, _) => "unknown",
            (Some(_), None) => "missing",
            (Some(c), Some(n)) if s(c, "id") == s(n, "id") => "current",
            (Some(_), Some(n)) => {
                m.new_version = s(n, "version_number");
                m.new_file = primary_file(n);
                "update"
            }
        }
        .into();
    }
}

/// Erstellt einen Update-Plan. Ändert nichts.
pub fn check(dir: &Path, target: &str, progress: Progress) -> Result<Plan> {
    let info = server::detect_server(dir);
    let Some(kind) = Kind::parse(&info.kind) else {
        return err("Den Update-Assistenten gibt es für Fabric-, Paper-, NeoForge- und Vanilla-Server.");
    };
    let loaders = kind.loaders();
    let sub = kind.mod_dir().unwrap_or("");
    progress(0.05, "Lese installierte Mods und Plugins …");
    let mut mods = if sub.is_empty() { vec![] } else { installed_in(dir, sub) };
    // Ausgeschaltete mitprüfen: gibt es jetzt eine passende Version, wird sie wieder eingeschaltet
    let disabled = if sub.is_empty() {
        vec![]
    } else {
        mods::disabled_in(dir, sub)
    };
    let mut off: Vec<Item> = disabled
        .into_iter()
        .map(|(base, path)| Item {
            kind: "mod".into(),
            sha1: net::sha1_file(&path).ok(),
            rel: format!("{sub}/{base}"),
            title: base.clone(),
            file: base,
            disabled: true,
            ..Default::default()
        })
        .collect();
    let hashes: Vec<String> = mods.iter().chain(&off).filter_map(|m| m.sha1.clone()).collect();
    progress(0.2, "Frage Modrinth nach den Mods …");
    let known = version_files(&hashes)?;
    let updates = version_updates(&hashes, loaders, target)?;
    classify(&mut mods, &known, &updates);
    classify(&mut off, &known, &updates);
    // Ausgeschaltete nur aufnehmen, wenn es eine passende Version gibt und das Projekt nicht schon aktiv ist
    let active: HashSet<String> = mods.iter().filter_map(|m| m.project_id.clone()).collect();
    let mut seen_off = HashSet::new();
    off.retain(|m| {
        let pid = m.project_id.clone().unwrap_or_default();
        matches!(m.status.as_str(), "update" | "current") && !active.contains(&pid) && seen_off.insert(pid)
    });
    for m in off.iter_mut().filter(|m| m.status == "current") {
        m.status = "reenable".into(); // genau diese Version passt (wieder)
    }
    mods.extend(off);

    // Pflicht-Abhängigkeiten der neuen Versionen, die noch nicht installiert sind
    progress(0.4, "Prüfe Abhängigkeiten …");
    let installed: HashSet<String> = mods.iter().filter_map(|m| m.project_id.clone()).collect();
    let mut seen = installed.clone();
    let mut extra: Vec<Item> = vec![];
    let mut queue: Vec<Value> = mods
        .iter()
        .filter(|m| m.status == "update" || m.status == "current")
        .filter_map(|m| {
            let h = m.sha1.as_ref()?;
            updates.get(h).or(known.get(h)).cloned()
        })
        .collect();
    for _ in 0..3 {
        let mut next = vec![];
        for version in &queue {
            for dep in version
                .get("dependencies")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default()
            {
                let Some(pid) = s(&dep, "project_id") else { continue };
                if s(&dep, "dependency_type").as_deref() != Some("required") || !seen.insert(pid.clone()) {
                    continue;
                }
                match newest_for_project(&pid, loaders, target)? {
                    Some(v) => {
                        extra.push(Item {
                            kind: "mod".into(),
                            project_id: Some(pid),
                            new_version: s(&v, "version_number"),
                            new_file: primary_file(&v),
                            status: "new".into(),
                            ..Default::default()
                        });
                        next.push(v);
                    }
                    None => extra.push(Item {
                        kind: "mod".into(),
                        project_id: Some(pid),
                        file: "(Abhängigkeit)".into(),
                        status: "missing".into(),
                        ..Default::default()
                    }),
                }
            }
        }
        queue = next;
        if queue.is_empty() {
            break;
        }
    }

    // Datapacks im Welt-Ordner: gleiche Prüfung mit Loader "datapack"
    progress(0.6, "Prüfe Datapacks …");
    let mut packs = installed_datapacks(dir);
    let pack_hashes: Vec<String> = packs.iter().filter_map(|p| p.sha1.clone()).collect();
    let pk_known = version_files(&pack_hashes)?;
    let pk_updates = version_updates(&pack_hashes, &["datapack"], target)?;
    classify(&mut packs, &pk_known, &pk_updates);
    let mod_projects: HashSet<String> = installed
        .iter()
        .cloned()
        .chain(
            extra
                .iter()
                .filter(|e| e.status == "new")
                .filter_map(|e| e.project_id.clone()),
        )
        .collect();
    for p in packs.iter_mut() {
        // Gleiches Projekt ist schon als Mod installiert → Datapack wäre doppelt
        if p.project_id
            .as_ref()
            .map(|id| mod_projects.contains(id))
            .unwrap_or(false)
        {
            p.status = "duplicate".into();
            p.new_file = None;
            p.new_version = None;
        }
    }

    progress(0.75, "Lade Namen der Mods …");
    let ids: HashSet<String> = mods
        .iter()
        .chain(&extra)
        .chain(&packs)
        .filter_map(|m| m.project_id.clone())
        .collect();
    let infos = project_infos(&ids)?;
    for m in mods.iter_mut().chain(extra.iter_mut()).chain(packs.iter_mut()) {
        if let Some(p) = m.project_id.as_ref().and_then(|id| infos.get(id)) {
            m.title = s(p, "title").unwrap_or_else(|| m.file.clone());
            m.client_side = s(p, "client_side");
            m.server_side = s(p, "server_side");
        } else if m.title.is_empty() {
            m.title = m.file.clone();
        }
    }

    let env = server::read_env(dir);
    let change_version = target != info.version;
    let mut plan = Plan {
        dir: dir.to_path_buf(),
        kind: kind.id().into(),
        mod_dir: sub.into(),
        target: target.into(),
        current: info.version.clone(),
        change_version,
        mods,
        extra,
        datapacks: packs,
        ..Default::default()
    };
    progress(0.85, &format!("Suche {}-Server für {target} …", kind.label()));
    if change_version {
        plan.server = Some(flavor::server_artifact(kind, target)?);
    } else if matches!(kind, Kind::Paper | Kind::NeoForge) {
        // Gleiche Minecraft-Version: gibt es einen neueren Paper-Build bzw. ein neueres NeoForge?
        let art = flavor::server_artifact(kind, target)?;
        if flavor::newer_build(kind, dir, &art) {
            plan.server = Some(art);
        }
    }
    if plan.server.is_some() {
        plan.java_required = net::required_java(target)?;
    }
    progress(0.9, "Prüfe Java und Speicherplatz …");
    let java_now = env.get("MC_JAVA").cloned().unwrap_or_else(java::default_java);
    plan.java_have = java::java_major(&java::resolve_java(&java_now));
    plan.java_path = java_now;
    plan.java_ok = true;
    if let Some(need) = plan.java_required
        && plan.java_have < need
    {
        match java::java_at_least(need) {
            Some(p) => plan.java_path = p,
            None => plan.java_ok = false,
        }
    }
    plan.server_size = util::dir_size(dir);
    plan.free_space = util::free_space(dir);
    plan.space_ok = plan.free_space as f64 > plan.server_size as f64 * 1.3 + 2.0 * 1024f64.powi(3);
    Ok(plan)
}

/// Lädt alles herunter, prüft es und tauscht dann die Dateien.
/// Der Server muss gestoppt sein. Alte Dateien landen in `.panel-alt/vor_Update_<Datum>`.
pub fn apply(
    dir: &Path,
    plan: &Plan,
    disable_missing: bool,
    disable_unknown: bool,
    progress: Progress,
) -> Result<PathBuf> {
    let staging = dir.join(STAGING_NAME);
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(staging.join("mods"))?;
    fs::create_dir_all(staging.join("datapacks"))?;
    let alt = dir.join(ALT_DIR_NAME).join(format!("vor_Update_{}", util::now_stamp()));
    let level = level_name(dir);

    let downloads: Vec<&Item> = plan
        .mods
        .iter()
        .filter(|m| m.status == "update")
        .chain(plan.extra.iter().filter(|m| m.status == "new"))
        .chain(plan.datapacks.iter().filter(|p| p.status == "update"))
        .collect();
    let kind = Kind::parse(&plan.kind).unwrap_or(Kind::Fabric);
    let mod_sub = if plan.mod_dir.is_empty() {
        "mods"
    } else {
        plan.mod_dir.as_str()
    };
    let total = (downloads.len() + usize::from(plan.server.is_some())).max(1) as f64;
    for (i, m) in downloads.iter().enumerate() {
        let Some(f) = &m.new_file else { continue };
        let name = safe_filename(&f.filename)?;
        let sub = if m.kind == "datapack" { "datapacks" } else { "mods" };
        let text = format!("Lade {} {} …", m.title, m.new_version.clone().unwrap_or_default());
        progress(i as f64 / total, &text);
        net::download(
            &f.url,
            &staging.join(sub).join(&name),
            net::Expect {
                sha512: f.sha512.as_deref(),
                ..Default::default()
            },
            &|_| {},
        )?;
    }
    if let Some(art) = &plan.server {
        progress(downloads.len() as f64 / total, &format!("Lade {} …", art.label));
        let name = safe_filename(&art.file)?;
        art.download(&staging.join("server").join(&name), &|_| {})?;
    }

    // Ab hier wird getauscht – alles ist heruntergeladen und geprüft
    progress(0.97, "Tausche Dateien …");
    fs::create_dir_all(alt.join("mods"))?;
    fs::create_dir_all(alt.join("datapacks"))?;
    // Ersetzte Dateien kommen nach .panel-alt (nach geglücktem Start wird aufgeräumt).
    // Unpassende werden ausgeschaltet (.disabled) und bleiben mit Grund in der Mod-Liste.
    let handle_old = |m: &Item| -> Result<()> {
        if m.rel.is_empty() {
            return Ok(());
        }
        let sub = if m.kind == "datapack" { "datapacks" } else { "mods" }; // Unterordner in .panel-alt
        let src = if m.disabled {
            PathBuf::from(format!("{}{}", dir.join(&m.rel).display(), mods::DISABLED))
        } else {
            dir.join(&m.rel)
        };
        if !src.exists() {
            return Ok(());
        }
        match m.status.as_str() {
            "update" => {
                fs::rename(&src, alt.join(sub).join(&m.file))?;
                mods::set_note(dir, &m.rel, None)?;
            }
            "missing" if disable_missing && !m.disabled => {
                mods::disable(dir, &m.rel, &format!("gibt es nicht für Minecraft {}", plan.target))?
            }
            "unknown" if disable_unknown && !m.disabled => {
                mods::disable(dir, &m.rel, "nicht auf Modrinth – passende Version unklar")?
            }
            "reenable" => mods::set_enabled(dir, &m.rel, true)?,
            "duplicate" => mods::disable(
                dir,
                &m.rel,
                &format!("doppelt – „{}“ ist schon als Mod installiert", m.title),
            )?,
            _ => {}
        }
        Ok(())
    };
    for m in plan.mods.iter().chain(&plan.datapacks) {
        handle_old(m)?;
    }
    // Zwischenspeicher der Mods wird beim nächsten Start neu erzeugt (Rückweg geht übers ZIP-Backup)
    let _ = fs::remove_dir_all(dir.join(".fabric").join("processedMods"));
    for m in &downloads {
        let Some(f) = &m.new_file else { continue };
        let name = safe_filename(&f.filename)?;
        if m.kind == "datapack" {
            let dp = dir.join(&level).join("datapacks");
            fs::create_dir_all(&dp)?;
            fs::rename(staging.join("datapacks").join(&name), dp.join(&name))?;
        } else {
            fs::create_dir_all(dir.join(mod_sub))?;
            fs::rename(staging.join("mods").join(&name), dir.join(mod_sub).join(&name))?;
        }
    }

    let mut env = vec![("MC_VERSION", plan.target.clone())];
    if let Some(art) = &plan.server {
        let new = staging.join("server").join(safe_filename(&art.file)?);
        let java = if plan.java_path.is_empty() {
            crate::java::default_java()
        } else {
            plan.java_path.clone()
        };
        let old_jar = server::read_env(dir).get("MC_JAR").cloned().unwrap_or_default();
        let move_aside = |name: &str| -> Result<()> {
            let p = dir.join(name);
            if !name.is_empty() && p.is_file() {
                fs::rename(&p, alt.join(name))?;
            }
            Ok(())
        };
        match kind {
            Kind::Fabric => {
                for old in [
                    "fabric-server-launch.jar",
                    "server.jar",
                    "fabric-server-launcher.properties",
                ] {
                    move_aside(old)?;
                }
                fs::rename(&new, dir.join("fabric-server-launch.jar"))?;
                env.push(("MC_JAR", "fabric-server-launch.jar".into()));
            }
            Kind::Vanilla | Kind::Paper => {
                move_aside(&old_jar)?;
                if old_jar != art.file {
                    move_aside(&art.file)?;
                }
                fs::rename(&new, dir.join(&art.file))?;
                env.push(("MC_JAR", art.file.clone()));
            }
            Kind::NeoForge => {
                // Der Installer legt die neue Version zusätzlich ab; die alte räumt das Aufräumen weg
                progress(0.98, &format!("Richte {} ein …", art.label));
                flavor::run_neoforge_installer(&new, dir, &java, &|_, _| {})?;
                env.push(("MC_JAR", String::new()));
            }
        }
        env.push(("MC_TYPE", kind.id().into()));
        env.push(("MC_LOADER_VERSION", art.build.clone()));
        if !plan.java_path.is_empty() {
            env.push(("MC_JAVA", plan.java_path.clone()));
        }
    }
    server::write_env(dir, &env)?;
    let _ = fs::remove_dir_all(&staging);
    progress(1.0, "Dateien getauscht.");
    Ok(alt)
}

/// Modrinth-Modpack (.mrpack) mit den Mods dieses Servers, für die Clients der Freunde.
/// Reine Server-Mods werden für den Client als „unsupported“ markiert.
pub fn export_mrpack(dir: &Path, dest: &Path, name: &str) -> Result<(usize, usize)> {
    let mods = installed_mods(dir);
    let hashes: Vec<String> = mods.iter().filter_map(|m| m.sha1.clone()).collect();
    let known = version_files(&hashes)?;
    let ids: HashSet<String> = known.values().filter_map(|v| s(v, "project_id")).collect();
    let infos = project_infos(&ids)?;
    let info = server::detect_server(dir);
    let mut files = vec![];
    let mut overrides = vec![];
    for m in &mods {
        let v = m.sha1.as_ref().and_then(|h| known.get(h));
        let f = v
            .and_then(|v| v.get("files"))
            .and_then(|f| f.as_array())
            .and_then(|fs_| {
                fs_.iter()
                    .find(|f| f.pointer("/hashes/sha1").and_then(|h| h.as_str()) == m.sha1.as_deref())
                    .cloned()
            });
        let (Some(v), Some(f)) = (v, f) else {
            overrides.push(m);
            continue;
        };
        let p = s(v, "project_id")
            .and_then(|id| infos.get(&id).cloned())
            .unwrap_or(Value::Null);
        let side = |key: &str| {
            if s(&p, key).as_deref() == Some("unsupported") {
                "unsupported"
            } else {
                "required"
            }
        };
        files.push(json!({
            "path": format!("mods/{}", s(&f, "filename").unwrap_or_else(|| m.file.clone())),
            "hashes": {"sha1": f.pointer("/hashes/sha1"), "sha512": f.pointer("/hashes/sha512")},
            "env": {"client": side("client_side"), "server": side("server_side")},
            "downloads": [f.get("url")],
            "fileSize": f.get("size"),
        }));
    }
    let (loader_key, loader) = match info.kind.as_str() {
        "fabric" if info.loader.is_empty() => ("fabric-loader", net::fabric_server_jar_url(&info.version)?.1),
        "fabric" => ("fabric-loader", info.loader.clone()),
        "neoforge" => ("neoforge", info.loader.clone()),
        _ => return err("Ein Modpack für Freunde gibt es nur bei Fabric- und NeoForge-Servern."),
    };
    let index = json!({
        "formatVersion": 1,
        "game": "minecraft",
        "versionId": chrono::Local::now().format("%Y.%m.%d").to_string(),
        "name": name,
        "summary": format!("Mods vom Server {name}"),
        "files": files,
        "dependencies": {"minecraft": info.version, loader_key: loader},
    });
    let tmp = dest.with_extension("part");
    {
        let mut z = zip::ZipWriter::new(fs::File::create(&tmp)?);
        let o = zip::write::SimpleFileOptions::default();
        z.start_file("modrinth.index.json", o)?;
        z.write_all(serde_json::to_string_pretty(&index)?.as_bytes())?;
        for m in &overrides {
            z.start_file(format!("overrides/mods/{}", m.file), o)?;
            z.write_all(&fs::read(dir.join(&m.rel))?)?;
        }
        z.finish()?;
    }
    fs::rename(&tmp, dest)?;
    Ok((files.len(), overrides.len()))
}

/// Infos für die Updates-Seite.
pub fn current_info(dir: &Path) -> Value {
    let info = server::detect_server(dir);
    json!({
        "info": info,
        "mod_dir": flavor::mod_dir_of(dir),
        "mods": installed_mods(dir).len(),
        "datapacks": installed_datapacks(dir).len(),
    })
}

/// Versionen zur Auswahl: neuere als die jetzige, dann die jetzige.
pub fn target_versions(kind: Kind, current: &str) -> Result<Vec<String>> {
    let all = flavor::game_versions(kind)?;
    let newer: Vec<String> = match all.iter().position(|v| v == current) {
        Some(i) => all[..i].to_vec(),
        None => all.iter().take(10).cloned().collect(),
    };
    Ok(newer)
}
