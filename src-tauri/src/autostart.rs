//! Autostart beim PC-Start und Fenster beim Anmelden.
//!
//! Linux: systemd-User-Unit `redcraftserver.service`; für den Start ohne
//! Anmeldung braucht es `loginctl enable-linger` (nur auf Klick des Nutzers).
//! Windows: Aufgabe „Redcraftserver“ in der Aufgabenplanung („beim Systemstart“),
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

    pub const UNIT: &str = "redcraftserver.service";

    fn unit_path() -> PathBuf {
        config::home().join(".config/systemd/user").join(UNIT)
    }

    fn desktop_autostart_path() -> PathBuf {
        config::home().join(".config/autostart/redcraftserver.desktop")
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

    fn unit_text() -> String {
        let exe = self_exe().to_string_lossy().replace('"', "\\\"");
        format!(
            "[Unit]\n\
             Description=Redcraftserver – Hintergrund-Dienst für den Minecraft-Server\n\
             After=network-online.target\n\
             \n\
             [Service]\n\
             Type=simple\n\
             ExecStart=\"{exe}\" --service --start\n\
             # Nur den Dienst benachrichtigen; er stoppt Java selbst sauber per RCON\n\
             KillMode=mixed\n\
             TimeoutStopSec=210\n\
             Restart=on-failure\n\
             RestartSec=10\n\
             \n\
             [Install]\n\
             WantedBy=default.target\n"
        )
    }

    /// Unit-Datei schreiben (oder aktualisieren, wenn sich der Programmpfad geändert hat).
    pub fn install_unit() -> Result<()> {
        let path = unit_path();
        let text = unit_text();
        if fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
            util::write_atomic(&path, text.as_bytes())?;
            systemctl(&["daemon-reload"])?;
        }
        Ok(())
    }

    pub fn server_autostart_enabled() -> bool {
        systemctl(&["is-enabled", UNIT])
            .map(|s| s == "enabled")
            .unwrap_or(false)
    }

    pub fn set_server_autostart(on: bool) -> Result<()> {
        if !systemd_usable() {
            return err("systemd ist hier nicht verfügbar (oder Testmodus).");
        }
        install_unit()?;
        systemctl(&[if on { "enable" } else { "disable" }, UNIT])?;
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
            let exe = self_exe().to_string_lossy().into_owned();
            let text = format!(
                "[Desktop Entry]\nType=Application\nName=Redcraftserver\nComment=Minecraft-Serververwaltung\n\
                 Exec=\"{exe}\"\nIcon=redcraftserver\nTerminal=false\nX-GNOME-Autostart-enabled=true\n\
                 X-GNOME-Autostart-Delay=5\n"
            );
            util::write_atomic(&path, text.as_bytes())?;
        } else if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Ist das alte Panel (minecraft.service) noch eingerichtet?
    pub fn old_panel_unit() -> Option<String> {
        let state = systemctl(&["is-active", "minecraft.service"]).unwrap_or_else(|e| e.0);
        let enabled = systemctl(&["is-enabled", "minecraft.service"]).unwrap_or_default();
        if config::home().join(".config/systemd/user/minecraft.service").exists() {
            Some(format!("{state}/{enabled}"))
        } else {
            None
        }
    }
}

#[cfg(windows)]
mod windows {
    use std::path::PathBuf;

    use crate::config;
    use crate::launcher::self_exe;
    use crate::util::{self, Result, err};

    const TASK: &str = "Redcraftserver";
    const RUN_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";

    pub fn systemd_usable() -> bool {
        false
    }

    pub fn server_autostart_enabled() -> bool {
        util::command("schtasks")
            .args(["/Query", "/TN", TASK])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
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
    fn task_xml() -> String {
        let exe = xml_escape(&self_exe().to_string_lossy());
        let user = xml_escape(&current_user());
        format!(
            r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo><Description>Redcraftserver: startet den Minecraft-Server beim PC-Start</Description></RegistrationInfo>
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
  <Actions Context="Author"><Exec><Command>{exe}</Command><Arguments>--service --start</Arguments></Exec></Actions>
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
            return err(format!(
                "Aufgabenplanung hat nicht geklappt (abgebrochen oder keine Berechtigung). {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        Ok(())
    }

    pub fn set_server_autostart(on: bool) -> Result<()> {
        if on {
            let xml_path: PathBuf = config::state_dir().join("autostart-aufgabe.xml");
            std::fs::create_dir_all(config::state_dir())?;
            // schtasks erwartet UTF-16 mit BOM
            let mut bytes = vec![0xFF, 0xFE];
            for u in task_xml().encode_utf16() {
                bytes.extend_from_slice(&u.to_le_bytes());
            }
            std::fs::write(&xml_path, bytes)?;
            schtasks_elevated(&format!("/Create /F /TN {TASK} /XML \"{}\"", xml_path.display()))?;
        } else {
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

    pub fn old_panel_unit() -> Option<String> {
        None
    }
}
