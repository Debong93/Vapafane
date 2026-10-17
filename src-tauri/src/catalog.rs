//! „Mods hinzufügen“: Modrinth durchsuchen und Mods, Plugins oder Datapacks samt
//! Pflicht-Abhängigkeiten installieren – passend zu Typ und Version des Servers.

use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::archive::Progress;
use crate::flavor::Kind;
use crate::mods;
use crate::net;
use crate::server;
use crate::updater;
use crate::util::{Error, Result, err};

const API: &str = "https://api.modrinth.com/v2";

/// Wonach gesucht wird: Mods/Plugins des Servers oder Datapacks für die Welt.
#[derive(Clone, Copy, PartialEq)]
pub enum What {
    Mods,
    Datapacks,
}

struct Target {
    mc: String,
    loaders: Vec<&'static str>,
    project_type: &'static str,
    /// Zielordner relativ zum Server
    sub: String,
}

fn target(dir: &Path, what: What) -> Result<Target> {
    let info = server::detect_server(dir);
    let kind = Kind::parse(&info.kind)
        .ok_or_else(|| Error("Für diesen Server-Typ gibt es „Mods hinzufügen“ nicht.".into()))?;
    if info.version.is_empty() {
        return err("Die Minecraft-Version des Servers ist unbekannt (einmal starten hilft).");
    }
    let datapacks = what == What::Datapacks || kind == Kind::Vanilla;
    Ok(if datapacks {
        Target {
            mc: info.version,
            loaders: vec!["datapack"],
            project_type: "datapack",
            sub: format!("{}/datapacks", updater::level_name(dir)),
        }
    } else {
        Target {
            mc: info.version,
            loaders: kind.loaders().to_vec(),
            project_type: if kind == Kind::Paper { "plugin" } else { "mod" },
            sub: kind.mod_dir().unwrap_or("mods").into(),
        }
    })
}

/// Projekte (Modrinth-IDs), die schon im Server liegen – ein- oder ausgeschaltet.
pub fn installed_projects(dir: &Path) -> Result<HashSet<String>> {
    let mut files: Vec<PathBuf> = vec![];
    let level = updater::level_name(dir);
    for sub in [crate::flavor::mod_dir_of(dir).to_string(), format!("{level}/datapacks")] {
        if let Ok(rd) = fs::read_dir(dir.join(&sub)) {
            for e in rd.filter_map(|e| e.ok()) {
                let n = e.file_name().to_string_lossy().to_lowercase();
                let n = n.strip_suffix(mods::DISABLED).unwrap_or(&n).to_string();
                if e.path().is_file() && (n.ends_with(".jar") || n.ends_with(".zip")) {
                    files.push(e.path());
                }
            }
        }
    }
    let hashes: Vec<String> = files.iter().filter_map(|p| net::sha1_file(p).ok()).collect();
    let known = updater::version_files(&hashes)?;
    Ok(known
        .values()
        .filter_map(|v| v.get("project_id").and_then(|p| p.as_str()).map(str::to_string))
        .collect())
}

/// Modrinth durchsuchen. Ohne Suchbegriff: die beliebtesten.
pub fn search(dir: &Path, what: What, query: &str, offset: u32) -> Result<Value> {
    let t = target(dir, what)?;
    let mut facets = vec![
        json!([format!("project_type:{}", t.project_type)]),
        json!([format!("versions:{}", t.mc)]),
        json!(["server_side!=unsupported"]),
    ];
    if t.project_type != "datapack" {
        facets.push(json!(
            t.loaders.iter().map(|l| format!("categories:{l}")).collect::<Vec<_>>()
        ));
    }
    let query = query.trim();
    let v = net::get_json_query(
        &format!("{API}/search"),
        &[
            ("query", query.to_string()),
            ("facets", serde_json::to_string(&facets)?),
            (
                "index",
                if query.is_empty() { "downloads" } else { "relevance" }.to_string(),
            ),
            ("limit", "20".to_string()),
            ("offset", offset.to_string()),
        ],
    )?;
    let hits: Vec<Value> = v
        .get("hits")
        .and_then(|h| h.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|h| {
            let client = h.get("client_side").and_then(|c| c.as_str()).unwrap_or("");
            json!({
                "id": h.get("project_id"),
                "slug": h.get("slug"),
                "title": h.get("title"),
                "description": h.get("description"),
                "icon": h.get("icon_url"),
                "downloads": h.get("downloads"),
                "author": h.get("author"),
                // Brauchen die Spieler die Mod auch selbst?
                "client_required": client == "required",
            })
        })
        .collect();
    Ok(json!({
        "hits": hits,
        "total": v.get("total_hits"),
        "offset": offset,
        "mc": t.mc,
        "type": t.project_type,
    }))
}

