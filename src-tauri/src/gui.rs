//! Das Fenster: Befehle, die die Oberfläche (ui/) aufruft.

use serde::Serialize;
use serde_json::{Value, json};
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use regex::Regex;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;

use crate::archive;
use crate::config::Config;
use crate::ipc;
use crate::launcher;
use crate::ops::{self, server_state};
use crate::procs::{self, Meter};
use crate::rcon::Rcon;
use crate::server;
use crate::util::{self, Result, err};

pub struct Gui {
    rcon: Mutex<Option<Rcon>>,
    meter: Mutex<Meter>,
    busy: Arc<Mutex<Option<String>>>,
    log_reset: Arc<AtomicBool>,
}

pub(crate) type Cmd<T> = std::result::Result<T, String>;

/// Blockierende Arbeit außerhalb des UI-Threads ausführen.
pub(crate) async fn blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T> + Send + 'static) -> Cmd<T> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.0)
}

pub(crate) fn active_dir(cfg: &Config) -> Result<PathBuf> {
    cfg.active_path()
        .ok_or_else(|| util::Error("Kein Server ausgewählt.".into()))
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ---------------------------------------------------------------- Status

#[derive(Serialize)]
struct ServerItem {
    name: String,
    path: String,
    active: bool,
}

#[tauri::command]
pub async fn get_status(gui: State<'_, Gui>) -> Cmd<Value> {
    let cfg = Config::load();
    let busy = gui.busy.lock().unwrap().clone();
    let mut st = tauri::async_runtime::spawn_blocking({
        let cfg = cfg.clone();
        move || server_state(&cfg)
    })
    .await
    .map_err(|e| e.to_string())?;

    let active = cfg.active_path();
    let servers: Vec<ServerItem> = cfg
        .servers
        .iter()
        .map(|s| ServerItem {
            name: s.name.clone(),
            path: s.path.clone(),
            active: active
                .as_ref()
                .map(|a| util::canonical(Path::new(&s.path)) == *a)
                .unwrap_or(false),
        })
        .collect();
    st["servers"] = json!(servers);
    st["busy"] = json!(busy);
    if st.get("job").map(|j| j.is_null()).unwrap_or(true) {
        st["job"] = json!(ops::current_job().filter(|j| Some(j) != busy.as_ref()));
    }
    if let Some(dir) = &active {
        let env = server::read_env(dir);
        st["server"] = json!({"name": cfg.name_of(dir), "path": dir});
        st["ram_limit_gb"] = json!(server::xmx_gb(&env));
        st["eula"] = json!(server::eula_accepted(dir));
    }
    let started = st.get("started_at").and_then(|s| s.as_u64()).unwrap_or(0);
    if started > 0 {
        st["uptime"] = json!(now_secs().saturating_sub(started));
    }
    let pid = st.get("pid").and_then(|p| p.as_u64()).map(|p| p as u32);
    let mut meter = gui.meter.lock().unwrap();
    st["ncpu"] = json!(meter.ncpu());
    if let Some(pid) = pid {
        let (cpu, ram) = meter.sample(pid);
        st["cpu"] = json!(cpu);
        st["ram"] = json!(ram);
    }
    Ok(st)
}

/// Langsame Angaben: Ordnergröße, Adresse, Version, Backups.
#[tauri::command]
pub async fn get_details() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let Some(dir) = cfg.active_path() else {
            return Ok(Value::Null);
        };
        let props = server::read_properties(&dir);
        let info = server::detect_server(&dir);
        let backups = archive::list_backups(&cfg, &cfg.name_of(&dir));
        let last = backups.first().and_then(|b| {
            let m = fs::metadata(b).ok()?;
            let age = m.modified().ok()?.elapsed().ok()?.as_secs();
            Some(json!({"age": age, "size": m.len(), "name": b.file_name()?.to_string_lossy()}))
        });
        Ok(json!({
            "size": util::dir_size(&dir),
            "free": util::free_space(&dir),
            "ip": util::local_ip(),
            "port": props.get("server-port").cloned().unwrap_or_else(|| "25565".into()),
            "info": info,
            "mods": server::count_mods(&dir),
            "backups": backups.len(),
            "last_backup": last,
            "backup_dir": cfg.backup_dir_for(&cfg.name_of(&dir)),
            "rcon": props.get("enable-rcon").map(|v| v == "true").unwrap_or(false),
            "total_memory": procs::total_memory(),
        }))
    })
    .await
}

