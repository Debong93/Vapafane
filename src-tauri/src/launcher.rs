//! Den Hintergrund-Dienst starten, falls er noch nicht läuft.
//!
//! Linux: über eine systemd-User-Unit `vapafane.service` (dann überlebt der
//! Server das Schließen des Fensters und wird beim Abmelden sauber gestoppt).
//! Windows oder ohne systemd: als eigenständiger Hintergrundprozess.

use std::path::PathBuf;
use std::process::Stdio;
use std::time::{Duration, Instant};

use crate::config;
use crate::ipc;
#[cfg(target_os = "linux")]
use crate::util;
use crate::util::{Result, err};

/// Pfad dieses Programms (bei AppImage die AppImage-Datei selbst).
pub fn self_exe() -> PathBuf {
    if let Some(p) = std::env::var_os("APPIMAGE") {
        return PathBuf::from(p);
    }
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("vapafane"))
}

pub fn ensure_service() -> Result<()> {
    if ipc::running() {
        return Ok(());
    }
    #[cfg(target_os = "linux")]
    if crate::autostart::systemd_usable() {
        crate::autostart::install_unit()?;
        let out = util::command("systemctl")
            .args(["--user", "start", crate::autostart::UNIT])
            .output()?;
        if !out.status.success() {
            return err(format!(
                "Dienst konnte nicht gestartet werden: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        return wait_for_service();
    }
    spawn_detached()?;
    wait_for_service()
}

fn wait_for_service() -> Result<()> {
    let end = Instant::now() + Duration::from_secs(8);
    while Instant::now() < end {
        if ipc::running() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    err(format!(
        "Der Hintergrund-Dienst antwortet nicht. Details stehen in {}",
        config::state_dir().join("dienst.log").display()
    ))
}

fn spawn_detached() -> Result<()> {
    let mut cmd = std::process::Command::new(self_exe());
    cmd.arg("--service")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x0000_0008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
        cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }
    cmd.spawn()?;
    Ok(())
}
