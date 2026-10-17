//! Umstieg von „Redcraftserver“ (bis 0.3.0) auf „Vapafane“.
//!
//! Läuft einmal beim Start des Fensters bzw. der Befehlszeile:
//! - läuft noch der alte Hintergrund-Dienst, stoppt er den Minecraft-Server sauber
//!   (RCON „stop“, Welt wird gespeichert) und beendet sich; danach startet Vapafane
//!   den Server wieder,
//! - alte systemd-Units werden durch die neuen ersetzt (An/Aus-Zustand bleibt),
//! - der alte Login-Autostart wird durch den neuen ersetzt.
//!
//! Die Konfiguration in `mc-server-panel/config.json` bleibt unverändert.

use serde_json::Value;
use std::fs;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::config;
use crate::ipc;
use crate::launcher;
use crate::util::Result;

/// Meldung für das Fenster (wird einmal angezeigt).
pub static MESSAGE: Mutex<Option<String>> = Mutex::new(None);

fn marker() -> std::path::PathBuf {
    config::state_dir().join("umstieg-von-redcraftserver.txt")
}

/// Gibt eine kurze Meldung zurück, wenn etwas übernommen wurde.
pub fn run() -> Option<String> {
    if config::test_home().is_some() && std::env::var_os("VAPAFANE_TEST_MIGRATION").is_none() {
        return None;
    }
    let old_dir = config::old_state_dir();
    if marker().exists() || !old_dir.exists() {
        return None;
    }
    match migrate(&old_dir) {
        Ok(Some(msg)) => {
            let _ = fs::create_dir_all(config::state_dir());
            let _ = fs::write(marker(), &msg);
            Some(msg)
        }
        Ok(None) => {
            let _ = fs::create_dir_all(config::state_dir());
            let _ = fs::write(marker(), "nichts zu übernehmen");
            None
        }
        Err(e) => Some(format!("Umstieg von Redcraftserver nicht ganz geklappt: {e}")),
    }
}

fn migrate(old_dir: &std::path::Path) -> Result<Option<String>> {
    let mut done: Vec<String> = vec![];
    let old_file = old_dir.join("service.json");

    // 1) Läuft der alte Dienst noch? Dann den Server darin sauber stoppen lassen.
    let mut restart = false;
    if let Ok(st) = ipc::call_at(&old_file, "status", Value::Null) {
        let state = st
            .get("state")
            .and_then(|s| s.as_str())
            .unwrap_or("stopped")
            .to_string();
        restart = matches!(state.as_str(), "running" | "starting" | "restarting");
        // „quit“ stoppt den Server per RCON „stop“ (Welt wird gespeichert) und beendet den Dienst
        let _ = ipc::call_at(&old_file, "quit", Value::Null);
        let end = Instant::now() + Duration::from_secs(240);
        while Instant::now() < end && ipc::call_at(&old_file, "ping", Value::Null).is_ok() {
            std::thread::sleep(Duration::from_secs(1));
        }
        done.push(if restart {
            "Der alte Hintergrund-Dienst hat den Server sauber gestoppt (Welt gespeichert).".into()
        } else {
            "Der alte Hintergrund-Dienst wurde beendet.".into()
        });
    }
    let _ = fs::remove_file(&old_file);
    // Tägliches Backup: Zeitpunkt des letzten übernehmen, damit es nicht doppelt läuft
    let _ = fs::create_dir_all(config::state_dir());
    if old_dir.join("daily.json").exists() && !config::state_dir().join("daily.json").exists() {
        let _ = fs::copy(old_dir.join("daily.json"), config::state_dir().join("daily.json"));
    }

    // 2) systemd-Units und Login-Autostart (nur Linux)
    #[cfg(target_os = "linux")]
    done.extend(crate::autostart::replace_old_units()?);

    // 3) Server unter Vapafane wieder starten, wenn er vorher lief
    if restart {
        launcher::ensure_service()?;
        ipc::call("start", Value::Null)?;
        done.push("Der Server läuft jetzt unter Vapafane weiter.".into());
    }
    Ok(if done.is_empty() { None } else { Some(done.join("\n")) })
}