/// Spieler und TPS per RCON.
#[tauri::command]
pub async fn get_live(app: AppHandle) -> Cmd<Value> {
    blocking(move || {
        let gui = app.state::<Gui>();
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let mut guard = gui.rcon.lock().unwrap();
        if guard.as_ref().map(|r| r.dir() != dir.as_path()).unwrap_or(true) {
            *guard = Some(Rcon::new(&dir).with_timeout(Duration::from_secs(4)));
        }
        let rcon = guard.as_mut().unwrap();
        let list = rcon.command("list")?;
        let tick = rcon.command("tick query").unwrap_or_default();
        static LIST: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
            Regex::new(r"There are (\d+) of a max of (\d+) players online:?\s*(.*)").unwrap()
        });
        static TICK: std::sync::LazyLock<Regex> =
            std::sync::LazyLock::new(|| Regex::new(r"Average time per tick:\s*([\d.,]+)\s*ms").unwrap());
        let mut out = json!({"ok": true});
        if let Some(c) = LIST.captures(&list) {
            let names: Vec<String> = c[3]
                .split(',')
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty())
                .collect();
            out["online"] = json!(c[1].parse::<u32>().unwrap_or(0));
            out["max"] = json!(c[2].parse::<u32>().unwrap_or(0));
            out["players"] = json!(names);
        }
        if let Some(c) = TICK.captures(&tick) {
            let mspt: f64 = c[1].replace(',', ".").parse().unwrap_or(0.0);
            let tps = if mspt > 0.0 { (1000.0 / mspt).min(20.0) } else { 20.0 };
            out["mspt"] = json!(mspt);
            out["tps"] = json!(tps);
        }
        let ops: Vec<String> = server::read_json_names(&dir.join("ops.json"))
            .iter()
            .map(|n| n.to_lowercase())
            .collect();
        out["ops"] = json!(ops);
        Ok(out)
    })
    .await
}

// ---------------------------------------------------------------- Start / Stopp

#[tauri::command]
pub async fn start_server() -> Cmd<()> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        if !server::eula_accepted(&dir) {
            return err("EULA");
        }
        launcher::ensure_service()?;
        ipc::call("start", Value::Null)?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn stop_server(warn: u64, restart: bool) -> Cmd<()> {
    blocking(move || ops::stop_server(warn, restart)).await
}

#[tauri::command]
pub async fn accept_eula() -> Cmd<()> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        server::prepare_server(&dir, true)?;
        Ok(())
    })
    .await
}

