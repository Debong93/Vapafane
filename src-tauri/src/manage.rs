//! Fenster-Befehle für Server-Auswahl (hinzufügen, importieren, neu) und Einstellungen.

use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use tauri::AppHandle;

use crate::archive;
use crate::autostart;
use crate::config::{self, Config};
use crate::flavor::{self, Kind};
use crate::gui::{Cmd, blocking, run_task};
use crate::launcher;
use crate::ops::server_state;
use crate::procs;
use crate::server;
use crate::util::{self, Error, Result, err};

fn state_of(cfg: &Config) -> String {
    server_state(cfg)
        .get("state")
        .and_then(|s| s.as_str())
        .unwrap_or("stopped")
        .to_string()
}

fn is_stopped(cfg: &Config) -> bool {
    matches!(state_of(cfg).as_str(), "stopped" | "failed" | "none")
}

// ---------------------------------------------------------------- Server-Auswahl

/// Anderen Server aktiv machen – nur wenn der jetzige gestoppt ist.
#[tauri::command]
pub async fn select_server(path: String) -> Cmd<()> {
    blocking(move || {
        let mut cfg = Config::load();
        if !is_stopped(&cfg) {
            return err(crate::sprache::t!("err-umschalten-laeuft"));
        }
        if crate::ops::current_job().is_some() {
            return err(crate::sprache::t!("err-aufgabe-laeuft"));
        }
        let p = PathBuf::from(&path);
        if !p.is_dir() {
            return err(crate::sprache::t!("err-ordner-fehlt", ordner = path.to_string()));
        }
        cfg.set_active(Some(&p))
    })
    .await
}

/// Ordner ansehen, bevor er hinzugefügt wird.
#[tauri::command]
pub async fn inspect_folder(path: String) -> Cmd<Value> {
    blocking(move || {
        let root = server::find_server_root(Path::new(&path));
        let info = server::detect_server(&root);
        let has_props = root.join("server.properties").is_file();
        if info.kind == "unbekannt" && !has_props {
            return err(crate::sprache::t!("err-kein-server-ordner", ordner = root.display().to_string()));
        }
        let cfg = Config::load();
        if cfg.entry(&root).is_some() {
            return err(crate::sprache::t!("err-ordner-doppelt"));
        }
        Ok(json!({
            "root": root,
            "name": root.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
            "info": info,
            "eula": server::eula_accepted(&root),
        }))
    })
    .await
}

/// Server-Ordner in die Liste aufnehmen (RCON wird eingeschaltet). Aktiv machen, wenn möglich.
#[tauri::command]
pub async fn add_server(path: String, name: String, eula: bool) -> Cmd<Value> {
    blocking(move || {
        let name = name.trim().to_string();
        if name.is_empty() {
            return err(crate::sprache::t!("err-name-fehlt"));
        }
        let root = util::canonical(Path::new(&path));
        let (_info, notes) = server::prepare_server(&root, eula && !server::eula_accepted(&root))?;
        let mut cfg = Config::load();
        cfg.add_server(&name, &root)?;
        let activated = is_stopped(&cfg) && crate::ops::current_job().is_none();
        if activated {
            cfg.set_active(Some(&root))?;
        }
        Ok(json!({"notes": notes, "activated": activated, "path": root}))
    })
    .await
}

#[tauri::command]
pub fn import_server(app: AppHandle, archive: String, name: String, target: Option<String>) -> Cmd<()> {
    run_task(&app, &crate::sprache::t!("task-import", name = name.to_string()), move |progress| {
        let target = match target.filter(|t| !t.trim().is_empty()) {
            Some(t) => PathBuf::from(t),
            None => default_dir(&name)?,
        };
        let path = archive::import_archive_to(Path::new(&archive), &target, progress)?;
        Ok(json!({"path": path, "eula": server::eula_accepted(&path)}))
    })
    .map_err(|e| e.0)
}

fn default_dir(name: &str) -> Result<PathBuf> {
    let base = config::servers_dir();
    fs::create_dir_all(&base)?;
    Ok(util::unique_dir(&base, &util::safe_name(name)))
}

/// Vorschlag für den Server-Ordner: `MC_Servers/<Name>` im Benutzerordner.
#[tauri::command]
pub fn suggest_dir(name: String) -> Cmd<String> {
    let base = config::servers_dir();
    Ok(util::unique_dir(&base, &util::safe_name(&name))
        .to_string_lossy()
        .into_owned())
}

/// Ist der gewählte Ordner frei oder leer? Sonst Vorschlag für einen Unterordner.
#[tauri::command]
pub fn check_dir(path: String, name: String) -> Cmd<Value> {
    let p = PathBuf::from(&path);
    let state = flavor::target_state(&p);
    let suggestion = (state == "nonempty").then(|| util::unique_dir(&p, &util::safe_name(&name)));
    Ok(json!({"state": state, "suggestion": suggestion}))
}

