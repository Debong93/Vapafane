//! Fenster-Befehle für den Update-Assistenten: Prüfen, Update, Rückweg,
//! Probelauf an einer Kopie und Export als .mrpack.

use serde_json::{Value, json};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::archive::{self, Progress};
use crate::config::{ALT_DIR_NAME, Config, STAGING_NAME};
use crate::gui::{Cmd, active_dir, blocking, run_task, stop_and_wait};
use crate::ops::server_state;
use crate::procs;
use crate::rcon::Rcon;
use crate::server;
use crate::updater::{self, Plan};
use crate::util::{self, Error, Result, err};
use crate::{catalog, cleanup, ipc, launcher, mods};

/// Zuletzt geprüfter Plan (das Update nutzt genau diesen).
static PLAN: Mutex<Option<Plan>> = Mutex::new(None);

fn take_plan(dir: &Path) -> Result<Plan> {
    let plan = crate::util::lock(&PLAN)
        .clone()
        .ok_or_else(|| Error("Bitte zuerst „Prüfen“ drücken.".into()))?;
    if util::canonical(&plan.dir) != util::canonical(dir) {
        return err("Die Prüfung gehört zu einem anderen Server. Bitte neu prüfen.");
    }
    Ok(plan)
}

fn state() -> String {
    server_state(&Config::load())
        .get("state")
        .and_then(|s| s.as_str())
        .unwrap_or("stopped")
        .to_string()
}

#[tauri::command]
pub async fn upd_info() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let mut v = updater::current_info(&dir);
        let last = cfg.extra.get("last_update").cloned().unwrap_or(Value::Null);
        let rollback = last
            .get("path")
            .and_then(|p| p.as_str())
            .map(|p| util::canonical(Path::new(p)))
            == Some(util::canonical(&dir))
            && last
                .get("backup")
                .and_then(|b| b.as_str())
                .map(|b| Path::new(b).is_file())
                .unwrap_or(false);
        v["rollback"] = json!(rollback);
        v["last_update"] = last;
        Ok(v)
    })
    .await
}

#[tauri::command]
pub async fn upd_versions() -> Cmd<Vec<String>> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let info = server::detect_server(&dir);
        let kind = crate::flavor::Kind::parse(&info.kind)
            .ok_or_else(|| Error("Den Update-Assistenten gibt es für diesen Server-Typ nicht.".into()))?;
        updater::target_versions(kind, &info.version)
    })
    .await
}

#[tauri::command]
pub fn upd_check(app: AppHandle, target: String) -> Cmd<()> {
    *crate::util::lock(&PLAN) = None;
    run_task(&app, &format!("Prüfe Mods für {target}"), move |progress| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let plan = updater::check(&dir, &target, progress)?;
        let out = json!({"plan": plan, "has_work": plan.has_work()});
        *crate::util::lock(&PLAN) = Some(plan);
        Ok(out)
    })
    .map_err(|e| e.0)
}

/// Spieler online? Dann mit Countdown stoppen.
fn players_online(dir: &Path) -> bool {
    Rcon::new(dir)
        .command("list")
        .map(|l| !l.contains("There are 0 "))
        .unwrap_or(false)
}

/// Startet den Server über den Dienst und wartet, bis er bereit ist.
fn start_and_wait(progress: Progress, from: f64) -> Result<()> {
    launcher::ensure_service()?;
    ipc::call("start", Value::Null)?;
    let started = Instant::now();
    std::thread::sleep(Duration::from_secs(2));
    while started.elapsed() < Duration::from_secs(900) {
        let st = server_state(&Config::load());
        match st.get("state").and_then(|s| s.as_str()).unwrap_or("") {
            "running" => return Ok(()),
            "failed" | "restarting" => {
                let why = st.get("last_exit").and_then(|l| l.as_str()).unwrap_or("").to_string();
                return err(format!("Der Server ist beim Start abgestürzt. {why}"));
            }
            "stopped" => return err("Der Server hat sich direkt wieder beendet."),
            _ => {}
        }
        let last = st
            .get("output")
            .and_then(|o| o.as_array())
            .and_then(|o| o.last())
            .and_then(|l| l.as_str())
            .unwrap_or("Server startet …");
        progress(
            from,
            &last
                .chars()
                .rev()
                .take(90)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<String>(),
        );
        std::thread::sleep(Duration::from_secs(2));
    }
    err("Der Server war nach 15 Minuten noch nicht fertig gestartet.")
}