/// Projekt-ID oder Slug → Projekt-ID und Titel.
fn project_info(id_or_slug: &str) -> Result<(String, String)> {
    let p = net::get_json(&format!("{API}/project/{id_or_slug}"))
        .map_err(|_| Error(format!("Projekt „{id_or_slug}“ gibt es auf Modrinth nicht.")))?;
    let id = p.get("id").and_then(|i| i.as_str()).unwrap_or(id_or_slug).to_string();
    let title = p
        .get("title")
        .and_then(|t| t.as_str())
        .unwrap_or(id_or_slug)
        .to_string();
    Ok((id, title))
}

/// Installiert ein Projekt (neueste passende Version) und seine Pflicht-Abhängigkeiten.
/// Schon vorhandene Projekte werden übersprungen.
pub fn install(dir: &Path, what: What, project: &str, progress: Progress) -> Result<Value> {
    let t = target(dir, what)?;
    let mut have = installed_projects(dir)?;
    let (root_id, root_title) = project_info(project)?;
    if have.contains(&root_id) {
        return err(format!("„{root_title}“ ist schon installiert."));
    }
    let dest_dir = dir.join(&t.sub);
    fs::create_dir_all(&dest_dir)?;
    let staging = dir.join(crate::config::STAGING_NAME).join("neu");
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging)?;

    // Erst alles herausfinden und laden, dann gemeinsam einsortieren
    let mut queue = vec![(root_id.clone(), root_title.clone(), true)];
    let mut planned: Vec<(String, String, String)> = vec![]; // (Titel, Version, Dateiname)
    let mut skipped: Vec<String> = vec![];
    let mut missing: Vec<String> = vec![];
    let mut seen: HashSet<String> = HashSet::new();
    let res = (|| -> Result<()> {
        while let Some((pid, title, is_root)) = queue.pop() {
            if !seen.insert(pid.clone()) {
                continue;
            }
            if !is_root && have.contains(&pid) {
                skipped.push(title);
                continue;
            }
            progress(
                0.1 + 0.6 * (planned.len() as f64 / (planned.len() + queue.len() + 1) as f64),
                &format!("Suche {title} …"),
            );
            let Some(version) = updater::newest_for_project(&pid, &t.loaders, &t.mc)? else {
                if is_root {
                    return err(format!(
                        "„{title}“ gibt es nicht für Minecraft {} ({}).",
                        t.mc,
                        t.loaders.join("/")
                    ));
                }
                missing.push(title);
                continue;
            };
            let file = updater::primary_file(&version).ok_or_else(|| Error(format!("„{title}“ hat keine Datei.")))?;
            let name = updater::safe_filename(&file.filename)?;
            if dest_dir.join(&name).exists() || dest_dir.join(format!("{name}{}", mods::DISABLED)).exists() {
                skipped.push(title);
                continue;
            }
            progress(
                0.1 + 0.6 * (planned.len() as f64 / (planned.len() + queue.len() + 1) as f64),
                &format!("Lade {title} …"),
            );
            net::download(
                &file.url,
                &staging.join(&name),
                net::Expect {
                    sha512: file.sha512.as_deref(),
                    ..Default::default()
                },
                &|_| {},
            )?;
            let vnum = version
                .get("version_number")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            planned.push((title, vnum, name));
            have.insert(pid);
            for dep in version
                .get("dependencies")
                .and_then(|d| d.as_array())
                .cloned()
                .unwrap_or_default()
            {
                if dep.get("dependency_type").and_then(|d| d.as_str()) != Some("required") {
                    continue;
                }
                let dep_pid = match dep.get("project_id").and_then(|p| p.as_str()) {
                    Some(p) => p.to_string(),
                    None => match dep.get("version_id").and_then(|v| v.as_str()) {
                        Some(vid) => net::get_json(&format!("{API}/version/{vid}"))?
                            .get("project_id")
                            .and_then(|p| p.as_str())
                            .unwrap_or_default()
                            .to_string(),
                        None => continue,
                    },
                };
                if dep_pid.is_empty() || seen.contains(&dep_pid) {
                    continue;
                }
                let dep_title = project_info(&dep_pid)
                    .map(|(_, t)| t)
                    .unwrap_or_else(|_| dep_pid.clone());
                queue.push((dep_pid, dep_title, false));
            }
        }
        progress(0.9, "Sortiere ein …");
        for (_, _, name) in &planned {
            fs::rename(staging.join(name), dest_dir.join(name))?;
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(dir.join(crate::config::STAGING_NAME));
    res?;
    Ok(json!({
        "title": root_title,
        "folder": t.sub,
        "installed": planned.iter().map(|(t, v, f)| json!({"title": t, "version": v, "file": f})).collect::<Vec<_>>(),
        "skipped": skipped,
        "missing": missing,
    }))
}
