//! Datentypen, die Backend (Tauri-Befehle) und Oberfläche (Leptos/WASM) austauschen.
//! Beide Seiten nutzen dieselben Strukturen – passt ein Feld nicht, fällt das beim Bauen auf.

// Kein unwrap/expect und kein unsafe – Fehler werden sauber behandelt.
#![deny(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};

/// Ein Server aus der Liste (Auswahl oben).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerItem {
    pub name: String,
    pub path: String,
    pub active: bool,
}

/// Aktiver Server.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ActiveServer {
    pub name: String,
    pub path: String,
}

/// Zustand für die Kopfzeile und die Übersicht (`get_status`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Status {
    /// none, stopped, starting, running, stopping, restarting, failed, external, other
    pub state: String,
    pub server: Option<ActiveServer>,
    pub servers: Vec<ServerItem>,
    pub busy: Option<String>,
    pub job: Option<String>,
    pub uptime: Option<u64>,
    pub restarts: Option<u32>,
    pub countdown: Option<String>,
    pub pid: Option<u32>,
    pub cpu: Option<f64>,
    pub ncpu: Option<u32>,
    pub ram: Option<u64>,
    pub ram_limit_gb: Option<f64>,
    pub eula: Option<bool>,
    /// Letzte Fehlermeldung beim Beenden (nach Absturz)
    pub last_exit: Option<String>,
    pub service: bool,
}

impl Status {
    /// Läuft gerade ein Server (oder fährt er hoch/runter)?
    pub fn up(&self) -> bool {
        matches!(
            self.state.as_str(),
            "running" | "starting" | "stopping" | "restarting"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_aus_teil_json() {
        // Fehlende Felder sind kein Fehler – das Backend schickt nur, was es weiß
        let s: Status = serde_json::from_str(
            r#"{"state":"running","servers":[{"name":"A","path":"/a","active":true}],"x":1}"#,
        )
        .unwrap();
        assert!(s.up());
        assert_eq!(s.servers[0].name, "A");
        assert!(s.server.is_none() && s.busy.is_none());
    }
}
