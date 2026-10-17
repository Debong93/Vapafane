//! Fenster-Befehle für Server-Auswahl (hinzufügen, importieren, neu) und Einstellungen.

use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use tauri::AppHandle;

use crate::archive;
use crate::autostart;
use crate::config::{self, Config};
use crate::gui::{Cmd, blocking, run_task};
use crate::java;
use crate::launcher;
use crate::net;
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
            return err("Umschalten geht nur, wenn der aktuelle Server gestoppt ist.");
        }
        if crate::ops::current_job().is_some() {
            return err("Bitte warten, bis die laufende Aufgabe fertig ist.");
        }
        let p = PathBuf::from(&path);
        if !p.is_dir() {
            return err(format!("Ordner fehlt: {path}"));
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
            return err(format!(
                "In {} liegt weder eine Server-Jar noch eine server.properties.",
                root.display()
            ));
        }
        let cfg = Config::load();
        if cfg.entry(&root).is_some() {
            return err("Dieser Ordner ist schon in der Liste.");
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
            return err("Bitte einen Namen eingeben.");
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
pub fn import_server(app: AppHandle, archive: String, name: String) -> Cmd<()> {
    run_task(&app, &format!("Importiere „{name}“"), move |progress| {
        let path = archive::import_archive(Path::new(&archive), &name, progress)?;
        Ok(json!({"path": path, "eula": server::eula_accepted(&path)}))
    })
    .map_err(|e| e.0)
}

#[tauri::command]
pub async fn fabric_versions() -> Cmd<Vec<String>> {
    blocking(net::fabric_game_versions).await
}

#[tauri::command]
pub fn create_fabric_server(app: AppHandle, name: String, version: String) -> Cmd<()> {
    run_task(&app, &format!("Erstelle „{name}“"), move |progress| {
        let base = config::servers_dir();
        fs::create_dir_all(&base)?;
        let path = util::unique_dir(&base, &util::safe_name(&name));
        fs::create_dir_all(path.join("mods"))?;
        let res = (|| -> Result<()> {
            progress(0.05, "Suche passenden Fabric-Loader …");
            let (url, loader) = net::fabric_server_jar_url(&version)?;
            progress(0.1, &format!("Lade Fabric {loader} für Minecraft {version} …"));
            net::download(&url, &path.join("fabric-server-launch.jar"), None, None, &|f| {
                progress(0.1 + f * 0.8, "Lade Fabric-Server …")
            })?;
            let props: BTreeMap<String, String> = [
                ("motd", name.as_str()),
                ("enable-rcon", "true"),
                ("rcon.port", "25575"),
                ("rcon.password", &util::random_hex(12)),
                ("white-list", "true"),
                ("server-port", "25565"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
            server::write_properties(&path, &props)?;
            let mut env = vec![("MC_VERSION", version.clone())];
            progress(0.95, "Prüfe Java …");
            if let Ok(Some(need)) = net::required_java(&version) {
                let have = java::java_major(&java::default_java());
                if have < need {
                    match java::java_at_least(need) {
                        Some(j) => env.push(("MC_JAVA", j)),
                        None => {
                            return err(format!(
                                "Minecraft {version} braucht Java {need}, das ist nicht installiert. {}",
                                java_hint(need)
                            ));
                        }
                    }
                }
            }
            server::write_env(&path, &env)?;
            server::prepare_server(&path, true)?;
            Ok(())
        })();
        if let Err(e) = res {
            let _ = fs::remove_dir_all(&path);
            return Err(e);
        }
        Ok(json!({"path": path}))
    })
    .map_err(|e| e.0)
}

pub fn java_hint(need: u32) -> String {
    if cfg!(windows) {
        format!("Installieren z. B. von https://adoptium.net (Temurin {need}).")
    } else {
        format!("Installieren mit: sudo apt install openjdk-{need}-jre-headless")
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
            return err("Bitte den Server erst stoppen.");
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

const PROP_KEYS: &[&str] = &[
    "motd",
    "max-players",
    "difficulty",
    "gamemode",
    "view-distance",
    "simulation-distance",
    "spawn-protection",
    "server-port",
    "white-list",
];

#[tauri::command]
pub async fn get_settings() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let mut out = json!({
            "total_memory": procs::total_memory(),
            "notify": cfg.notify,
            "daily": cfg.daily_backup,
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
            let sel: BTreeMap<&str, String> = PROP_KEYS
                .iter()
                .map(|k| (*k, props.get(*k).cloned().unwrap_or_default()))
                .collect();
            out["props"] = json!(sel);
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
            .ok_or_else(|| Error("Kein Server ausgewählt.".into()))?;
        let props: BTreeMap<String, String> = props
            .into_iter()
            .filter(|(k, _)| PROP_KEYS.contains(&k.as_str()))
            .collect();
        if let Some(port) = props.get("server-port") {
            let p: u32 = port.parse().map_err(|_| Error("Port muss eine Zahl sein.".into()))?;
            if !(1024..=65535).contains(&p) {
                return err("Port muss zwischen 1024 und 65535 liegen.");
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
