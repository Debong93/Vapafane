//! Konfiguration und feste Orte.
//!
//! Die Konfiguration ist dieselbe Datei wie beim alten Python-Panel
//! (`~/.config/mc-server-panel/config.json`). Unbekannte Felder bleiben beim
//! Speichern erhalten, damit beide Programme dieselbe Datei nutzen können.
//!
//! Für Tests lässt sich alles mit der Umgebungsvariable `VAPAFANE_HOME`
//! in einen anderen Ordner umleiten (dann wird auch kein systemd benutzt).

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs;
use std::path::{Path, PathBuf};

use crate::util::{self, Result};

pub const ENV_NAME: &str = ".mcpanel.env";
pub const ALT_DIR_NAME: &str = ".panel-alt";
pub const STAGING_NAME: &str = ".update-staging";

/// Basisordner (normalerweise das Home-Verzeichnis).
pub fn home() -> PathBuf {
    if let Some(h) = test_home() {
        return h;
    }
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

/// Gesetzt, wenn im Testmodus gearbeitet wird.
pub fn test_home() -> Option<PathBuf> {
    ["VAPAFANE_HOME", "REDCRAFTSERVER_HOME"] // alter Name bis 0.3.0
        .iter()
        .find_map(|k| std::env::var_os(k).filter(|v| !v.is_empty()))
        .map(PathBuf::from)
}

pub fn config_dir() -> PathBuf {
    if let Some(h) = test_home() {
        return h.join(".config").join("mc-server-panel");
    }
    dirs::config_dir()
        .unwrap_or_else(|| home().join(".config"))
        .join("mc-server-panel")
}

/// Ordner für Daten von Vapafane (z. B. eingerichtete Clients):
/// Linux ~/.local/share/vapafane, Windows %APPDATA%\vapafane.
pub fn data_dir() -> PathBuf {
    if let Some(h) = test_home() {
        return h.join(".local").join("share").join("vapafane");
    }
    dirs::data_dir()
        .unwrap_or_else(|| home().join(".local").join("share"))
        .join("vapafane")
}

pub fn config_file() -> PathBuf {
    config_dir().join("config.json")
}

/// Ordner für Laufzeit-Dateien des Hintergrund-Dienstes.
pub fn state_dir() -> PathBuf {
    config_dir().join("vapafane")
}

/// Laufzeit-Ordner der Vorgängerversion (hieß bis 0.3.0 „Redcraftserver“).
pub fn old_state_dir() -> PathBuf {
    config_dir().join("redcraftserver")
}

pub fn servers_dir() -> PathBuf {
    home().join("MC_Servers")
}

/// Unter Linux zeigt dieser Link auf den aktiven Server (wie beim alten Panel).
#[cfg(unix)]
pub fn active_link() -> PathBuf {
    home().join("MC_Active")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerEntry {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub servers: Vec<ServerEntry>,
    #[serde(default = "default_true")]
    pub notify: bool,
    #[serde(default = "default_stop_warning")]
    pub stop_warning: u64,
    #[serde(default = "default_backup_dir")]
    pub backup_dir: String,
    #[serde(default = "default_keep")]
    pub backup_keep: usize,
    /// Tägliches Backup durch den Hintergrund-Dienst.
    #[serde(default)]
    pub daily_backup: bool,
    #[serde(default = "default_daily_time")]
    pub daily_backup_time: String,
    /// Server beim PC-Start automatisch starten (Wunsch; unter Linux zählt die systemd-Unit)
    #[serde(default)]
    pub autostart_server: bool,
    /// Aktiver Server (unter Windows maßgeblich, unter Linux nur Kopie des Links).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    /// Alle anderen Felder (z. B. `last_update`) unverändert mitspeichern.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

fn default_true() -> bool {
    true
}
fn default_stop_warning() -> u64 {
    30
}
fn default_backup_dir() -> String {
    home().join("MC_Backups").to_string_lossy().into_owned()
}
fn default_keep() -> usize {
    5
}
fn default_daily_time() -> String {
    "05:00".into()
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str("{}").expect("leere Konfiguration")
    }
}

impl Config {
    pub fn load() -> Config {
        fs::read_to_string(config_file())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let text = serde_json::to_string_pretty(self)?;
        util::write_atomic(&config_file(), text.as_bytes())
    }

    pub fn entry(&self, path: &Path) -> Option<&ServerEntry> {
        let path = util::canonical(path);
        self.servers
            .iter()
            .find(|s| util::canonical(Path::new(&s.path)) == path)
    }

    pub fn name_of(&self, path: &Path) -> String {
        match self.entry(path) {
            Some(e) => e.name.clone(),
            None => path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "?".into()),
        }
    }

    pub fn add_server(&mut self, name: &str, path: &Path) -> Result<()> {
        if self.entry(path).is_none() {
            self.servers.push(ServerEntry {
                name: name.to_string(),
                path: util::canonical(path).to_string_lossy().into_owned(),
            });
            self.save()?;
        }
        Ok(())
    }

    pub fn backup_dir_for(&self, name: &str) -> PathBuf {
        PathBuf::from(&self.backup_dir).join(util::safe_name(name))
    }

    /// Pfad des aktiven Servers.
    pub fn active_path(&self) -> Option<PathBuf> {
        #[cfg(unix)]
        {
            let link = active_link();
            if fs::symlink_metadata(&link)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false)
            {
                return Some(util::canonical(&link));
            }
            None
        }
        #[cfg(not(unix))]
        {
            self.active.as_ref().map(|p| util::canonical(Path::new(p)))
        }
    }

    pub fn set_active(&mut self, path: Option<&Path>) -> Result<()> {
        #[cfg(unix)]
        {
            let link = active_link();
            let tmp = link.with_extension("tmp");
            let _ = fs::remove_file(&tmp);
            match path {
                Some(p) => {
                    std::os::unix::fs::symlink(p, &tmp)?;
                    fs::rename(&tmp, &link)?;
                }
                None => {
                    if fs::symlink_metadata(&link).is_ok() {
                        fs::remove_file(&link)?;
                    }
                }
            }
        }
        self.active = path.map(|p| p.to_string_lossy().into_owned());
        self.save()
    }
}