#[tauri::command]
pub fn upd_run(app: AppHandle, disable_missing: bool, disable_unknown: bool) -> Cmd<()> {
    run_task(&app, "Update", move |progress| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let plan = take_plan(&dir)?;
        let r = do_update(&dir, &plan, disable_missing, disable_unknown, progress)?;
        *crate::util::lock(&PLAN) = None;
        Ok(r)
    })
    .map_err(|e| e.0)
}

/// Ganzes Update: stoppen (mit Warnung) → Backup → laden/prüfen/tauschen → starten und warten.
/// Fehler beim Start kommen als `ok: false` zurück, damit man den Rückweg anbieten kann.
pub fn do_update(
    dir: &Path,
    plan: &Plan,
    disable_missing: bool,
    disable_unknown: bool,
    progress: Progress,
) -> Result<Value> {
    let what = if plan.change_version {
        format!("Minecraft {} → {}", plan.current, plan.target)
    } else {
        "Mods aktualisieren".into()
    };
    let st = state();
    if st == "external" {
        return err("Der Server läuft außerhalb von Vapafane. Bitte zuerst dort stoppen.");
    }
    if matches!(st.as_str(), "running" | "starting") && players_online(dir) {
        progress(0.0, "Warne Spieler (30 Sekunden) …");
        let _ = Rcon::new(dir).command("say Server startet für ein Update neu!");
        ipc::call("stop", json!({"warn": 30}))?;
    }
    stop_and_wait(progress)?;

    progress(0.0, "Erstelle Backup …");
    let cfg = Config::load();
    let backup = archive::backup_server(dir, &cfg.name_of(dir), &cfg, &|f, t| progress(f * 0.5, t))?;
    let mut cfg = Config::load();
    cfg.extra.insert(
        "last_update".into(),
        json!({"path": dir, "backup": backup, "date": chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
               "target": plan.target}),
    );
    cfg.save()?;

    let alt = updater::apply(dir, plan, disable_missing, disable_unknown, &|f, t| {
        progress(0.5 + f * 0.3, t)
    })?;
    progress(0.8, "Starte Server (der erste Start kann ein paar Minuten dauern) …");
    let res = start_and_wait(progress, 0.85);
    // Erst nach geglücktem Start aufräumen – bei Fehlern bleibt alles für den Rückweg liegen
    let mut cleaned = json!(null);
    if res.is_ok() {
        progress(0.98, "Räume alte Dateien auf …");
        let loader = plan
            .server
            .as_ref()
            .map(|a| a.build.clone())
            .unwrap_or_else(|| server::detect_server(dir).loader);
        let items = cleanup::after_update(dir, &alt, &plan.target, &loader);
        let (freed, failed) = cleanup::delete(dir, &items);
        cleaned = json!({"count": items.len() - failed.len(), "freed": freed, "failed": failed});
    }
    Ok(json!({
        "ok": res.is_ok(),
        "error": res.err().map(|e| e.0),
        "backup": backup,
        "alt": alt,
        "what": what,
        "cleaned": cleaned,
    }))
}

// ---------------------------------------------------------------- Mod-Liste und Aufräumen

#[tauri::command]
pub async fn mods_list() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        Ok(mods::list(&dir))
    })
    .await
}

/// Mod/Datapack ein- oder ausschalten. Läuft der Server, wirkt es erst nach einem Neustart
/// (unter Windows lässt sich eine geladene Mod dann gar nicht umbenennen).
#[tauri::command]
pub async fn mod_set_enabled(rel: String, on: bool) -> Cmd<Value> {
    blocking(move || {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let running = !matches!(state().as_str(), "stopped" | "failed" | "none");
        if let Err(e) = mods::set_enabled(&dir, &rel, on) {
            if running {
                return err(format!("{e}\nBitte erst den Server stoppen."));
            }
            return Err(e);
        }
        Ok(json!({"restart_needed": running}))
    })
    .await
}