#[tauri::command]
pub async fn server_versions(kind: String) -> Cmd<Vec<String>> {
    blocking(move || {
        let k = Kind::parse(&kind).ok_or_else(|| Error(crate::sprache::t!("err-typ-unbekannt")))?;
        flavor::game_versions(k)
    })
    .await
}

/// Empfohlene Leistungs-Mods für einen neuen Server (Typ und Minecraft-Version).
#[tauri::command]
pub async fn recommended_mods(kind: String, version: String) -> Cmd<Vec<Value>> {
    blocking(move || {
        let k = Kind::parse(&kind).ok_or_else(|| Error(crate::sprache::t!("err-typ-unbekannt")))?;
        crate::catalog::empfohlen(k, &version)
    })
    .await
}

/// Neuen Server anlegen – auf Wunsch gleich mit den empfohlenen Leistungs-Mods.
/// Klappt das bei einer Mod nicht, ist der Server trotzdem fertig (Liste in „mods“).
#[tauri::command]
pub fn create_server(
    app: AppHandle,
    kind: String,
    name: String,
    version: String,
    target: Option<String>,
    recommended: Option<bool>,
) -> Cmd<()> {
    let k = Kind::parse(&kind).ok_or_else(|| crate::sprache::t!("err-typ-unbekannt"))?;
    run_task(
        &app,
        &crate::sprache::t!("task-erstelle", typ = k.label(), name = name.to_string()),
        move |progress| {
            let target = match target.filter(|t| !t.trim().is_empty()) {
                Some(t) => PathBuf::from(t),
                None => default_dir(&name)?,
            };
            let mit_mods = recommended == Some(true) && matches!(k, Kind::Fabric | Kind::NeoForge);
            // Mit Mods: drei Viertel des Fortschritts für den Server, der Rest für die Mods
            let anteil = if mit_mods { 0.75 } else { 1.0 };
            let server_teil = |p: f64, msg: &str| progress(p * anteil, msg);
            let path = flavor::create_server(k, &name, &version, &target, &server_teil)?;
            let mut r = json!({"path": path});
            if mit_mods {
                let mod_teil = |p: f64, msg: &str| progress(anteil + p * (1.0 - anteil), msg);
                r["mods"] = crate::catalog::install_recommended(&path, k, &version, &mod_teil);
            }
            Ok(r)
        },
    )
    .map_err(|e| e.0)
}

/// Sprache: Wunsch ("system" oder Code) und was daraus geworden ist.
#[tauri::command]
pub fn get_language() -> Value {
    let cfg = Config::load();
    json!({
        "wish": cfg.language.unwrap_or_else(|| "system".into()),
        "resolved": crate::sprache::code(),
    })
}

/// Sprache setzen ("system" = Systemsprache). Gilt sofort für neue Meldungen;
/// die Oberfläche lädt sich danach selbst neu. Gibt den aufgelösten Code zurück.
#[tauri::command]
pub fn set_language(language: String) -> Cmd<String> {
    let wunsch = language.trim().to_ascii_lowercase();
    if wunsch != "system" && !vapafane_shared::sprache::SPRACHEN.iter().any(|(c, ..)| *c == wunsch) {
        return Err(crate::sprache::t!("err-sprache-unbekannt"));
    }
    let mut cfg = Config::load();
    cfg.language = (wunsch != "system").then_some(wunsch);
    cfg.save().map_err(|e| e.0)?;
    crate::sprache::init();
    Ok(crate::sprache::code().to_string())
}

pub fn java_hint(need: u32) -> String {
    if cfg!(windows) {
        crate::sprache::t!("java-hinweis-windows", version = need)
    } else {
        crate::sprache::t!("java-hinweis-linux", version = need)
    }
}

/// Eintrag aus der Liste entfernen. Die Dateien bleiben erhalten.
#[tauri::command]
pub async fn remove_server(path: String) -> Cmd<()> {
    blocking(move || {
        let mut cfg = Config::load();
        let p = util::canonical(Path::new(&path));
        let is_active = cfg.active_path().as_deref() == Some(p.as_path());
        if is_active && !is_stopped(&cfg) {
            return err(crate::sprache::t!("err-erst-stoppen"));
        }
        cfg.servers.retain(|s| util::canonical(Path::new(&s.path)) != p);
        cfg.save()?;
        if is_active {
            let next = cfg.servers.first().map(|s| PathBuf::from(&s.path));
            cfg.set_active(next.as_deref())?;
        }
        Ok(())
    })
    .await
}

// ---------------------------------------------------------------- Einstellungen

