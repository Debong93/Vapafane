//! Fenster-Befehle für „Freunde einladen“ (Server) und „Einladung einlösen“ (Client).

use serde_json::{Value, json};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::client;
use crate::config::Config;
use crate::gui::{Cmd, active_dir, blocking, run_task};
use crate::invite::{self, Invite};
use crate::transfer::{self, Take};
use crate::util::{self, Result};

// ---------------------------------------------------------------- Server: einladen

#[tauri::command]
pub async fn invite_defaults() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        Ok(json!({"lan": invite::lan_address(&dir), "name": cfg.name_of(&dir)}))
    })
    .await
}

#[tauri::command]
pub async fn invite_public_address() -> Cmd<String> {
    blocking(|| {
        let cfg = Config::load();
        invite::public_address(&active_dir(&cfg)?)
    })
    .await
}

fn built_info(b: &invite::Built) -> Value {
    json!({
        "mods": b.invite.mods.len(),
        "optional": b.invite.mods.iter().filter(|m| m.optional).count(),
        "server_only": b.server_only,
        "files": b.files.iter().map(|(t, _)| t.clone()).collect::<Vec<_>>(),
        "loader": b.invite.loader,
        "mc": b.invite.mc,
        "address6": b.invite.address6,
        "only_v6": b.invite.only_v6,
    })
}

#[tauri::command]
pub async fn invite_code(address: String) -> Cmd<Value> {
    blocking(move || {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let b = invite::build(&dir, &address)?;
        let code = invite::encode(&b.invite)?;
        let mut out = built_info(&b);
        out["code"] = json!(code);
        Ok(out)
    })
    .await
}

#[tauri::command]
pub async fn invite_file(app: AppHandle, address: String) -> Cmd<Option<Value>> {
    blocking(move || {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let name = cfg.name_of(&dir);
        let b = invite::build(&dir, &address)?;
        let Some(dest) = app
            .dialog()
            .file()
            .set_title("Einladungs-Datei speichern")
            .set_directory(crate::config::home())
            .set_file_name(format!("Einladung_{}.vapafane", util::safe_name(&name)))
            .add_filter("Vapafane-Einladung", &["vapafane"])
            .blocking_save_file()
            .and_then(|p| p.into_path().ok())
        else {
            return Ok(None);
        };
        let inv = invite::write_file(&b, &dest)?;
        let mut out = built_info(&b);
        out["path"] = json!(dest);
        out["mods"] = json!(inv.mods.len());
        Ok(Some(out))
    })
    .await
}

// ---------------------------------------------------------------- Client: einlösen

/// Code oder Dateipfad → Einladung (+ Pfad der Datei).
fn load(source: &str) -> Result<(Invite, Option<PathBuf>)> {
    let p = Path::new(source.trim());
    if source.trim().ends_with(".vapafane") && p.is_file() {
        return Ok((invite::read_file(p)?, Some(p.to_path_buf())));
    }
    Ok((invite::decode(source)?, None))
}

#[tauri::command]
pub async fn client_preview(source: String) -> Cmd<Value> {
    blocking(move || {
        let (inv, zip) = load(&source)?;
        let mut v = client::preview(&inv, zip.as_deref())?;
        v["source"] = json!(source);
        Ok(v)
    })
    .await
}

#[tauri::command]
pub async fn client_pick_file(app: AppHandle) -> Cmd<Option<String>> {
    blocking(move || {
        Ok(app
            .dialog()
            .file()
            .set_title("Einladungs-Datei öffnen")
            .add_filter("Vapafane-Einladung", &["vapafane"])
            .blocking_pick_file()
            .and_then(|p| p.into_path().ok())
            .map(|p| p.to_string_lossy().into_owned()))
    })
    .await
}

/// `take`: Einstellungen aus dieser Quelle übernehmen. `update`: vorhandene Instanz in der
/// Modrinth-App aktualisieren statt eine neue anzulegen.
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn client_install(
    app: AppHandle,
    source: String,
    target: Option<String>,
    selected: Vec<String>,
    destination: Option<String>,
    take: Option<Take>,
    update: Option<bool>,
) -> Cmd<()> {
    let dest = destination.unwrap_or_else(|| "launcher".into());
    let take = take.filter(|t| !t.parts.is_empty() && !t.dir.as_os_str().is_empty());
    run_task(&app, "Client einrichten", move |progress| {
        let (inv, zip) = load(&source)?;
        let selected: HashSet<String> = selected.into_iter().collect();
        // Gewähltes Ziel merken
        let mut cfg = Config::load();
        cfg.client_target = Some(dest.clone());
        cfg.save()?;
        let choices = client::Choices { selected, take: take.as_ref() };
        if dest == "modrinth" {
            return client::install_mrpack(&inv, zip.as_deref(), &choices, update.unwrap_or(true), true, progress);
        }
        let target = target.filter(|t| !t.trim().is_empty()).map(PathBuf::from);
        client::install(&inv, zip.as_deref(), target.as_deref(), &choices, progress)
    })
    .map_err(|e| e.0)
}

/// Gespeicherte Einladung eines eingerichteten Clients als Code (für „Aktualisieren“).
#[tauri::command]
pub async fn client_saved_code(dir: String) -> Cmd<String> {
    blocking(move || invite::encode(&client::invite_of(Path::new(&dir))?)).await
}

