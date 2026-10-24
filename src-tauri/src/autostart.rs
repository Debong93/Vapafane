//! Autostart beim PC-Start und Fenster beim Anmelden.
//!
//! Der Hintergrund-Dienst soll beim PC-Start laufen, wenn der Server automatisch
//! starten soll oder das tägliche Backup an ist. Den Server selbst startet er nur
//! bei „Autostart“.
//!
//! Linux: systemd-User-Units `vapafane.service` (Dienst) und
//! `vapafane-autostart.service` (startet einmal den Server). Für den Start
//! ohne Anmeldung braucht es `loginctl enable-linger` (nur auf Klick des Nutzers).
//! Windows: Aufgabe „Vapafane“ in der Aufgabenplanung („beim Systemstart“),
//! das Fenster über den Run-Schlüssel in der Registry.

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux {
    use std::fs;
    use std::path::PathBuf;

    use crate::config;
    use crate::launcher::self_exe;
    use crate::util::{self, Result, err};

    pub const UNIT: &str = "vapafane.service";
    const START_UNIT: &str = "vapafane-autostart.service";

    fn unit_dir() -> PathBuf {
        config::home().join(".config/systemd/user")
    }

    fn desktop_autostart_path() -> PathBuf {
        config::home().join(".config/autostart/vapafane.desktop")
    }

    /// systemd nur im echten Betrieb nutzen, nicht im Testmodus.
    pub fn systemd_usable() -> bool {
        config::test_home().is_none()
            && util::command("systemctl")
                .args(["--user", "show-environment"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
    }

    fn systemctl(args: &[&str]) -> Result<String> {
        let out = util::command("systemctl").arg("--user").args(args).output()?;
        if !out.status.success() {
            return err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    fn quoted_exe() -> String {
        format!("\"{}\"", self_exe().to_string_lossy().replace('"', "\\\""))
    }

    fn service_text() -> String {
        format!(
            "[Unit]\n\
             Description=Vapafane – Hintergrund-Dienst für den Minecraft-Server\n\
             After=network-online.target\n\
             \n\
             [Service]\n\
             Type=simple\n\
             ExecStart={} --service\n\
             # Beim Stoppen bekommt nur der Dienst das Signal; er stoppt Java selbst\n\
             # sauber per RCON. Stürzt der Dienst ab, läuft Java weiter und wird\n\
             # beim Neustart des Dienstes wieder übernommen.\n\
             KillMode=process\n\
             TimeoutStopSec=210\n\
             Restart=on-failure\n\
             RestartSec=10\n\
             \n\
             [Install]\n\
             WantedBy=default.target\n",
            quoted_exe()
        )
    }

    fn start_text() -> String {
        format!(
            "[Unit]\n\
             Description=Vapafane – aktiven Minecraft-Server beim Start hochfahren\n\
             Wants={UNIT}\n\
             After={UNIT} network-online.target\n\
             \n\
             [Service]\n\
             Type=oneshot\n\
             ExecStart={} --ctl start\n\
             \n\
             [Install]\n\
             WantedBy=default.target\n",
            quoted_exe()
        )
    }

    /// Unit-Dateien schreiben (oder aktualisieren, wenn sich der Programmpfad geändert hat).
    pub fn install_unit() -> Result<()> {
        let mut changed = false;
        for (name, text) in [(UNIT, service_text()), (START_UNIT, start_text())] {
            let path = unit_dir().join(name);
            if fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
                util::write_atomic(&path, text.as_bytes())?;
                changed = true;
            }
        }
        if changed {
            systemctl(&["daemon-reload"])?;
        }
        Ok(())
    }

    fn is_enabled(unit: &str) -> bool {
        systemctl(&["is-enabled", unit])
            .map(|s| s == "enabled")
            .unwrap_or(false)
    }

    /// (Server startet beim PC-Start, Dienst läuft beim PC-Start)
    pub fn boot_state() -> (bool, bool) {
        if !systemd_usable() {
            return (false, false);
        }
        (is_enabled(START_UNIT), is_enabled(UNIT))
    }

    pub fn apply_boot(server: bool, service: bool) -> Result<()> {
        if !systemd_usable() {
            return err(crate::sprache::t!("err-kein-systemd"));
        }
        install_unit()?;
        systemctl(&[if service || server { "enable" } else { "disable" }, UNIT])?;
        systemctl(&[if server { "enable" } else { "disable" }, START_UNIT])?;
        Ok(())
    }

    /// Darf der Dienst laufen, ohne dass jemand angemeldet ist?
    pub fn linger_enabled() -> bool {
        let user = std::env::var("USER").unwrap_or_default();
        PathBuf::from("/var/lib/systemd/linger").join(user).exists()
    }

    pub fn enable_linger() -> Result<()> {
        let out = util::command("loginctl").arg("enable-linger").output()?;
        if !out.status.success() {
            return err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
        Ok(())
    }

    pub fn window_autostart_enabled() -> bool {
        desktop_autostart_path().exists()
    }

    pub fn set_window_autostart(on: bool) -> Result<()> {
        let path = desktop_autostart_path();
        if on {
            let text = format!(
                "[Desktop Entry]\nType=Application\nName=Vapafane\nComment=Minecraft-Serververwaltung\n\
                 Exec={}\nIcon=vapafane\nTerminal=false\nX-GNOME-Autostart-enabled=true\n\
                 X-GNOME-Autostart-Delay=5\n",
                quoted_exe()
            );
            util::write_atomic(&path, text.as_bytes())?;
        } else if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Ist das alte Panel (minecraft.service) noch eingerichtet? (aktiv, beim Start an, Backup-Timer an)
    pub fn old_panel() -> Option<(bool, bool, bool)> {
        if config::test_home().is_some() || !unit_dir().join("minecraft.service").exists() {
            return None;
        }
        let active = systemctl(&["is-active", "minecraft.service"])
            .map(|s| s == "active")
            .unwrap_or(false);
        Some((
            active,
            is_enabled("minecraft.service"),
            is_enabled("minecraft-backup.timer"),
        ))
    }

    #[cfg(test)]
    #[test]
    fn writes_units_for_check() {
        let dir = std::env::temp_dir().join("vapafane-units");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(UNIT), service_text()).unwrap();
        fs::write(dir.join(START_UNIT), start_text()).unwrap();
        assert!(service_text().contains("KillMode=process"));
    }

    /// Altes Panel abschalten: Dienst und Backup-Timer stoppen und nicht mehr automatisch starten.
    /// Der Dienst stoppt den Server dabei sauber (sein ExecStop schickt „stop“ per RCON).
    /// Die Dateien des alten Panels bleiben liegen, man kann es jederzeit wieder einschalten.
    pub fn disable_old_panel() -> Result<()> {
        systemctl(&["disable", "--now", "minecraft-backup.timer"]).ok();
        systemctl(&["disable", "--now", "minecraft.service"])?;
        Ok(())
    }
}

#[cfg(windows)]
mod windows {
    use std::path::PathBuf;

    use crate::config::{self, Config};
    use crate::launcher::self_exe;
    use crate::util::{self, Result, err};

    const TASK: &str = "Vapafane";
    const RUN_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";

    pub fn systemd_usable() -> bool {
        false
    }

    fn task_exists() -> bool {
        util::command("schtasks")
            .args(["/Query", "/TN", TASK])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// (Server startet beim PC-Start, Dienst läuft beim PC-Start)
    pub fn boot_state() -> (bool, bool) {
        if !task_exists() {
            return (false, false);
        }
        (Config::load().autostart_server, true)
    }

    fn xml_escape(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn current_user() -> String {
        let user = std::env::var("USERNAME").unwrap_or_default();
        match std::env::var("USERDOMAIN") {
            Ok(d) if !d.is_empty() => format!("{d}\\{user}"),
            _ => user,
        }
    }

    /// Aufgabe als XML: läuft beim Systemstart ohne Anmeldung (ohne Passwort, „S4U“),
    /// ohne Zeitlimit, auch im Akkubetrieb und mit normaler Priorität.
    fn task_xml(start_server: bool) -> String {
        let exe = xml_escape(&self_exe().to_string_lossy());
        let user = xml_escape(&current_user());
        let args = if start_server { "--service --start" } else { "--service" };
        format!(
            r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo><Description>Vapafane: Hintergrund-Dienst für den Minecraft-Server</Description></RegistrationInfo>
  <Triggers><BootTrigger><Enabled>true</Enabled><Delay>PT30S</Delay></BootTrigger></Triggers>
  <Principals><Principal id="Author"><UserId>{user}</UserId><LogonType>S4U</LogonType><RunLevel>LeastPrivilege</RunLevel></Principal></Principals>
  <Settings>
    <MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <ExecutionTimeLimit>PT0S</ExecutionTimeLimit>
    <Priority>5</Priority>
    <StartWhenAvailable>true</StartWhenAvailable>
    <RestartOnFailure><Interval>PT1M</Interval><Count>3</Count></RestartOnFailure>
  </Settings>
  <Actions Context="Author"><Exec><Command>{exe}</Command><Arguments>{args}</Arguments></Exec></Actions>
</Task>
"#
        )
    }

    /// schtasks mit Administratorrechten ausführen (Windows fragt per UAC nach).
    fn schtasks_elevated(args: &str) -> Result<()> {
        let ps_args = args.replace('\'', "''");
        let script = format!(
            "$p = Start-Process -FilePath schtasks.exe -ArgumentList '{ps_args}' -Verb RunAs -Wait -WindowStyle Hidden -PassThru; exit $p.ExitCode"
        );
        let out = util::command("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()?;
        if !out.status.success() {
            return err(crate::sprache::t!("err-aufgabenplanung", details = String::from_utf8_lossy(&out.stderr).trim().to_string()));
        }
        Ok(())
    }

    pub fn apply_boot(server: bool, service: bool) -> Result<()> {
        if server || service {
            let xml_path: PathBuf = config::state_dir().join("autostart-aufgabe.xml");
            std::fs::create_dir_all(config::state_dir())?;
            // schtasks erwartet UTF-16 mit BOM
            let mut bytes = vec![0xFF, 0xFE];
            for u in task_xml(server).encode_utf16() {
                bytes.extend_from_slice(&u.to_le_bytes());
            }
            std::fs::write(&xml_path, bytes)?;
            schtasks_elevated(&format!("/Create /F /TN {TASK} /XML \"{}\"", xml_path.display()))?;
        } else if task_exists() {
            schtasks_elevated(&format!("/Delete /F /TN {TASK}"))?;
        }
        Ok(())
    }

    pub fn linger_enabled() -> bool {
        true
    }

    pub fn enable_linger() -> Result<()> {
        Ok(())
    }

    pub fn window_autostart_enabled() -> bool {
        util::command("reg")
            .args(["query", RUN_KEY, "/v", TASK])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn set_window_autostart(on: bool) -> Result<()> {
        let out = if on {
            let exe = format!("\"{}\"", self_exe().display());
            util::command("reg")
                .args(["add", RUN_KEY, "/v", TASK, "/t", "REG_SZ", "/d", &exe, "/f"])
                .output()?
        } else {
            util::command("reg")
                .args(["delete", RUN_KEY, "/v", TASK, "/f"])
                .output()?
        };
        if !out.status.success() {
            return err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
        Ok(())
    }

    pub fn old_panel() -> Option<(bool, bool, bool)> {
        None
    }

    pub fn disable_old_panel() -> Result<()> {
        Ok(())
    }
}