#[tauri::command]
pub fn open_eula(app: AppHandle) -> Cmd<()> {
    app.opener()
        .open_url("https://aka.ms/MinecraftEULA", None::<&str>)
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------- Spieler

#[tauri::command]
pub fn get_whitelist() -> Cmd<Vec<String>> {
    let cfg = Config::load();
    let dir = active_dir(&cfg).map_err(|e| e.0)?;
    Ok(server::read_json_names(&dir.join("whitelist.json")))
}

/// Eintrag aus whitelist.json löschen, solange der Server aus ist.
#[tauri::command]
pub fn whitelist_remove_offline(name: String) -> Cmd<()> {
    (|| -> Result<()> {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let path = dir.join("whitelist.json");
        let list: Vec<Value> = serde_json::from_str(&fs::read_to_string(&path)?)?;
        let list: Vec<Value> = list
            .into_iter()
            .filter(|e| e.get("name").and_then(|n| n.as_str()) != Some(name.as_str()))
            .collect();
        fs::write(&path, serde_json::to_string_pretty(&list)?)?;
        Ok(())
    })()
    .map_err(|e| e.0)
}

// ---------------------------------------------------------------- Konsole

#[tauri::command]
pub async fn send_command(app: AppHandle, cmd: String) -> Cmd<String> {
    blocking(move || {
        let c = cmd.trim().trim_start_matches('/');
        if c == "stop" || c.starts_with("stop ") {
            return err("Bitte den Stopp-Knopf in der Übersicht benutzen (der speichert sicher).");
        }
        let gui = app.state::<Gui>();
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let mut guard = gui.rcon.lock().unwrap();
        if guard.as_ref().map(|r| r.dir() != dir.as_path()).unwrap_or(true) {
            *guard = Some(Rcon::new(&dir).with_timeout(Duration::from_secs(8)));
        }
        guard.as_mut().unwrap().command(c)
    })
    .await
}

/// Konsole neu füllen (z. B. nach Serverwechsel).
#[tauri::command]
pub fn log_reset(gui: State<'_, Gui>) {
    gui.log_reset.store(true, Ordering::SeqCst);
}

/// Liest latest.log fortlaufend, auch nach dem Umbenennen beim Serverstart.
struct LogTail {
    path: PathBuf,
    pos: u64,
    head: Vec<u8>,
    partial: String,
}

impl LogTail {
    fn new(path: PathBuf) -> LogTail {
        LogTail {
            path,
            pos: 0,
            head: vec![],
            partial: String::new(),
        }
    }

    fn read_head(f: &mut fs::File) -> Vec<u8> {
        let mut head = vec![0u8; 64];
        let n = f.read(&mut head).unwrap_or(0);
        head.truncate(n);
        head
    }

    /// Letzte Zeilen beim Öffnen.
    fn initial(&mut self, max_lines: usize) -> Vec<String> {
        let Ok(mut f) = fs::File::open(&self.path) else {
            self.pos = 0;
            self.head.clear();
            return vec![];
        };
        let size = f.metadata().map(|m| m.len()).unwrap_or(0);
        self.head = Self::read_head(&mut f);
        let start = size.saturating_sub(200_000);
        let _ = f.seek(SeekFrom::Start(start));
        let mut buf = Vec::new();
        let _ = f.read_to_end(&mut buf);
        self.pos = start + buf.len() as u64;
        let text = String::from_utf8_lossy(&buf);
        let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
        if start > 0 && !lines.is_empty() {
            lines.remove(0);
        }
        self.partial.clear();
        if !text.ends_with('\n')
            && let Some(last) = lines.pop()
        {
            self.partial = last;
        }
        let skip = lines.len().saturating_sub(max_lines);
        lines.into_iter().skip(skip).collect()
    }

    /// Neue Zeilen seit dem letzten Aufruf. `true` = Datei wurde neu angelegt.
    fn poll(&mut self) -> (bool, Vec<String>) {
        let Ok(mut f) = fs::File::open(&self.path) else {
            return (false, vec![]);
        };
        let size = f.metadata().map(|m| m.len()).unwrap_or(0);
        let head = Self::read_head(&mut f);
        let mut rotated = false;
        let same_head = head.len() >= self.head.len() && head[..self.head.len()] == self.head[..];
        if size < self.pos || !same_head {
            rotated = true;
            self.pos = 0;
            self.partial.clear();
        }
        if head.len() > self.head.len() || rotated {
            self.head = head;
        }
        if size == self.pos {
            return (rotated, vec![]);
        }
        let _ = f.seek(SeekFrom::Start(self.pos));
        let mut buf = Vec::new();
        let _ = f.take(4_000_000).read_to_end(&mut buf);
        self.pos += buf.len() as u64;
        let text = self.partial.clone() + &String::from_utf8_lossy(&buf);
        let mut lines: Vec<String> = text.split('\n').map(|l| l.trim_end_matches('\r').to_string()).collect();
        self.partial = lines.pop().unwrap_or_default();
        (rotated, lines)
    }
}

#[derive(Clone, Serialize)]
struct LogEvent {
    reset: bool,
    initial: bool,
    lines: Vec<String>,
}

fn log_thread(app: AppHandle, reset: Arc<AtomicBool>) {
    let mut tail: Option<LogTail> = None;
    let mut dir: Option<PathBuf> = None;
    let mut last_check = std::time::Instant::now() - Duration::from_secs(10);
    loop {
        std::thread::sleep(Duration::from_millis(400));
        if last_check.elapsed() > Duration::from_secs(2) || reset.load(Ordering::SeqCst) {
            last_check = std::time::Instant::now();
            let now = Config::load().active_path();
            if now != dir {
                dir = now;
                tail = None;
                reset.store(true, Ordering::SeqCst);
            }
        }
        if reset.swap(false, Ordering::SeqCst) {
            tail = dir.as_ref().map(|d| LogTail::new(d.join("logs").join("latest.log")));
            let lines = tail.as_mut().map(|t| t.initial(500)).unwrap_or_default();
            let _ = app.emit(
                "log",
                LogEvent {
                    reset: true,
                    initial: true,
                    lines,
                },
            );
            continue;
        }
        if let Some(t) = tail.as_mut() {
            let (rotated, lines) = t.poll();
            if rotated || !lines.is_empty() {
                let _ = app.emit(
                    "log",
                    LogEvent {
                        reset: rotated,
                        initial: false,
                        lines,
                    },
                );
            }
        }
    }
}

// ---------------------------------------------------------------- Aufgaben mit Fortschritt

#[derive(Clone, Serialize)]
struct TaskEvent {
    title: String,
    frac: Option<f64>,
    text: Option<String>,
    done: bool,
    error: Option<String>,
    result: Value,
}

/// Führt `f` im Hintergrund aus und meldet den Fortschritt als Ereignis „task“.
pub(crate) fn run_task(
    app: &AppHandle,
    title: &str,
    f: impl FnOnce(&(dyn Fn(f64, &str) + Sync)) -> Result<Value> + Send + 'static,
) -> Result<()> {
    let gui = app.state::<Gui>();
    {
        let mut busy = gui.busy.lock().unwrap();
        if let Some(b) = busy.as_ref() {
            return err(format!("Es läuft gerade schon: {b}"));
        }
        *busy = Some(title.to_string());
    }
    let lock = match ops::JobLock::acquire(title) {
        Ok(l) => l,
        Err(e) => {
            *gui.busy.lock().unwrap() = None;
            return Err(e);
        }
    };
    let app = app.clone();
    let busy = gui.busy.clone();
    let title = title.to_string();
    std::thread::spawn(move || {
        let last = Mutex::new(std::time::Instant::now() - Duration::from_secs(1));
        let progress = |frac: f64, text: &str| {
            let mut l = last.lock().unwrap();
            if l.elapsed() < Duration::from_millis(150) && frac < 1.0 {
                return;
            }
            *l = std::time::Instant::now();
            let _ = app.emit(
                "task",
                TaskEvent {
                    title: title.clone(),
                    frac: Some(frac.clamp(0.0, 1.0)),
                    text: Some(text.to_string()),
                    done: false,
                    error: None,
                    result: Value::Null,
                },
            );
        };
        let res = f(&progress);
        drop(lock);
        *busy.lock().unwrap() = None;
        let (error, result) = match res {
            Ok(v) => (None, v),
            Err(e) => (Some(e.0), Value::Null),
        };
        let _ = app.emit(
            "task",
            TaskEvent {
                title,
                frac: None,
                text: None,
                done: true,
                error,
                result,
            },
        );
    });
    Ok(())
}

/// Server über den Dienst stoppen und warten, bis er wirklich aus ist.
pub(crate) fn stop_and_wait(progress: &(dyn Fn(f64, &str) + Sync)) -> Result<()> {
    let cfg = Config::load();
    let st = server_state(&cfg);
    match st.get("state").and_then(|s| s.as_str()).unwrap_or("stopped") {
        "stopped" | "failed" | "none" => return Ok(()),
        "external" => return err("Der Server läuft außerhalb von Redcraftserver. Bitte zuerst dort stoppen."),
        "other" => return err("Ein anderer Server läuft gerade. Bitte zuerst stoppen."),
        _ => {}
    }
    progress(0.0, "Stoppe Server (speichert die Welt) …");
    let _ = ipc::call("stop", json!({"warn": 0}));
    for _ in 0..240 {
        std::thread::sleep(Duration::from_secs(1));
        let st = server_state(&Config::load());
        if matches!(
            st.get("state").and_then(|s| s.as_str()),
            Some("stopped" | "failed" | "none")
        ) {
            return Ok(());
        }
    }
    err("Der Server ließ sich nicht stoppen.")
}

pub(crate) fn is_running(cfg: &Config) -> bool {
    server_state(cfg).get("state").and_then(|s| s.as_str()) == Some("running")
}

// ---------------------------------------------------------------- Backups

#[tauri::command]
pub async fn list_backups() -> Cmd<Value> {
    blocking(|| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let name = cfg.name_of(&dir);
        let items: Vec<Value> = archive::list_backups(&cfg, &name)
            .iter()
            .map(|p| {
                json!({
                    "path": p,
                    "file": p.file_name().map(|n| n.to_string_lossy().into_owned()),
                    "label": archive::backup_label(p),
                    "size": fs::metadata(p).map(|m| m.len()).unwrap_or(0),
                })
            })
            .collect();
        Ok(json!({"items": items, "dir": cfg.backup_dir_for(&name), "keep": cfg.backup_keep}))
    })
    .await
}

