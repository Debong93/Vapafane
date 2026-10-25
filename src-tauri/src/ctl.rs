//! Befehlszeile: `vapafane --ctl <befehl>` – für Skripte und zum Testen.

use serde_json::Value;
use std::time::Duration;

use crate::config::Config;
use crate::util::{Result, err};
use crate::{ipc, launcher, ops, server};

fn help() -> String {
    crate::sprache::t!("ctl-hilfe")
}

pub fn run(args: &[String]) -> i32 {
    match run_inner(args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", crate::sprache::t!("ctl-fehler", fehler = e.0.clone()));
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
            return err(crate::sprache::t!("ueb-ev-absturz"));
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    err(crate::sprache::t!("ctl-timeout", zustand = state()))
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
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
            if !server::eula_accepted(&dir) {
                return err(crate::sprache::t!("svc-err-eula"));
            }
            ops::check_port(&dir)?;
            launcher::ensure_service()?;
            ipc::call("start", Value::Null)?;
            println!("{}", crate::sprache::t!("ctl-startet"));
        }
        "stop" | "restart" => {
            ops::stop_server(secs(0), cmd == "restart")?;
            println!(
                "{}",
                if cmd == "stop" {
                    crate::sprache::t!("ctl-stoppt")
                } else {
                    crate::sprache::t!("ctl-neustart")
                }
            );
        }
        "wait-ready" => wait_for(&["running"], secs(600))?,
        "wait-stopped" => wait_for(&["stopped", "none"], secs(240))?,
        "backup" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
            let _lock = ops::JobLock::acquire(&crate::sprache::t!("task-backup-ctl"))?;
            let running = state() == "running";
            let progress = |_: f64, _: &str| {};
            let path = ops::backup_with_save(&dir, &cfg, running, &progress)?;
            println!("{}", path.display());
        }
        "cmd" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
            println!("{}", crate::rcon::once(&dir, &args[1..].join(" "))?);
        }
        "aktiv" => {
            let want = args
                .get(1)
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-name")))?;
            let mut cfg = Config::load();
            let entry = cfg
                .servers
                .iter()
                .find(|s| &s.name == want || &s.path == want)
                .cloned()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-nicht-in-liste", name = want.to_string())))?;
            if !matches!(state().as_str(), "stopped" | "failed" | "none") {
                return err(crate::sprache::t!("err-umschalten-laeuft"));
            }
            cfg.set_active(Some(std::path::Path::new(&entry.path)))?;
            println!("{}", crate::sprache::t!("ctl-aktiv", name = entry.name.clone(), pfad = entry.path.clone()));
        }
        "neu" => {
            let (Some(kind), Some(name), Some(version)) = (args.get(1), args.get(2), args.get(3)) else {
                return err(crate::sprache::t!("ctl-neu-benutzung"));
            };
            let kind = crate::flavor::Kind::parse(kind).ok_or_else(|| crate::util::Error(crate::sprache::t!("err-typ-unbekannt")))?;
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
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
            let target = args
                .get(1)
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-version")))?;
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
                "{}",
                crate::sprache::t!(
                    "ctl-update-plan",
                    von = plan.current.to_string(),
                    nach = plan.target.to_string(),
                    java = plan.java_required.map(|v| v.to_string()).unwrap_or_else(|| "–".into()),
                    javaok = plan.java_ok.to_string(),
                    platzok = plan.space_ok.to_string(),
                    tun = plan.has_work().to_string()
                )
            );
            if cmd == "update-check" {
                return Ok(());
            }
            if !plan.java_ok || !plan.space_ok {
                return err(crate::sprache::t!("ctl-java-platz"));
            }
            let _lock = ops::JobLock::acquire(&if cmd == "trial" {
                crate::sprache::t!("upd-trial")
            } else {
                crate::sprache::t!("task-update")
            })?;
            let r = if cmd == "trial" {
                crate::updates_gui::do_trial(&dir, &plan, true, false, &print)?
            } else {
                crate::updates_gui::do_update(&dir, &plan, true, false, &print)?
            };
            println!("{}", serde_json::to_string_pretty(&r)?);
        }
        "mods" | "mod-an" | "mod-aus" | "mod-entfernen" | "aufraeumen" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
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
                                crate::sprache::t!("set-an")
                            } else {
                                crate::sprache::t!("set-aus").to_uppercase()
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
                        .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-pfad")))?;
                    crate::mods::set_enabled(&dir, rel, cmd == "mod-an")?;
                    println!("{rel}: {}", if cmd == "mod-an" { crate::sprache::t!("set-an") } else { crate::sprache::t!("set-aus") });
                }
                "mod-entfernen" => {
                    let rel = args
                        .get(1)
                        .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-pfad")))?;
                    crate::mods::remove(&dir, rel)?;
                    println!("{}", crate::sprache::t!("mods-ev-entfernt", name = rel.to_string()));
                }
                _ => {
                    let items = crate::cleanup::candidates(&dir);
                    for c in &items {
                        println!("{:>10}  {:<55} {}", crate::util::fmt_bytes(c.size), c.rel, c.what);
                    }
                    let total: u64 = items.iter().map(|c| c.size).sum();
                    println!("{}", crate::sprache::t!("ctl-zusammen", groesse = crate::util::fmt_bytes(total)));
                    if args.get(1).map(String::as_str) == Some("--ja") {
                        if !matches!(state().as_str(), "stopped" | "failed" | "none") {
                            return err(crate::sprache::t!("cln-err-laeuft"));
                        }
                        let (freed, failed) = crate::cleanup::delete(&dir, &items);
                        println!(
                            "{}",
                            crate::sprache::t!("ctl-geloescht", groesse = crate::util::fmt_bytes(freed), fehler = format!("{failed:?}"))
                        );
                    }
                }
            }
        }
        "mod-suche" | "mod-install" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
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
                    "{}",
                    crate::sprache::t!(
                        "ctl-treffer",
                        anzahl = r["total"].to_string(),
                        mc = r["mc"].as_str().unwrap_or("").to_string(),
                        typ = r["type"].as_str().unwrap_or("").to_string()
                    )
                );
                for h in r["hits"].as_array().into_iter().flatten() {
                    let id = h["id"].as_str().unwrap_or("");
                    println!(
                        "{:<10} {:<28} {:>11} Downloads  {}{}",
                        h["slug"].as_str().unwrap_or(""),
                        h["title"].as_str().unwrap_or(""),
                        h["downloads"],
                        if h["client_required"].as_bool().unwrap_or(false) {
                            crate::sprache::t!("kat-beides")
                        } else {
                            crate::sprache::t!("kat-nur-server")
                        },
                        if have.contains(id) {
                            format!("  [{}]", crate::sprache::t!("kat-installiert"))
                        } else {
                            String::new()
                        }
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
                    .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-projekt")))?;
                let _lock = ops::JobLock::acquire(&crate::sprache::t!("task-mod-installieren"))?;
                let print = |f: f64, t: &str| eprintln!("[{:>3.0} %] {t}", f * 100.0);
                let r = crate::catalog::install(&dir, what, project, &print)?;
                println!("{}", serde_json::to_string_pretty(&r)?);
            }
        }
        "curseforge-schluessel" => {
            if args.get(1).map(String::as_str) == Some("--loeschen") {
                crate::curseforge::delete_key()?;
                println!("{}", crate::sprache::t!("ctl-cf-geloescht"));
                return Ok(());
            }
            let mut k = String::new();
            std::io::stdin().read_line(&mut k)?;
            crate::curseforge::save_key(&k)?;
            println!("{}", crate::sprache::t!("ctl-gespeichert", schluessel = crate::curseforge::masked().unwrap_or_default()));
        }
        "curseforge-test" => {
            let k = crate::curseforge::key()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("cf-err-fehlt")))?;
            crate::curseforge::test_key(&k)?;
            println!(
                "{}",
                crate::sprache::t!("ctl-cf-ok", schluessel = crate::curseforge::masked().unwrap_or_default())
            );
        }
        "einladung" | "einladung-datei" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
            let (dest, addr) = if cmd == "einladung-datei" {
                (args.get(1).cloned(), args.get(2).cloned())
            } else {
                (None, args.get(1).cloned())
            };
            let addr = addr.unwrap_or_else(|| crate::invite::lan_address(&dir));
            let built = crate::invite::build(&dir, &addr)?;
            if !built.server_only.is_empty() {
                eprintln!(
                    "{}",
                    crate::sprache::t!("ctl-nur-server", mods = built.server_only.join(", "))
                );
            }
            match dest {
                Some(d) => {
                    let inv = crate::invite::write_file(&built, std::path::Path::new(&d))?;
                    println!(
                        "{}",
                        crate::sprache::t!(
                            "ctl-einladung-gespeichert",
                            datei = d.clone(),
                            mods = inv.mods.len(),
                            dateien = built.files.len()
                        )
                    );
                }
                None => {
                    if !built.files.is_empty() {
                        let names: Vec<&str> = built.files.iter().map(|(t, _)| t.as_str()).collect();
                        eprintln!(
                            "{}",
                            crate::sprache::t!("ctl-ohne-quelle", mods = names.join(", "))
                        );
                    }
                    let code = crate::invite::encode(&built.invite)?;
                    eprintln!("{}", crate::sprache::t!("ctl-code-laenge", mods = built.invite.mods.len(), zeichen = code.len()));
                    println!("{code}");
                }
            }
        }
        "einloesen" => {
            let what = args
                .get(1)
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-code")))?;
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
                        .map(|k| k.to_string())
                        .filter(|k| k != "worlds")
                        .collect(),
                });
            let update = !args.iter().any(|a| a == "--neue-instanz");
            let choices = crate::client::Choices {
                selected,
                take: take.as_ref(),
            };
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
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
            let r = crate::reach::check(&dir)?;
            println!("{}: {}\n{}\nIPv4: {}\nIPv6: {}", r.light, r.title, r.text, r.v4, r.v6);
        }
        "clients" => println!("{}", serde_json::to_string_pretty(&crate::client::list())?),
        "client-entfernen" => {
            let d = args
                .get(1)
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-ordner")))?;
            crate::client::remove(std::path::Path::new(d))?;
            println!("{}", crate::sprache::t!("ctl-entfernt"));
        }
        "ping" => {
            let a = args
                .get(1)
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-adresse")))?;
            println!("{}", crate::client::ping(a)?);
        }
        "mrpack" => {
            let cfg = Config::load();
            let dir = cfg
                .active_path()
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("err-kein-server")))?;
            let dest = args
                .get(1)
                .ok_or_else(|| crate::util::Error(crate::sprache::t!("ctl-fehlt-zieldatei")))?;
            let (files, overrides) =
                crate::updater::export_mrpack(&dir, std::path::Path::new(dest), &cfg.name_of(&dir))?;
            println!(
                "{dest}: {}, {}",
                crate::sprache::t!("upd-export-mods", anzahl = files),
                crate::sprache::t!("upd-export-eingepackt", anzahl = overrides)
            );
        }
        "quit-service" => {
            ipc::call("quit", Value::Null)?;
            println!("{}", crate::sprache::t!("ctl-dienst-ende"));
        }
        _ => println!("{}", help()),
    }
    Ok(())
}
