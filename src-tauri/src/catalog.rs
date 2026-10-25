//! „Mods hinzufügen“: Modrinth durchsuchen und Mods, Plugins oder Datapacks samt
//! Pflicht-Abhängigkeiten installieren – passend zu Typ und Version des Servers.

use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::archive::Progress;
use crate::curseforge;
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
    kind: Kind,
    datapacks: bool,
    mc: String,
    loaders: Vec<&'static str>,
    project_type: &'static str,
    /// Zielordner relativ zum Server
    sub: String,
}

fn target(dir: &Path, what: What) -> Result<Target> {
    let info = server::detect_server(dir);
    let kind = Kind::parse(&info.kind).ok_or_else(|| Error(crate::sprache::t!("kat-err-typ")))?;
    if info.version.is_empty() {
        return err(crate::sprache::t!("err-mc-unbekannt"));
    }
    let datapacks = what == What::Datapacks || kind == Kind::Vanilla;
    Ok(if datapacks {
        Target {
            kind,
            datapacks: true,
            mc: info.version,
            loaders: vec!["datapack"],
            project_type: "datapack",
            sub: format!("{}/datapacks", updater::level_name(dir)),
        }
    } else {
        Target {
            kind,
            datapacks: false,
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
    let mut out: HashSet<String> = known
        .values()
        .filter_map(|v| v.get("project_id").and_then(|p| p.as_str()).map(str::to_string))
        .collect();
    // Von CurseForge installierte (Herkunft gemerkt) als „cf:<id>“
    for v in mods::sources(dir).values() {
        if v.get("source").and_then(|s| s.as_str()) == Some("curseforge")
            && let Some(id) = v.get("id").and_then(|i| i.as_u64())
        {
            out.insert(format!("cf:{id}"));
        }
    }
    Ok(out)
}

/// Modrinth durchsuchen. Ohne Suchbegriff: die beliebtesten.
pub fn search(dir: &Path, what: What, query: &str, offset: u32, cf_offset: Option<u32>) -> Result<Value> {
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
                "page": h.get("slug").and_then(|s| s.as_str()).map(|s| {
                    let typ = h.get("project_type").and_then(|t| t.as_str()).unwrap_or("mod");
                    format!("https://modrinth.com/{typ}/{s}")
                }),
                "title": h.get("title"),
                "description": h.get("description"),
                "icon": h.get("icon_url"),
                "downloads": h.get("downloads"),
                "author": h.get("author"),
                // Brauchen die Spieler die Mod auch selbst?
                "client_required": client == "required",
                "source": "modrinth",
            })
        })
        .collect();
    let mut out = json!({
        "hits": hits,
        "total": v.get("total_hits"),
        "offset": offset,
        "mc": t.mc,
        "type": t.project_type,
        "curseforge": curseforge::key().is_some(),
    });
    // Zusätzlich CurseForge – nur Projekte, die es auf Modrinth nicht gibt
    if let (Some(cf_index), true) = (cf_offset, curseforge::key().is_some()) {
        match cf_only(&t, query, cf_index, &out["hits"]) {
            Ok((cf_hits, cf_total)) => {
                out["cf_hits"] = json!(cf_hits);
                out["cf_total"] = json!(cf_total);
                out["cf_offset"] = json!(cf_index + 20);
            }
            Err(e) => out["cf_error"] = json!(e.0),
        }
    }
    Ok(out)
}

fn norm(s: &str) -> String {
    s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect()
}