#[tauri::command]
pub fn set_backup_keep(keep: usize) -> Cmd<()> {
    let mut cfg = Config::load();
    cfg.backup_keep = keep.clamp(1, 100);
    cfg.save().map_err(|e| e.0)
}

#[tauri::command]
pub fn backup_now(app: AppHandle) -> Cmd<()> {
    run_task(&app, "Backup (ganzer Server als ZIP)", |progress| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        let running = is_running(&cfg);
        let path = ops::backup_with_save(&dir, &cfg, running, progress)?;
        Ok(json!({"path": path, "size": fs::metadata(&path).map(|m| m.len()).unwrap_or(0)}))
    })
    .map_err(|e| e.0)
}

#[tauri::command]
pub fn restore_backup(app: AppHandle, path: String) -> Cmd<()> {
    run_task(&app, "Backup wiederherstellen", move |progress| {
        let cfg = Config::load();
        let dir = active_dir(&cfg)?;
        stop_and_wait(progress)?;
        let aside = archive::restore_backup(Path::new(&path), &dir, progress)?;
        server::prepare_server(&dir, false)?;
        Ok(json!({"aside": aside, "label": archive::backup_label(Path::new(&path))}))
    })
    .map_err(|e| e.0)
}

// ---------------------------------------------------------------- Dialoge und Ordner

