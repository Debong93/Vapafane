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

#[tauri::command]
pub fn client_install(
    app: AppHandle,
    source: String,
    target: Option<String>,
    selected: Vec<String>,
    destination: Option<String>,
) -> Cmd<()> {
    let dest = destination.unwrap_or_else(|| "launcher".into());
    run_task(&app, "Client einrichten", move |progress| {
        let (inv, zip) = load(&source)?;
        let selected: HashSet<String> = selected.into_iter().collect();
        // Gewähltes Ziel merken
        let mut cfg = Config::load();
        cfg.client_target = Some(dest.clone());
        cfg.save()?;
        if dest == "modrinth" {
            return client::install_mrpack(&inv, zip.as_deref(), &selected, true, progress);
        }
        let target = target.filter(|t| !t.trim().is_empty()).map(PathBuf::from);
        client::install(&inv, zip.as_deref(), target.as_deref(), &selected, progress)
    })
    .map_err(|e| e.0)
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