/// Einstellbare Schlüssel aus server.properties mit dem Minecraft-Standardwert,
/// der gilt, solange die Datei (vor dem ersten Start) den Schlüssel noch nicht hat.
const PROP_KEYS: &[(&str, &str)] = &[
    ("motd", "A Minecraft Server"),
    ("max-players", "20"),
    ("difficulty", "easy"),
    ("gamemode", "survival"),
    ("view-distance", "10"),
    ("simulation-distance", "10"),
    ("spawn-protection", "16"),
    ("server-port", "25565"),
    ("white-list", "false"),
];

/// Die einstellbaren Werte – fehlende mit dem Minecraft-Standard aufgefüllt.
fn settings_props(props: &BTreeMap<String, String>) -> BTreeMap<&'static str, String> {
    PROP_KEYS
        .iter()
        .map(|(k, d)| {
            let v = props.get(*k).filter(|v| !v.is_empty()).map_or(*d, String::as_str);
            (*k, v.to_string())
        })
        .collect()
}

#[tauri::command]
pub async fn get_settings() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let mut out = json!({
            "total_memory": procs::total_memory(),
            "notify": cfg.notify,
            "daily": cfg.daily_backup,
            "daily_time": cfg.daily_backup_time,
            "reach_check": cfg.reach_check,
            "window": autostart::window_autostart_enabled(),
            "linger": autostart::linger_enabled(),
            "systemd": autostart::systemd_usable(),
            "windows": cfg!(windows),
            "test_mode": config::test_home().is_some(),
        });
        let (server_boot, service_boot) = autostart::boot_state();
        out["autostart"] = json!(server_boot);
        out["service_boot"] = json!(service_boot);
        if let Some((active, enabled, timer)) = autostart::old_panel() {
            out["old_panel"] = json!({"active": active, "enabled": enabled, "timer": timer});
        }
        if let Some(dir) = cfg.active_path() {
            let props = server::read_properties(&dir);
            out["props"] = json!(settings_props(&props));
            out["ram_gb"] = json!(server::xmx_gb(&server::read_env(&dir)));
        }
        Ok(out)
    })
    .await
}

#[tauri::command]
pub async fn save_settings(props: BTreeMap<String, String>, ram_gb: u64) -> Cmd<()> {
    blocking(move || {
        let cfg = Config::load();
        let dir = cfg
            .active_path()
            .ok_or_else(|| Error(crate::sprache::t!("err-kein-server")))?;
        let props: BTreeMap<String, String> = props
            .into_iter()
            // leere Felder nicht schreiben – sonst startet Minecraft mit Fehlern
            .filter(|(k, v)| PROP_KEYS.iter().any(|(p, _)| p == k) && !v.is_empty())
            .collect();
        if let Some(port) = props.get("server-port") {
            let p: u32 = port.parse().map_err(|_| Error(crate::sprache::t!("err-port-zahl")))?;
            if !(1024..=65535).contains(&p) {
                return err(crate::sprache::t!("err-port-bereich"));
            }
        }
        server::write_properties(&dir, &props)?;
        let total_gb = procs::total_memory() / 1024 / 1024 / 1024;
        let ram = ram_gb.clamp(1, total_gb.max(2));
        server::write_env(&dir, &[("MC_XMX", format!("{ram}G"))])?;
        // Startspeicher nie größer als das Limit
        let env = server::read_env(&dir);
        let xms = env
            .get("MC_XMS")
            .and_then(|x| x.trim_end_matches(['G', 'g']).parse::<u64>().ok());
        if xms.map(|x| x > ram).unwrap_or(false) {
            server::write_env(&dir, &[("MC_XMS", format!("{ram}G"))])?;
        }
        Ok(())
    })
    .await
}

fn apply_boot(cfg: &Config) -> Result<()> {
    if config::test_home().is_some() {
        return Ok(()); // Testmodus: nichts am System ändern
    }
    autostart::apply_boot(cfg.autostart_server, cfg.daily_backup || cfg.autostart_server)
}

#[tauri::command]
pub async fn set_autostart(on: bool) -> Cmd<()> {
    blocking(move || {
        let mut cfg = Config::load();
        cfg.autostart_server = on;
        apply_boot(&cfg)?;
        cfg.save()
    })
    .await
}

#[tauri::command]
pub async fn enable_linger() -> Cmd<()> {
    blocking(autostart::enable_linger).await
}

#[tauri::command]
pub async fn set_window_autostart(on: bool) -> Cmd<()> {
    blocking(move || autostart::set_window_autostart(on)).await
}

#[tauri::command]
pub async fn set_daily_backup(on: bool) -> Cmd<()> {
    blocking(move || {
        let mut cfg = Config::load();
        cfg.daily_backup = on;
        cfg.save()?;
        apply_boot(&cfg)?;
        if on {
            // Damit das Backup auch heute schon läuft
            launcher::ensure_service()?;
        }
        Ok(())
    })
    .await
}