#[tauri::command]
pub async fn cleanup_preview() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let items = cleanup::candidates(&dir);
        let total: u64 = items.iter().map(|c| c.size).sum();
        Ok(json!({"items": items, "total": total}))
    })
    .await
}

#[tauri::command]
pub fn cleanup_run(app: AppHandle) -> Cmd<()> {
    run_task(&app, "Aufräumen", |progress| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        if !matches!(state().as_str(), "stopped" | "failed" | "none") {
            return err("Aufräumen geht nur, wenn der Server gestoppt ist.");
        }
        progress(0.3, "Lösche alte Dateien …");
        let items = cleanup::candidates(&dir);
        let (freed, failed) = cleanup::delete(&dir, &items);
        Ok(json!({"count": items.len() - failed.len(), "freed": freed, "failed": failed}))
    })
    .map_err(|e| e.0)
}

/// Zurück zum Stand vor dem letzten Update (über dessen Backup).
#[tauri::command]
pub fn upd_rollback(app: AppHandle) -> Cmd<()> {
    run_task(&app, "Zurück zum Stand vor dem Update", |progress| {
        let mut cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let backup = cfg
            .extra
            .get("last_update")
            .filter(|l| {
                l.get("path")
                    .and_then(|p| p.as_str())
                    .map(|p| util::canonical(Path::new(p)))
                    == Some(util::canonical(&dir))
            })
            .and_then(|l| l.get("backup").and_then(|b| b.as_str()).map(PathBuf::from))
            .filter(|b| b.is_file())
            .ok_or_else(|| Error("Für diesen Server gibt es kein Backup vom letzten Update.".into()))?;
        stop_and_wait(progress)?;
        let aside = archive::restore_backup(&backup, &dir, progress)?;
        server::prepare_server(&dir, false)?;
        cfg.extra.remove("last_update");
        cfg.save()?;
        Ok(json!({"aside": aside, "label": archive::backup_label(&backup)}))
    })
    .map_err(|e| e.0)
}

// ---------------------------------------------------------------- Probelauf

fn copy_server(src: &Path, dest: &Path, progress: Progress) -> Result<()> {
    let skip = [ALT_DIR_NAME, STAGING_NAME, "logs", "crash-reports"];
    let entries: Vec<walkdir::DirEntry> = walkdir::WalkDir::new(src)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| e.depth() != 1 || !skip.contains(&e.file_name().to_string_lossy().as_ref()))
        .filter_map(|e| e.ok())
        .collect();
    let total: u64 = entries
        .iter()
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum::<u64>()
        .max(1);
    let mut done = 0u64;
    for e in entries {
        let rel = e.path().strip_prefix(src).unwrap_or(e.path());
        let target = dest.join(rel);
        if e.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else if e.file_type().is_file() {
            if let Some(p) = target.parent() {
                fs::create_dir_all(p)?;
            }
            // Session-Lock der laufenden Welt nicht mitnehmen
            if e.file_name() == "session.lock" {
                continue;
            }
            done += fs::copy(e.path(), &target)?;
            progress(done as f64 / total as f64, &format!("Kopiere {}", rel.display()));
        }
    }
    Ok(())
}

/// Aktualisiert eine Kopie im Temp-Ordner, startet sie auf einem anderen Port,
/// prüft, ob sie fertig startet, stoppt sie und löscht die Kopie wieder.
#[tauri::command]
pub fn upd_trial(app: AppHandle, disable_missing: bool, disable_unknown: bool) -> Cmd<()> {
    run_task(&app, "Probelauf", move |progress| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let plan = take_plan(&dir)?;
        do_trial(&dir, &plan, disable_missing, disable_unknown, progress)
    })
    .map_err(|e| e.0)
}