#[tauri::command]
pub async fn pick_archive(app: AppHandle, title: String, start: Option<String>) -> Cmd<Option<String>> {
    blocking(move || {
        let mut d = app.dialog().file().set_title(title).add_filter(
            "Archive (zip, tar.gz, tar.zst)",
            &["zip", "ZIP", "gz", "tgz", "zst", "tar"],
        );
        if let Some(s) = start.filter(|s| Path::new(s).is_dir()) {
            d = d.set_directory(s);
        }
        Ok(d.blocking_pick_file()
            .and_then(|p| p.into_path().ok())
            .map(|p| p.to_string_lossy().into_owned()))
    })
    .await
}

#[tauri::command]
pub async fn pick_folder(app: AppHandle, title: String) -> Cmd<Option<String>> {
    blocking(move || {
        Ok(app
            .dialog()
            .file()
            .set_title(title)
            .set_directory(crate::config::home())
            .blocking_pick_folder()
            .and_then(|p| p.into_path().ok())
            .map(|p| p.to_string_lossy().into_owned()))
    })
    .await
}

#[tauri::command]
pub fn open_folder(app: AppHandle, which: String) -> Cmd<()> {
    let cfg = Config::load();
    let dir = active_dir(&cfg).map_err(|e| e.0)?;
    let path = match which.as_str() {
        "server" => dir.clone(),
        "mods" => dir.join("mods"),
        "backups" => cfg.backup_dir_for(&cfg.name_of(&dir)),
        "logs" => dir.join("logs"),
        _ => return Err("Unbekannter Ordner".into()),
    };
    let _ = fs::create_dir_all(&path);
    app.opener()
        .open_path(path.to_string_lossy(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn notify(app: AppHandle, title: String, body: String) {
    if Config::load().notify {
        let _ = app.notification().builder().title(title).body(body).show();
    }
}

#[tauri::command]
pub fn get_config() -> Value {
    let cfg = Config::load();
    json!({"notify": cfg.notify, "stop_warning": cfg.stop_warning, "backup_keep": cfg.backup_keep,
           "daily_backup": cfg.daily_backup, "backup_dir": cfg.backup_dir,
           "test_mode": crate::config::test_home().is_some(), "version": env!("CARGO_PKG_VERSION")})
}

// ---------------------------------------------------------------- Start

pub fn run() {
    let busy = Arc::new(Mutex::new(None));
    let reset_flag = Arc::new(AtomicBool::new(true));
    let gui = Gui {
        rcon: Mutex::new(None),
        meter: Mutex::new(Meter::new()),
        busy,
        log_reset: reset_flag.clone(),
    };
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .manage(gui)
        .setup(move |app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || log_thread(handle, reset_flag));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            get_details,
            get_live,
            start_server,
            stop_server,
            accept_eula,
            open_eula,
            get_whitelist,
            whitelist_remove_offline,
            send_command,
            log_reset,
            list_backups,
            set_backup_keep,
            backup_now,
            restore_backup,
            pick_archive,
            pick_folder,
            open_folder,
            notify,
            get_config,
            crate::manage::select_server,
            crate::manage::inspect_folder,
            crate::manage::add_server,
            crate::manage::import_server,
            crate::manage::server_versions,
            crate::manage::create_server,
            crate::manage::suggest_dir,
            crate::manage::check_dir,
            crate::manage::remove_server,
            crate::manage::get_settings,
            crate::manage::save_settings,
            crate::manage::set_autostart,
            crate::manage::enable_linger,
            crate::manage::set_window_autostart,
            crate::manage::set_daily_backup,
            crate::manage::set_notify,
            crate::manage::disable_old_panel,
            crate::updates_gui::upd_info,
            crate::updates_gui::upd_versions,
            crate::updates_gui::upd_check,
            crate::updates_gui::upd_run,
            crate::updates_gui::upd_rollback,
            crate::updates_gui::upd_trial,
            crate::updates_gui::export_mrpack,
            crate::updates_gui::mods_list,
            crate::updates_gui::mod_set_enabled,
            crate::updates_gui::cleanup_preview,
            crate::updates_gui::cleanup_run,
        ])
        .run(tauri::generate_context!())
        .expect("Fenster konnte nicht gestartet werden");
}
