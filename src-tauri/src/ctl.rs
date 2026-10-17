//! Befehlszeile: `redcraftserver --ctl <befehl>` – für Skripte und zum Testen.

use serde_json::Value;
use std::time::Duration;

use crate::config::Config;
use crate::util::{Result, err};
use crate::{ipc, launcher, ops, server};

const HELP: &str = "Benutzung: redcraftserver --ctl <befehl>

  status            Zustand des aktiven Servers anzeigen
  start             Server starten (startet bei Bedarf den Hintergrund-Dienst)
  stop [SEKUNDEN]   Server sauber stoppen, optional mit Countdown-Warnung
  restart [SEK.]    Server neu starten
  wait-ready [SEK.] warten, bis der Server bereit ist
  wait-stopped [S.] warten, bis der Server aus ist
  backup            ganzen Server-Ordner als ZIP sichern
  cmd <BEFEHL …>    Befehl per RCON schicken
  quit-service      Hintergrund-Dienst beenden (stoppt vorher den Server)";

pub fn run(args: &[String]) -> i32 {
    match run_inner(args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Fehler: {e}");
            1
        }
    }
}

fn state() -> String {
    ops::server_state(&Config::load())
        .get("state")
        .and_then(|s| s.as_str())
        .unwrap_or("?")
        .to_string()
}

fn wait_for(want: &[&str], secs: u64) -> Result<()> {
    for _ in 0..secs {
        let s = state();
        if want.contains(&s.as_str()) {
            println!("{s}");
            return Ok(());
        }
        if s == "failed" {
            return err("Server ist abgestürzt");
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    err(format!("Zeitüberschreitung (Zustand: {})", state()))
}

fn run_inner(args: &[String]) -> Result<()> {
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let secs = |default: u64| args.get(1).and_then(|s| s.parse().ok()).unwrap_or(default);
    match cmd {
        "status" => {
            println!("{}", serde_json::to_string_pretty(&ops::server_state(&Config::load()))?);
        }
        "start" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            if !server::eula_accepted(&dir) {
                return err("EULA nicht akzeptiert (im Fenster bestätigen)");
            }
            launcher::ensure_service()?;
            ipc::call("start", Value::Null)?;
            println!("Server wird gestartet.");
        }
        "stop" | "restart" => {
            ops::stop_server(secs(0), cmd == "restart")?;
            println!(
                "{}",
                if cmd == "stop" {
                    "Server wird gestoppt."
                } else {
                    "Server wird neu gestartet."
                }
            );
        }
        "wait-ready" => wait_for(&["running"], secs(600))?,
        "wait-stopped" => wait_for(&["stopped", "none"], secs(240))?,
        "backup" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            let _lock = ops::JobLock::acquire("Backup (Befehlszeile)")?;
            let running = state() == "running";
            let progress = |_: f64, _: &str| {};
            let path = ops::backup_with_save(&dir, &cfg, running, &progress)?;
            println!("{}", path.display());
        }
        "cmd" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            println!("{}", crate::rcon::once(&dir, &args[1..].join(" "))?);
        }
        "quit-service" => {
            ipc::call("quit", Value::Null)?;
            println!("Dienst wird beendet.");
        }
        _ => println!("{HELP}"),
    }
    Ok(())
}