pub fn do_trial(
    dir: &Path,
    plan: &Plan,
    disable_missing: bool,
    disable_unknown: bool,
    progress: Progress,
) -> Result<Value> {
    let base = std::env::temp_dir().join(format!("vapafane-probelauf-{}", util::now_stamp()));
    let size = util::dir_size(dir);
    fs::create_dir_all(&base)?;
    if util::free_space(&base) < size + size / 5 + 1024 * 1024 * 1024 {
        let _ = fs::remove_dir_all(&base);
        return err(format!(
            "Zu wenig Platz im Temp-Ordner für eine Kopie ({} nötig).",
            util::fmt_bytes(size + size / 5)
        ));
    }
    let copy = base.join(dir.file_name().unwrap_or_default());
    let result = trial_inner(dir, &copy, plan, disable_missing, disable_unknown, progress);
    progress(1.0, "Lösche die Kopie …");
    let deleted = fs::remove_dir_all(&base).is_ok();
    let mut r = result?;
    r["copy_deleted"] = json!(deleted);
    r["copy"] = json!(base);
    Ok(r)
}

fn trial_inner(
    dir: &Path,
    copy: &Path,
    plan: &Plan,
    disable_missing: bool,
    disable_unknown: bool,
    progress: Progress,
) -> Result<Value> {
    copy_server(dir, copy, &|f, t| progress(f * 0.4, t))?;

    // Andere Ports, damit der echte Server weiterlaufen kann
    let props = server::read_properties(copy);
    let used: Vec<u16> = ["server-port", "rcon.port", "query.port"]
        .iter()
        .filter_map(|k| props.get(*k).and_then(|p| p.parse().ok()))
        .collect();
    let game_port = procs::free_port(25566, &used);
    let rcon_port = procs::free_port(25576, &[game_port]);
    let query_port = procs::free_port(25567, &[game_port, rcon_port]);
    let mut upd = std::collections::BTreeMap::new();
    upd.insert("server-port".to_string(), game_port.to_string());
    upd.insert("rcon.port".to_string(), rcon_port.to_string());
    upd.insert("query.port".to_string(), query_port.to_string());
    upd.insert("enable-rcon".to_string(), "true".to_string());
    if props
        .get("management-server-port")
        .map(|p| p != "0" && !p.is_empty())
        .unwrap_or(false)
    {
        let mport = procs::free_port(25568, &[game_port, rcon_port, query_port]);
        upd.insert("management-server-port".to_string(), mport.to_string());
    }
    if props.get("rcon.password").map(|p| p.is_empty()).unwrap_or(true) {
        upd.insert("rcon.password".to_string(), util::random_hex(12));
    }
    server::write_properties(copy, &upd)?;

    // Nicht mehr Arbeitsspeicher nehmen als gerade frei ist
    let env = server::read_env(copy);
    let want = server::xmx_gb(&env).unwrap_or(4);
    let avail_gb = procs::available_memory() / 1024 / 1024 / 1024;
    let xmx = want.min(avail_gb.saturating_sub(2)).max(1);
    server::write_env(copy, &[("MC_XMX", format!("{xmx}G")), ("MC_XMS", "1G".into())])?;

    let mut plan = plan.clone();
    plan.dir = copy.to_path_buf();
    updater::apply(copy, &plan, disable_missing, disable_unknown, &|f, t| {
        progress(0.4 + f * 0.2, t)
    })?;

    progress(0.6, &format!("Starte die Kopie auf Port {game_port} …"));
    let (java, args) = server::launch_command(copy)?;
    let mut child = util::command(&java)
        .args(&args)
        .current_dir(copy)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error(format!("Java konnte nicht gestartet werden ({java}): {e}")))?;
    let lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    for stream in [
        child
            .stdout
            .take()
            .map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
        child
            .stderr
            .take()
            .map(|s| Box::new(s) as Box<dyn std::io::Read + Send>),
    ]
    .into_iter()
    .flatten()
    {
        let lines = lines.clone();
        std::thread::spawn(move || {
            for l in BufReader::new(stream).split(b'\n').map_while(|l| l.ok()) {
                crate::util::lock(&lines).push(String::from_utf8_lossy(&l).trim_end().to_string());
            }
        });
    }

    let started = Instant::now();
    let mut ready = None;
    let mut exited = None;
    while started.elapsed() < Duration::from_secs(900) {
        std::thread::sleep(Duration::from_secs(1));
        if let Ok(Some(status)) = child.try_wait() {
            exited = Some(status.code());
            break;
        }
        let l = crate::util::lock(&lines);
        if l.iter().any(|x| x.contains("]: Done (")) {
            ready = Some(started.elapsed().as_secs());
            break;
        }
        if let Some(last) = l.last() {
            let short: String = last.chars().take(100).collect();
            progress(0.6 + (started.elapsed().as_secs_f64() / 900.0) * 0.3, &short);
        }
    }

    if exited.is_none() {
        progress(0.92, "Stoppe die Kopie …");
        let mut stopped = Rcon::new(copy).command("stop").is_ok();
        if !stopped && let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            stopped = stdin.write_all(b"stop\n").is_ok();
        }
        let end = Instant::now() + Duration::from_secs(if stopped { 180 } else { 5 });
        while Instant::now() < end {
            if let Ok(Some(_)) = child.try_wait() {
                break;
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        let _ = child.kill();
        let _ = child.wait();
    }

    let all = crate::util::lock(&lines).clone();
    let problems: Vec<String> = all
        .iter()
        .filter(|l| l.contains("/ERROR]") || l.contains("Exception") || l.contains("Incompatible mod"))
        .take(15)
        .cloned()
        .collect();
    let tail: Vec<String> = all.iter().rev().take(25).rev().cloned().collect();
    let mods_line = all
        .iter()
        .find(|l| l.contains("Loading ") && l.contains(" mods:"))
        .cloned();
    let version_line = all
        .iter()
        .find(|l| l.contains("Loading Minecraft ") || l.contains("Starting minecraft server version"))
        .cloned();
    Ok(json!({
        "ok": ready.is_some(),
        "seconds": ready,
        "exit_code": exited.flatten(),
        "timeout": ready.is_none() && exited.is_none(),
        "problems": problems,
        "tail": if ready.is_some() { vec![] } else { tail },
        "mods_line": mods_line,
        "version_line": version_line,
        "port": game_port,
        "xmx": xmx,
    }))
}

