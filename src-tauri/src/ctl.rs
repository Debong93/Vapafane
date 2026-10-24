//! Befehlszeile: `vapafane --ctl <befehl>` – für Skripte und zum Testen.

use serde_json::Value;
use std::time::Duration;

use crate::config::Config;
use crate::util::{Result, err};
use crate::{ipc, launcher, ops, server};

const HELP: &str = "Benutzung: vapafane --ctl <befehl>

  status            Zustand des aktiven Servers anzeigen
  start             Server starten (startet bei Bedarf den Hintergrund-Dienst)
  stop [SEKUNDEN]   Server sauber stoppen, optional mit Countdown-Warnung
  restart [SEK.]    Server neu starten
  wait-ready [SEK.] warten, bis der Server bereit ist
  wait-stopped [S.] warten, bis der Server aus ist
  backup            ganzen Server-Ordner als ZIP sichern
  cmd <BEFEHL …>    Befehl per RCON schicken
  aktiv NAME        anderen Server aus der Liste aktiv machen (nur wenn gestoppt)
  neu TYP NAME VER [ORDNER]  neuen Server anlegen (fabric, paper, neoforge, vanilla) und aktiv machen
  update-check VER  Mods/Datapacks für Minecraft VER über Modrinth prüfen
  trial VER         Probelauf: Kopie aktualisieren, auf anderem Port testen, löschen
  update VER        Update: stoppen, Backup, tauschen, starten (nur wenn Prüfung ok)
  mods              Mods und Datapacks mit An/Aus-Zustand anzeigen
  mod-an PFAD       Mod einschalten (PFAD wie in „mods“, z. B. mods/x.jar)
  mod-aus PFAD      Mod ausschalten (wird zu .jar.disabled)
  aufraeumen [--ja] zeigt alte Reste; mit --ja werden sie gelöscht (Server muss aus sein)
  mod-suche [BEGRIFF] [--datapack]   Modrinth nach passenden Mods/Plugins (bzw. Datapacks) durchsuchen
  mod-install PROJEKT [--datapack]  Mod/Plugin/Datapack samt Pflicht-Abhängigkeiten installieren
  curseforge-schluessel [--loeschen]  CurseForge-Schlüssel von der Eingabe lesen und speichern
  curseforge-test   gespeicherten CurseForge-Schlüssel ausprobieren
  einladung [ADRESSE]               Einladungs-Code für Freunde ausgeben (Adresse = host:port)
  einladung-datei DATEI [ADRESSE]   Einladungs-Datei (.vapafane) schreiben
  einloesen CODE|DATEI [--ordner O] [--alle-optionalen] [--ziel launcher|modrinth]
            [--einstellungen-von ORDNER] [--neue-instanz] [--shader]
                                    Client für eine Einladung einrichten (Launcher-Profil oder Modrinth-Paket);
                                    Einstellungen (ohne Welten) aus ORDNER übernehmen; mit --neue-instanz wird
                                    eine vorhandene Instanz in der Modrinth-App nicht aktualisiert; --shader
                                    richtet Shader-Unterstützung (Iris) mit ein
  clients                           eingerichtete Clients anzeigen
  client-entfernen ORDNER           Client und sein Launcher-Profil entfernen
  ping ADRESSE                      Server-Status abfragen (wie die Serverliste im Spiel)
  erreichbarkeit    prüfen, ob der Server von außen erreichbar ist (IPv4/IPv6, DS-Lite)
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
            ops::check_port(&dir)?;
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
        "aktiv" => {
            let want = args
                .get(1)
                .ok_or_else(|| crate::util::Error("Bitte Namen angeben".into()))?;
            let mut cfg = Config::load();
            let entry = cfg
                .servers
                .iter()
                .find(|s| &s.name == want || &s.path == want)
                .cloned()
                .ok_or_else(|| crate::util::Error(format!("„{want}“ ist nicht in der Liste")))?;
            if !matches!(state().as_str(), "stopped" | "failed" | "none") {
                return err("Umschalten geht nur, wenn der Server gestoppt ist.");
            }
            cfg.set_active(Some(std::path::Path::new(&entry.path)))?;
            println!("Aktiv: {} ({})", entry.name, entry.path);
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
        "mod-suche" | "mod-install" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            let what = if args.iter().any(|a| a == "--datapack") {
                crate::catalog::What::Datapacks
            } else {
                crate::catalog::What::Mods
            };
            let rest: Vec<&str> = args[1..]
                .iter()
                .map(String::as_str)
                .filter(|a| *a != "--datapack")
                .collect();
            if cmd == "mod-suche" {
                let r = crate::catalog::search(&dir, what, &rest.join(" "), 0, Some(0))?;
                let have = crate::catalog::installed_projects(&dir)?;
                println!(
                    "{} Treffer für Minecraft {} ({}):",
                    r["total"],
                    r["mc"].as_str().unwrap_or(""),
                    r["type"].as_str().unwrap_or("")
                );
                for h in r["hits"].as_array().into_iter().flatten() {
                    let id = h["id"].as_str().unwrap_or("");
                    println!(
                        "{:<10} {:<28} {:>11} Downloads  {}{}",
                        h["slug"].as_str().unwrap_or(""),
                        h["title"].as_str().unwrap_or(""),
                        h["downloads"],
                        if h["client_required"].as_bool().unwrap_or(false) {
                            "Server + Client"
                        } else {
                            "nur Server"
                        },
                        if have.contains(id) { "  [installiert]" } else { "" }
                    );
                }
                for h in r["cf_hits"].as_array().into_iter().flatten() {
                    println!(
                        "{:<10} {:<28} {:>11} Downloads  CurseForge{}",
                        h["id"].as_str().unwrap_or(""),
                        h["title"].as_str().unwrap_or(""),
                        h["downloads"],
                        if have.contains(h["id"].as_str().unwrap_or("")) {
                            "  [installiert]"
                        } else {
                            ""
                        }
                    );
                }
                if let Some(e) = r["cf_error"].as_str() {
                    println!("CurseForge: {e}");
                }
            } else {
                let project = rest
                    .first()
                    .ok_or_else(|| crate::util::Error("Bitte Projekt angeben".into()))?;
                let _lock = ops::JobLock::acquire("Mod installieren")?;
                let print = |f: f64, t: &str| eprintln!("[{:>3.0} %] {t}", f * 100.0);
                let r = crate::catalog::install(&dir, what, project, &print)?;
                println!("{}", serde_json::to_string_pretty(&r)?);
            }
        }
        "curseforge-schluessel" => {
            if args.get(1).map(String::as_str) == Some("--loeschen") {
                crate::curseforge::delete_key()?;
                println!("CurseForge-Schlüssel gelöscht.");
                return Ok(());
            }
            let mut k = String::new();
            std::io::stdin().read_line(&mut k)?;
            crate::curseforge::save_key(&k)?;
            println!("Gespeichert ({}).", crate::curseforge::masked().unwrap_or_default());
        }
        "curseforge-test" => {
            let k = crate::curseforge::key()
                .ok_or_else(|| crate::util::Error("Kein CurseForge-Schlüssel hinterlegt.".into()))?;
            crate::curseforge::test_key(&k)?;
            println!(
                "CurseForge-Schlüssel funktioniert ({}).",
                crate::curseforge::masked().unwrap_or_default()
            );
        }
        "einladung" | "einladung-datei" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server".into()))?;
            let (dest, addr) = if cmd == "einladung-datei" {
                (args.get(1).cloned(), args.get(2).cloned())
            } else {
                (None, args.get(1).cloned())
            };
            let addr = addr.unwrap_or_else(|| crate::invite::lan_address(&dir));
            let built = crate::invite::build(&dir, &addr)?;
            if !built.server_only.is_empty() {
                eprintln!(
                    "Nur auf dem Server (nicht in der Einladung): {}",
                    built.server_only.join(", ")
                );
            }
            match dest {
                Some(d) => {
                    let inv = crate::invite::write_file(&built, std::path::Path::new(&d))?;
                    println!("{d}: {} Mods ({} eingepackt)", inv.mods.len(), built.files.len());
                }
                None => {
                    if !built.files.is_empty() {
                        let names: Vec<&str> = built.files.iter().map(|(t, _)| t.as_str()).collect();
                        eprintln!(
                            "Ohne Quelle (nur in der Einladungs-Datei möglich): {}",
                            names.join(", ")
                        );
                    }
                    let code = crate::invite::encode(&built.invite)?;
                    eprintln!("{} Mods, {} Zeichen", built.invite.mods.len(), code.len());
                    println!("{code}");
                }
            }
        }
        "einloesen" => {
            let what = args
                .get(1)
                .ok_or_else(|| crate::util::Error("Bitte Code oder Datei angeben".into()))?;
            let folder = args
                .iter()
                .position(|a| a == "--ordner")
                .and_then(|i| args.get(i + 1))
                .map(std::path::PathBuf::from);
            let path = std::path::Path::new(what);
            let (inv, zip) = if path.is_file() {
                (crate::invite::read_file(path)?, Some(path))
            } else {
                (crate::invite::decode(what)?, None)
            };
            let pv = crate::client::preview(&inv, zip)?;
            println!(
                "Server: {} ({}) – Minecraft {} – {} {}",
                inv.name, inv.address, inv.mc, inv.loader, inv.loader_version
            );
            println!("Status: {}", pv["ping"]);
            for m in pv["mods"].as_array().into_iter().flatten() {
                println!(
                    "  {:<34} {:<16} {:>9}  {}{}",
                    m["title"].as_str().unwrap_or(""),
                    m["version"].as_str().unwrap_or(""),
                    crate::util::fmt_bytes(m["size"].as_u64().unwrap_or(0)),
                    m["source"].as_str().unwrap_or(""),
                    if m["optional"].as_bool().unwrap_or(false) {
                        " (optional)"
                    } else {
                        ""
                    }
                );
            }
            let selected: std::collections::HashSet<String> = if args.iter().any(|a| a == "--alle-optionalen") {
                inv.mods.iter().map(|m| m.id.clone()).collect()
            } else {
                Default::default()
            };
            let print = |f: f64, t: &str| eprintln!("[{:>3.0} %] {t}", f * 100.0);
            let modrinth = args
                .iter()
                .position(|a| a == "--ziel")
                .and_then(|i| args.get(i + 1))
                .map(String::as_str)
                == Some("modrinth");
            let take = args
                .iter()
                .position(|a| a == "--einstellungen-von")
                .and_then(|i| args.get(i + 1))
                .map(|d| crate::transfer::Take {
                    dir: std::path::PathBuf::from(d),
                    parts: crate::transfer::PARTS
                        .iter()
                        .map(|(k, _)| k.to_string())
                        .filter(|k| k != "worlds")
                        .collect(),
                });
            let update = !args.iter().any(|a| a == "--neue-instanz");
            let shaders = args.iter().any(|a| a == "--shader");
            let choices = crate::client::Choices { selected, take: take.as_ref(), shaders };
            let r = if modrinth {
                crate::client::install_mrpack(&inv, zip, &choices, update, true, &print)?
            } else {
                crate::client::install(&inv, zip, folder.as_deref(), &choices, &print)?
            };
            println!("{}", serde_json::to_string_pretty(&r)?);
        }
        "erreichbarkeit" => {
            let dir = Config::load()
                .active_path()
                .ok_or_else(|| crate::util::Error("Kein aktiver Server.".into()))?;
            let r = crate::reach::check(&dir)?;
            println!("{}: {}\n{}\nIPv4: {}\nIPv6: {}", r.light, r.title, r.text, r.v4, r.v6);
        }
        "clients" => println!("{}", serde_json::to_string_pretty(&crate::client::list())?),
        "client-entfernen" => {
            let d = args
                .get(1)
                .ok_or_else(|| crate::util::Error("Bitte Ordner angeben".into()))?;
            crate::client::remove(std::path::Path::new(d))?;
            println!("Entfernt.");
        }
        "ping" => {
            let a = args
                .get(1)
                .ok_or_else(|| crate::util::Error("Bitte Adresse angeben".into()))?;
            println!("{}", crate::client::ping(a)?);
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