/// Uhrzeit für das tägliche Backup („HH:MM“).
#[tauri::command]
pub fn set_daily_time(time: String) -> Cmd<String> {
    let t = chrono::NaiveTime::parse_from_str(time.trim(), "%H:%M")
        .map_err(|_| crate::sprache::t!("err-uhrzeit"))?;
    let mut cfg = Config::load();
    cfg.daily_backup_time = t.format("%H:%M").to_string();
    cfg.save().map_err(|e| e.0)?;
    Ok(cfg.daily_backup_time)
}

#[tauri::command]
pub fn set_reach_check(on: bool) -> Cmd<()> {
    let mut cfg = Config::load();
    cfg.reach_check = on;
    cfg.save().map_err(|e| e.0)
}

/// Erreichbarkeit von außen prüfen (Reiter „Erreichbarkeit“).
#[tauri::command]
pub async fn reach_check() -> Cmd<crate::reach::Report> {
    blocking(|| {
        let cfg = Config::load();
        let dir = cfg
            .active_path()
            .ok_or_else(|| Error(crate::sprache::t!("err-noch-kein-server")))?;
        crate::reach::check(&dir)
    })
    .await
}

/// Letztes Ergebnis und ob beim Wechsel in den Server-Bereich geprüft werden soll.
#[tauri::command]
pub fn reach_last() -> Value {
    let cfg = Config::load();
    let last = crate::reach::last().filter(|r| {
        cfg.active_path()
            .is_some_and(|d| crate::reach::server_port(&d) == r.port)
    });
    json!({"report": last, "auto": crate::reach::should_check_on_start(&cfg), "server": cfg.active_path().is_some()})
}

#[tauri::command]
pub fn set_notify(on: bool) -> Cmd<()> {
    let mut cfg = Config::load();
    cfg.notify = on;
    cfg.save().map_err(|e| e.0)
}

#[tauri::command]
pub async fn disable_old_panel() -> Cmd<()> {
    blocking(autostart::disable_old_panel).await
}

// ---------------------------------------------------------------- CurseForge-Schlüssel

#[tauri::command]
pub fn cf_status() -> Value {
    json!({"set": crate::curseforge::key().is_some(), "masked": crate::curseforge::masked()})
}

/// Schlüssel ausprobieren. Ohne Angabe wird der gespeicherte getestet.
#[tauri::command]
pub async fn cf_key_test(key: Option<String>) -> Cmd<()> {
    blocking(move || {
        let k = key
            .filter(|k| !k.trim().is_empty())
            .or_else(crate::curseforge::key)
            .ok_or_else(|| Error(crate::sprache::t!("cf-err-leer")))?;
        crate::curseforge::test_key(&k)
    })
    .await
}

#[tauri::command]
pub fn cf_key_save(key: String) -> Cmd<()> {
    crate::curseforge::save_key(&key).map_err(|e| e.0)
}

#[tauri::command]
pub fn cf_key_delete() -> Cmd<()> {
    crate::curseforge::delete_key().map_err(|e| e.0)
}

/// Webseite im Browser öffnen – nur fest hinterlegte Ziele bzw. Seiten auf curseforge.com/modrinth.com.
#[tauri::command]
pub fn open_link(app: AppHandle, target: String) -> Cmd<()> {
    use tauri_plugin_opener::OpenerExt;
    let url = match target.as_str() {
        "cf-apply" => crate::curseforge::APPLY_URL.to_string(),
        "cf-docs" => crate::curseforge::DOCS_URL.to_string(),
        "cf-console" => crate::curseforge::CONSOLE_URL.to_string(),
        "cf-terms" => crate::curseforge::TERMS_URL.to_string(),
        "issues" => "https://github.com/Debong93/Vapafane/issues".to_string(),
        "repo" => "https://github.com/Debong93/Vapafane".to_string(),
        "launcher-download" => crate::client::LAUNCHER_DOWNLOAD.to_string(),
        "modrinth-download" => "https://modrinth.com/app".to_string(),
        "portforward" => crate::reach::GUIDE_URL.to_string(),
        u if u.starts_with("https://www.curseforge.com/") || u.starts_with("https://modrinth.com/") => u.to_string(),
        _ => return Err(crate::sprache::t!("err-link-verboten")),
    };
    app.opener().open_url(url, None::<&str>).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fehlende_einstellungen_bekommen_standardwerte() {
        let mut props = BTreeMap::new();
        props.insert("motd".to_string(), "E2E".to_string());
        props.insert("max-players".to_string(), String::new());
        let s = settings_props(&props);
        assert_eq!(s["motd"], "E2E");
        assert_eq!(s["max-players"], "20");
        assert_eq!(s["difficulty"], "easy");
        assert_eq!(s.len(), PROP_KEYS.len());
    }
}