/// CurseForge-Treffer ohne die, die es (wahrscheinlich) auch auf Modrinth gibt:
/// gleicher Slug, gleicher Name oder gleiche Datei (SHA1).
fn cf_only(t: &Target, query: &str, index: u32, modrinth_hits: &Value) -> Result<(Vec<Value>, u64)> {
    let (mods, total) = curseforge::search(t.kind, t.datapacks, &t.mc, query, index)?;
    let mut seen_names: HashSet<String> = modrinth_hits
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|h| [h["slug"].as_str().map(norm), h["title"].as_str().map(norm)])
        .flatten()
        .collect();
    // Slugs auf Modrinth nachschlagen (eine Anfrage für alle)
    let slugs: Vec<String> = mods
        .iter()
        .filter_map(|m| m.get("slug").and_then(|s| s.as_str()).map(str::to_string))
        .collect();
    if !slugs.is_empty()
        && let Ok(v) = net::get_json_query(&format!("{API}/projects"), &[("ids", serde_json::to_string(&slugs)?)])
    {
        for p in v.as_array().into_iter().flatten() {
            for key in ["slug", "title"] {
                if let Some(x) = p.get(key).and_then(|x| x.as_str()) {
                    seen_names.insert(norm(x));
                }
            }
        }
    }
    // Dateien per SHA1 auf Modrinth suchen
    let mut sha_to_mod: Vec<(String, u64)> = vec![];
    for m in &mods {
        let id = m.get("id").and_then(|i| i.as_u64()).unwrap_or(0);
        for f in m
            .get("latestFiles")
            .and_then(|f| f.as_array())
            .into_iter()
            .flatten()
            .take(3)
        {
            if let Some(h) = curseforge::file_sha1(f) {
                sha_to_mod.push((h, id));
            }
        }
    }
    let hashes: Vec<String> = sha_to_mod.iter().map(|(h, _)| h.clone()).collect();
    let on_modrinth: HashSet<u64> = updater::version_files(&hashes)
        .map(|known| {
            sha_to_mod
                .iter()
                .filter(|(h, _)| known.contains_key(h))
                .map(|(_, id)| *id)
                .collect()
        })
        .unwrap_or_default();
    let hits = mods
        .into_iter()
        .filter(|m| {
            let id = m.get("id").and_then(|i| i.as_u64()).unwrap_or(0);
            let slug = m.get("slug").and_then(|s| s.as_str()).map(norm).unwrap_or_default();
            let name = m.get("name").and_then(|s| s.as_str()).map(norm).unwrap_or_default();
            !on_modrinth.contains(&id) && !seen_names.contains(&slug) && !seen_names.contains(&name)
        })
        .map(|m| {
            json!({
                "id": format!("cf:{}", m.get("id").and_then(|i| i.as_u64()).unwrap_or(0)),
                "slug": m.get("slug"),
                "title": m.get("name"),
                "description": m.get("summary"),
                "icon": m.pointer("/logo/thumbnailUrl"),
                "downloads": m.get("downloadCount"),
                "author": m.pointer("/authors/0/name"),
                "client_required": Value::Null,
                "source": "curseforge",
                "page": m.pointer("/links/websiteUrl"),
            })
        })
        .collect();
    Ok((hits, total))
}

