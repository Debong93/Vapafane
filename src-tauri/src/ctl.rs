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
  neu TYP NAME VER [ORDNER]  neuen Server anlegen (fabric, paper, neoforge, vanilla) und aktiv machen
  update-check VER  Mods/Datapacks für Minecraft VER über Modrinth prüfen
  trial VER         Probelauf: Kopie aktualisieren, auf anderem Port testen, löschen
  update VER        Update: stoppen, Backup, tauschen, starten (nur wenn Prüfung ok)
  mods              Mods und Datapacks mit An/Aus-Zustand anzeigen
  mod-an PFAD       Mod einschalten (PFAD wie in „mods“, z. B. mods/x.jar)
  mod-aus PFAD      Mod ausschalten (wird zu .jar.disabled)
  aufraeumen [--ja] zeigt alte Reste; mit --ja werden sie gelöscht (Server muss aus sein)
  mrpack DATEI      Mods als Modrinth-Modpack (.mrpack) für Freunde exportieren
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
        "neu" => {
            let (Some(kind), Some(name), Some(version)) = (args.get(1), args.get(2), args.get(3)) else {
                return err("Benutzung: --ctl neu TYP NAME VERSION [ORDNER]");
            };
            let kind = crate::flavor::Kind::parse(kind).ok_or_else(|| crate::util::Error("Unbekannter Typ".into()))?;
            let target = match args.get(4) {
                Some(t) => std::path::PathBuf::from(t),
                None => crate::util::unique_dir(&crate::config::servers_dir(), &crate::util::safe_name(name)),
            };
            let print = |f: f64, t: &str| eprintln!("[{:>3.0} %] {t}", f * 100.0);
            let path = crate::flavor::create_server(kind, name, version, &target, &print)?;
            let mut cfg = Config::load();
            cfg.add_server(name, &path)?;
            if matches!(state().as_str(), "stopped" | "failed" | "none") {
                cfg.set_active(Some(&path))?;
            }
            println!("{}", path.display());
        }
        "update-check" | "trial" | "update" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            let target = args
                .get(1)
                .ok_or_else(|| crate::util::Error("Bitte Zielversion angeben".into()))?;
            let print = |f: f64, t: &str| eprintln!("[{:>3.0} %] {t}", f * 100.0);
            let plan = crate::updater::check(&dir, target, &print)?;
            for m in plan.mods.iter().chain(&plan.extra).chain(&plan.datapacks) {
                println!(
                    "{:<10} {:<9} {:<40} {} → {}",
                    m.status,
                    m.kind,
                    m.title,
                    m.current_version.clone().unwrap_or_default(),
                    m.new_version.clone().unwrap_or_default()
                );
            }
            println!(
                "Version {} → {}, Java nötig {:?} (ok: {}), Platz ok: {}, etwas zu tun: {}",
                plan.current,
                plan.target,
                plan.java_required,
                plan.java_ok,
                plan.space_ok,
                plan.has_work()
            );
            if cmd == "update-check" {
                return Ok(());
            }
            if !plan.java_ok || !plan.space_ok {
                return err("Java oder Speicherplatz passt nicht.");
            }
            let _lock = ops::JobLock::acquire(if cmd == "trial" { "Probelauf" } else { "Update" })?;
            let r = if cmd == "trial" {
                crate::updates_gui::do_trial(&dir, &plan, true, false, &print)?
            } else {
                crate::updates_gui::do_update(&dir, &plan, true, false, &print)?
            };
            println!("{}", serde_json::to_string_pretty(&r)?);
        }
        "mods" | "mod-an" | "mod-aus" | "aufraeumen" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            match cmd {
                "mods" => {
                    let l = crate::mods::list(&dir);
                    for m in l["mods"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .chain(l["datapacks"].as_array().into_iter().flatten())
                    {
                        println!(
                            "{:<4} {:<45} {}{}",
                            if m["enabled"].as_bool().unwrap_or(false) {
                                "an"
                            } else {
                                "AUS"
                            },
                            m["rel"].as_str().unwrap_or(""),
                            m["version"].as_str().unwrap_or(""),
                            m["reason"].as_str().map(|r| format!("  ({r})")).unwrap_or_default()
                        );
                    }
                }
                "mod-an" | "mod-aus" => {
                    let rel = args
                        .get(1)
                        .ok_or_else(|| crate::util::Error("Bitte Pfad angeben".into()))?;
                    crate::mods::set_enabled(&dir, rel, cmd == "mod-an")?;
                    println!("{rel}: {}", if cmd == "mod-an" { "an" } else { "aus" });
                }
                _ => {
                    let items = crate::cleanup::candidates(&dir);
                    for c in &items {
                        println!("{:>10}  {:<55} {}", crate::util::fmt_bytes(c.size), c.rel, c.what);
                    }
                    let total: u64 = items.iter().map(|c| c.size).sum();
                    println!("Zusammen: {}", crate::util::fmt_bytes(total));
                    if args.get(1).map(String::as_str) == Some("--ja") {
                        if !matches!(state().as_str(), "stopped" | "failed" | "none") {
                            return err("Aufräumen geht nur, wenn der Server gestoppt ist.");
                        }
                        let (freed, failed) = crate::cleanup::delete(&dir, &items);
                        println!(
                            "Gelöscht: {} (nicht möglich: {failed:?})",
                            crate::util::fmt_bytes(freed)
                        );
                    }
                }
            }
        }
        "mrpack" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            let dest = args
                .get(1)
                .ok_or_else(|| crate::util::Error("Bitte Zieldatei angeben".into()))?;
            let (files, overrides) =
                crate::updater::export_mrpack(&dir, std::path::Path::new(dest), &cfg.name_of(&dir))?;
            println!("{dest}: {files} Mods von Modrinth, {overrides} direkt eingepackt");
        }
        "quit-service" => {
            ipc::call("quit", Value::Null)?;
            println!("Dienst wird beendet.");
        }
        _ => println!("{HELP}"),
    }
    Ok(())
}