// ---------------------------------------------------------------- Export

#[tauri::command]
pub async fn export_mrpack(app: AppHandle) -> Cmd<Option<Value>> {
    blocking(move || {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let name = cfg.name_of(&dir);
        let file = format!(
            "{}_{}.mrpack",
            util::safe_name(&name),
            chrono::Local::now().format("%Y-%m-%d")
        );
        let Some(dest) = app
            .dialog()
            .file()
            .set_title("Modpack speichern")
            .set_directory(crate::config::home())
            .set_file_name(&file)
            .add_filter("Modrinth-Modpack", &["mrpack"])
            .blocking_save_file()
            .and_then(|p| p.into_path().ok())
        else {
            return Ok(None);
        };
        let (files, overrides) = updater::export_mrpack(&dir, &dest, &name)?;
        Ok(Some(json!({"path": dest, "files": files, "overrides": overrides})))
    })
    .await
}

// ---------------------------------------------------------------- Mods hinzufügen

fn what_of(datapacks: bool) -> catalog::What {
    if datapacks {
        catalog::What::Datapacks
    } else {
        catalog::What::Mods
    }
}

#[tauri::command]
pub async fn catalog_search(query: String, offset: u32, cf_offset: Option<u32>, datapacks: bool) -> Cmd<Value> {
    blocking(move || {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        catalog::search(&dir, what_of(datapacks), &query, offset, cf_offset)
    })
    .await
}

#[tauri::command]
pub async fn catalog_installed() -> Cmd<Vec<String>> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        Ok(catalog::installed_projects(&dir)?.into_iter().collect())
    })
    .await
}

#[tauri::command]
pub fn catalog_install(app: AppHandle, project: String, title: String, datapacks: bool) -> Cmd<()> {
    run_task(&app, &format!("Installiere {title}"), move |progress| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let mut r = catalog::install(&dir, what_of(datapacks), &project, progress)?;
        r["restart_needed"] = json!(!matches!(state().as_str(), "stopped" | "failed" | "none"));
        Ok(r)
    })
    .map_err(|e| e.0)
}