/// Projekt-ID oder Slug → Projekt-ID und Titel.
fn project_info(id_or_slug: &str) -> Result<(String, String)> {
    let p = net::get_json(&format!("{API}/project/{id_or_slug}"))
        .map_err(|_| Error(crate::sprache::t!("kat-err-projekt", projekt = id_or_slug.to_string())))?;
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
    if let Some(id) = project.strip_prefix("cf:") {
        let id: u64 = id.parse().map_err(|_| Error(crate::sprache::t!("cf-err-id")))?;
        return install_cf(dir, &t, id, progress);
    }
    let mut have = installed_projects(dir)?;
    let (root_id, root_title) = project_info(project)?;
    if have.contains(&root_id) {
        return err(crate::sprache::t!("kat-err-schon-da", name = root_title.to_string()));
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
    let mut sources: Vec<(String, Value)> = vec![];
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
                &crate::sprache::t!("fort-suche", name = title.clone()),
            );
            let Some(version) = updater::newest_for_project(&pid, &t.loaders, &t.mc)? else {
                if is_root {
                    return err(crate::sprache::t!(
                        "kat-err-keine-version",
                        name = title.clone(),
                        mc = t.mc.clone(),
                        loader = t.loaders.join("/")
                    ));
                }
                missing.push(title);
                continue;
            };
            let file = updater::primary_file(&version).ok_or_else(|| Error(crate::sprache::t!("kat-err-keine-datei", name = title.to_string())))?;
            let name = updater::safe_filename(&file.filename)?;
            if dest_dir.join(&name).exists() || dest_dir.join(format!("{name}{}", mods::DISABLED)).exists() {
                skipped.push(title);
                continue;
            }
            progress(
                0.1 + 0.6 * (planned.len() as f64 / (planned.len() + queue.len() + 1) as f64),
                &crate::sprache::t!("fort-lade", name = title.to_string()),
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
            sources.push((name.clone(), json!({"source": "modrinth", "id": pid, "dep": !is_root})));
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
        progress(0.9, &crate::sprache::t!("fort-sortiere"));
        for (_, _, name) in &planned {
            fs::rename(staging.join(name), dest_dir.join(name))?;
        }
        for (name, src) in &sources {
            mods::set_source(dir, &format!("{}/{name}", t.sub), Some(src.clone()))?;
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

/// Von CurseForge installieren. Erlaubt der Autor keine Downloads über andere
/// Programme, kommt stattdessen die Seite für den Download im Browser zurück.
fn install_cf(dir: &Path, t: &Target, root: u64, progress: Progress) -> Result<Value> {
    let have = installed_projects(dir)?;
    if have.contains(&format!("cf:{root}")) {
        return err(crate::sprache::t!("kat-err-installiert"));
    }
    let dest_dir = dir.join(&t.sub);
    fs::create_dir_all(&dest_dir)?;
    let staging = dir.join(crate::config::STAGING_NAME).join("neu");
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging)?;
    let mut queue = vec![(root, true)];
    let mut seen = HashSet::new();
    // (Titel, Version, Datei, Mod-ID, Datei-ID, als Abhängigkeit?)
    let mut planned: Vec<(String, String, String, u64, u64, bool)> = vec![];
    let (mut skipped, mut missing, mut manual) = (vec![], vec![], vec![]);
    let mut root_title = String::new();
    let res = (|| -> Result<()> {
        while let Some((id, is_root)) = queue.pop() {
            if !seen.insert(id) {
                continue;
            }
            let m = curseforge::get_mod(id)?;
            let title = m.get("name").and_then(|n| n.as_str()).unwrap_or("?").to_string();
            if is_root {
                root_title = title.clone();
            } else if have.contains(&format!("cf:{id}")) {
                skipped.push(title);
                continue;
            }
            progress(0.2, &crate::sprache::t!("fort-suche-cf", name = title.clone()));
            let Some(file) = curseforge::best_file(id, t.kind, t.datapacks, &t.mc)? else {
                if is_root {
                    return err(crate::sprache::t!(
                        "kat-err-cf-version",
                        name = title.clone(),
                        mc = t.mc.clone()
                    ));
                }
                missing.push(title);
                continue;
            };
            let name = updater::safe_filename(file.get("fileName").and_then(|f| f.as_str()).unwrap_or(""))?;
            if dest_dir.join(&name).exists() || dest_dir.join(format!("{name}{}", mods::DISABLED)).exists() {
                skipped.push(title);
                continue;
            }
            let Some(url) = curseforge::download_url(&file) else {
                // Der Autor erlaubt keine Downloads über andere Programme
                manual.push(json!({"title": title, "page": curseforge::file_page(&m, &file), "file": name}));
                continue;
            };
            progress(0.5, &crate::sprache::t!("fort-lade-cf", name = title.to_string()));
            net::download(
                &url,
                &staging.join(&name),
                net::Expect {
                    sha1: curseforge::file_sha1(&file).as_deref(),
                    ..Default::default()
                },
                &|_| {},
            )?;
            let fid = file.get("id").and_then(|i| i.as_u64()).unwrap_or(0);
            let shown = file
                .get("displayName")
                .and_then(|d| d.as_str())
                .unwrap_or(&name)
                .to_string();
            planned.push((title, shown, name, id, fid, !is_root));
            for dep in curseforge::required_deps(&file) {
                queue.push((dep, false));
            }
        }
        progress(0.9, &crate::sprache::t!("fort-sortiere"));
        for (_, _, name, mid, fid, dep) in &planned {
            fs::rename(staging.join(name), dest_dir.join(name))?;
            mods::set_source(
                dir,
                &format!("{}/{name}", t.sub),
                Some(json!({"source": "curseforge", "id": mid, "file": fid, "dep": dep})),
            )?;
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(dir.join(crate::config::STAGING_NAME));
    res?;
    Ok(json!({
        "title": root_title,
        "folder": t.sub,
        "source": "curseforge",
        "installed": planned
            .iter()
            .map(|(t, v, f, _, _, _)| json!({"title": t, "version": v, "file": f}))
            .collect::<Vec<_>>(),
        "skipped": skipped,
        "missing": missing,
        "manual": manual,
    }))
}
