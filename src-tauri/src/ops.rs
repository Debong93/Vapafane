//! Arbeiten, die Fenster und Dienst gleichermaßen brauchen.

use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::archive::{self, Progress};
use crate::config::{self, Config};
use crate::ipc;
use crate::procs;
use crate::rcon::Rcon;
use crate::util::{self, Result, err};

/// Sperre für längere Arbeiten (Backup, Update, Wiederherstellen …), damit
/// Fenster und Dienst nicht gleichzeitig am Server-Ordner arbeiten.
pub struct JobLock {
    _file: fs::File,
}

fn lock_path() -> PathBuf {
    config::state_dir().join("job.lock")
}

fn job_text_path() -> PathBuf {
    config::state_dir().join("job.txt")
}

impl JobLock {
    pub fn acquire(what: &str) -> Result<JobLock> {
        fs::create_dir_all(config::state_dir())?;
        let f = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(lock_path())?;
        if f.try_lock().is_err() {
            let other = current_job().unwrap_or_else(|| "eine andere Aufgabe".into());
            return err(format!("Es läuft gerade schon: {other}"));
        }
        let _ = fs::write(job_text_path(), what);
        Ok(JobLock { _file: f })
    }
}

impl Drop for JobLock {
    fn drop(&mut self) {
        let _ = fs::write(job_text_path(), "");
    }
}

/// Text der gerade laufenden Aufgabe (egal ob im Fenster oder im Dienst).
pub fn current_job() -> Option<String> {
    let f = fs::OpenOptions::new().write(true).open(lock_path()).ok()?;
    if f.try_lock().is_ok() {
        return None;
    }
    let t = fs::read_to_string(job_text_path()).unwrap_or_default();
    Some(if t.trim().is_empty() { "Aufgabe".into() } else { t })
}

/// Backup, auch während der Server läuft: Speichern kurz pausieren.
pub fn backup_with_save(dir: &Path, cfg: &Config, running: bool, progress: Progress) -> Result<PathBuf> {
    let name = cfg.name_of(dir);
    let mut rcon = Rcon::new(dir).with_timeout(Duration::from_secs(30));
    let mut paused = false;
    if running {
        progress(0.0, "Server speichert die Welt …");
        if rcon.command("say Backup läuft …").is_ok()
            && rcon.command("save-off").is_ok()
            && rcon.command("save-all flush").is_ok()
        {
            paused = true;
            std::thread::sleep(Duration::from_secs(2));
        }
    }
    let res = archive::backup_server(dir, &name, cfg, progress);
    if paused {
        let _ = rcon.command("save-on");
        let _ = rcon.command(if res.is_ok() {
            "say Backup fertig."
        } else {
            "say Backup fehlgeschlagen."
        });
    }
    res
}

/// Zustand des Servers: vom Dienst, sonst prüfen ob er außerhalb läuft.
pub fn server_state(cfg: &Config) -> Value {
    let dir = cfg.active_path();
    let svc = ipc::call("status", Value::Null).ok();
    let mut st = svc.clone().unwrap_or_else(|| json!({"state": "stopped"}));
    let svc_dir = st.get("dir").and_then(|d| d.as_str()).map(PathBuf::from);
    let svc_state = st
        .get("state")
        .and_then(|s| s.as_str())
        .unwrap_or("stopped")
        .to_string();
    st["service"] = json!(svc.is_some());
    let Some(dir) = dir else {
        st["state"] = json!("none");
        return st;
    };
    // Dienst kümmert sich um einen anderen Server als den ausgewählten?
    if matches!(svc_state.as_str(), "running" | "starting" | "stopping" | "restarting")
        && svc_dir
            .as_ref()
            .map(|d| util::canonical(d) != util::canonical(&dir))
            .unwrap_or(false)
    {
        st["state"] = json!("other");
        return st;
    }
    if (svc_state == "stopped" || svc_state == "failed")
        && let Some(pid) = procs::java_in_dir(&dir, None)
    {
        st["state"] = json!("external");
        st["pid"] = json!(pid);
        st["started_at"] = json!(procs::start_time(pid).unwrap_or(0));
    }
    st
}

/// Stopp-Befehl an den Dienst. Läuft der Server außerhalb von Vapafane,
/// bekommt er wenigstens sauber „stop“ per RCON.
pub fn stop_server(warn: u64, restart: bool) -> Result<()> {
    let cfg = Config::load();
    let st = server_state(&cfg);
    if st.get("state").and_then(|s| s.as_str()) == Some("external") {
        if restart {
            return err(
                "Dieser Server läuft außerhalb von Vapafane – Neustart geht nur über das Programm, das ihn gestartet hat.",
            );
        }
        let dir = cfg
            .active_path()
            .ok_or_else(|| util::Error("Kein Server ausgewählt.".into()))?;
        crate::rcon::once(&dir, "stop")?;
        return Ok(());
    }
    ipc::call(if restart { "restart" } else { "stop" }, json!({"warn": warn}))?;
    Ok(())
}

/// Vor dem Start: Benutzt ein anderes Programm (z. B. ein zweiter Server) schon den Port?
/// Dann würde Minecraft nur mit einem Fehler abbrechen.
pub fn check_port(dir: &Path) -> Result<()> {
    let port = crate::reach::server_port(dir);
    crate::reach::release_port(port);
    if !procs::port_free(port) && procs::java_in_dir(dir, None).is_none() {
        let free = procs::free_port(port.saturating_add(1), &[]);
        return err(format!(
            "Port {port} ist schon belegt – wahrscheinlich läuft dort ein anderer Minecraft-Server. \
             Stell unter Einstellungen → „Server-Port“ einen freien Port ein, z. B. {free}."
        ));
    }
    Ok(())
}