/// Selbst gewählter Ordner als Quelle für „Einstellungen übernehmen“.
#[tauri::command]
pub async fn client_source(dir: String) -> Cmd<Value> {
    blocking(move || transfer::describe(Path::new(&dir))).await
}

/// Mögliche Quellen für „Einstellungen übernehmen“ (ohne den Ordner, in den eingerichtet wird).
#[tauri::command]
pub async fn client_sources(exclude: Option<String>) -> Cmd<Vec<Value>> {
    blocking(move || Ok(transfer::sources(exclude.as_deref().map(Path::new)))).await
}

#[tauri::command]
pub async fn client_list() -> Cmd<Vec<Value>> {
    blocking(|| Ok(client::list())).await
}

/// Status eines eingerichteten Clients: Server erreichbar? Version noch gleich?
#[tauri::command]
pub async fn client_status(dir: String) -> Cmd<Value> {
    blocking(move || {
        let inv = client::invite_of(Path::new(&dir))?;
        let (_, p, hint) = client::best_address(&inv);
        Ok(if p["online"] == true {
            json!({"online": true, "version_ok": client::version_matches(&p, &inv.mc), "ping": p})
        } else {
            json!({"online": false, "error": p["error"], "hint": hint})
        })
    })
    .await
}

/// Client-Wächter: prüft alle eingerichteten Server (das Fenster ruft ihn alle 20 s auf,
/// solange die Client-Ansicht sichtbar ist).
#[tauri::command]
pub async fn client_watch() -> Cmd<Vec<Value>> {
    blocking(|| {
        let checks: Vec<_> = client::list()
            .iter()
            .filter_map(|c| c.get("dir").and_then(|d| d.as_str()).map(str::to_string))
            .map(|dir| {
                std::thread::spawn(move || {
                    let inv = client::invite_of(Path::new(&dir)).ok()?;
                    let (_, p, hint) = client::best_address(&inv);
                    Some(if p["online"] == true {
                        json!({"dir": dir, "online": true, "version_ok": client::version_matches(&p, &inv.mc), "ping": p})
                    } else {
                        json!({"dir": dir, "online": false, "error": p["error"], "hint": hint})
                    })
                })
            })
            .collect();
        Ok(checks.into_iter().filter_map(|t| t.join().ok().flatten()).collect())
    })
    .await
}

/// Erreichbarkeit des Servers einer gerade angezeigten Einladung (Code oder Datei).
#[tauri::command]
pub async fn client_preview_ping(source: String) -> Cmd<Value> {
    blocking(move || {
        let (inv, _) = load(&source)?;
        let (address, p, hint) = client::best_address(&inv);
        Ok(json!({"ping": p, "version_ok": client::version_matches(&p, &inv.mc), "address": address, "hint": hint}))
    })
    .await
}

/// Einstellungen für den Client-Bereich.
#[tauri::command]
pub fn client_settings() -> Value {
    json!({
        "target": client_info()["target"],
        "version": env!("CARGO_PKG_VERSION"),
    })
}

#[tauri::command]
pub fn set_client_target(target: String) -> Cmd<()> {
    if !matches!(target.as_str(), "launcher" | "modrinth") {
        return Err("Unbekanntes Ziel".into());
    }
    let mut cfg = Config::load();
    cfg.client_target = Some(target);
    cfg.save().map_err(|e| e.0)
}

/// Ordner, in dem Vapafane die eingerichteten Server ablegt.
#[tauri::command]
pub fn client_open_base(app: AppHandle) -> Cmd<()> {
    use tauri_plugin_opener::OpenerExt;
    let base = client::clients_base();
    let _ = std::fs::create_dir_all(&base);
    app.opener()
        .open_path(base.to_string_lossy(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn client_remove(dir: String) -> Cmd<()> {
    blocking(move || client::remove(Path::new(&dir))).await
}

#[tauri::command]
pub fn client_open_folder(app: AppHandle, dir: String) -> Cmd<()> {
    use tauri_plugin_opener::OpenerExt;
    let p = PathBuf::from(&dir);
    if !p.join("vapafane-client.json").is_file() {
        return Err("Das ist kein Vapafane-Client.".into());
    }
    app.opener().open_path(dir, None::<&str>).map_err(|e| e.to_string())
}

/// „Spielen“: Launcher bzw. Modrinth-App öffnen – je nachdem, wofür der Client eingerichtet ist.
#[tauri::command]
pub async fn client_play(target: Option<String>) -> Cmd<()> {
    blocking(move || {
        if target.as_deref() == Some("modrinth") {
            client::open_modrinth_app()
        } else {
            client::open_launcher()
        }
    })
    .await
}

#[tauri::command]
pub fn client_info() -> Value {
    let launcher = !client::launcher_dirs().is_empty();
    let modrinth = client::modrinth_app().is_some();
    let default = Config::load().client_target.unwrap_or_else(|| {
        if !launcher && modrinth {
            "modrinth".into()
        } else {
            "launcher".into()
        }
    });
    json!({
        "modrinth": modrinth,
        "target": default,
        "launcher": launcher,
        "profiles": !client::profile_files().is_empty(),
        "base": client::clients_base(),
    })
}
