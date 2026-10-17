//! Hintergrund-Modus (`redcraftserver --service`).
//!
//! Startet den aktiven Server als Kindprozess, merkt, wann er bereit ist,
//! startet ihn nach einem Absturz neu, stoppt ihn sauber per RCON „stop“ und
//! macht das tägliche Backup. Das Fenster spricht über [`crate::ipc`] mit ihm.

use serde::Serialize;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Stdio};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::config::{self, Config};
use crate::ipc::{self, Request, Response, ServiceFile};
use crate::ops;
use crate::procs;
use crate::rcon::{self, Rcon};
use crate::server;
use crate::util::{self, Result, err};

/// So lange darf der Server nach „stop“ zum Speichern brauchen.
const STOP_TIMEOUT: Duration = Duration::from_secs(180);
/// Wartezeit vor einem automatischen Neustart nach Absturz.
const RESTART_DELAY: Duration = Duration::from_secs(20);
/// Höchstens so viele automatische Neustarts innerhalb von 10 Minuten.
const MAX_RESTARTS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Stopped,
    Starting,
    Running,
    Stopping,
    Restarting,
    Failed,
}

#[derive(Default)]
struct Inner {
    state: Option<State>,
    dir: Option<PathBuf>,
    pid: Option<u32>,
    started_at: u64,
    stdin: Option<ChildStdin>,
    /// Prozess, der schon lief, als der Dienst gestartet wurde (kein Kindprozess)
    adopted: bool,
    stop_requested: bool,
    restart_after_stop: bool,
    crash_times: VecDeque<Instant>,
    restarts: u32,
    last_exit: Option<String>,
    countdown: Option<String>,
    output: VecDeque<String>,
    job: Option<String>,
    /// Hochgezählt bei jedem Start, damit alte Überwachungs-Threads aufhören
    generation: u64,
}

struct Shared {
    inner: Mutex<Inner>,
    changed: Condvar,
    quit: Mutex<bool>,
}

type Sup = Arc<Shared>;

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn log(msg: &str) {
    let line = format!("[{}] {msg}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    eprintln!("{line}");
    let path = config::state_dir().join("dienst.log");
    if fs::metadata(&path).map(|m| m.len() > 2_000_000).unwrap_or(false) {
        let _ = fs::rename(&path, config::state_dir().join("dienst.alt.log"));
    }
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{line}");
    }
}

fn pid_file() -> PathBuf {
    config::state_dir().join("server.pid")
}

// ---------------------------------------------------------------- Einstieg

pub fn run(start_now: bool) -> i32 {
    let _ = fs::create_dir_all(config::state_dir());
    // Nur ein Dienst gleichzeitig
    let lock = match fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(config::state_dir().join("service.lock"))
    {
        Ok(f) => f,
        Err(e) => {
            log(&format!("Kann Sperrdatei nicht öffnen: {e}"));
            return 1;
        }
    };
    if lock.try_lock().is_err() {
        log("Dienst läuft schon.");
        if start_now {
            let _ = ipc::call("start", Value::Null);
        }
        return 0;
    }

    let listener = match TcpListener::bind("127.0.0.1:0") {
        Ok(l) => l,
        Err(e) => {
            log(&format!("Kann keinen Port öffnen: {e}"));
            return 1;
        }
    };
    let port = listener.local_addr().map(|a| a.port()).unwrap_or(0);
    let token = util::random_hex(24);
    if let Err(e) = ipc::write_service_file(&ServiceFile {
        port,
        token: token.clone(),
        pid: std::process::id(),
        version: env!("CARGO_PKG_VERSION").into(),
    }) {
        log(&format!("Kann service.json nicht schreiben: {e}"));
        return 1;
    }
    log(&format!("Dienst gestartet (Version {}).", env!("CARGO_PKG_VERSION")));

    let sup: Sup = Arc::new(Shared {
        inner: Mutex::new(Inner {
            state: Some(State::Stopped),
            ..Default::default()
        }),
        changed: Condvar::new(),
        quit: Mutex::new(false),
    });

    adopt_running(&sup);

    {
        let sup = sup.clone();
        let _ = ctrlc::set_handler(move || {
            log("Beenden angefordert – stoppe den Server sauber.");
            shutdown(&sup);
        });
    }

    {
        let sup = sup.clone();
        std::thread::spawn(move || daily_backup_loop(sup));
    }

    if start_now
        && let Err(e) = start(&sup) {
            log(&format!("Autostart fehlgeschlagen: {e}"));
        }

    {
        let sup = sup.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let sup = sup.clone();
                let token = token.clone();
                std::thread::spawn(move || handle_client(stream, &sup, &token));
            }
        });
    }

    // Warten, bis jemand „quit“ sagt
    let mut q = sup.quit.lock().unwrap();
    while !*q {
        q = sup.changed.wait(q).unwrap();
    }
    drop(q);
    let _ = fs::remove_file(ipc::service_file());
    log("Dienst beendet.");
    drop(lock);
    0
}

fn shutdown(sup: &Sup) {
    let running = sup.inner.lock().unwrap().pid.is_some();
    if running {
        stop(sup, 0, false);
        wait_until_stopped(sup, STOP_TIMEOUT + Duration::from_secs(15));
    }
    *sup.quit.lock().unwrap() = true;
    sup.changed.notify_all();
    // Falls run() gerade nicht wartet (z. B. beim Signal), sicher beenden
    std::thread::sleep(Duration::from_millis(300));
    let _ = fs::remove_file(ipc::service_file());
    std::process::exit(0);
}

fn wait_until_stopped(sup: &Sup, timeout: Duration) -> bool {
    let end = Instant::now() + timeout;
    let mut g = sup.inner.lock().unwrap();
    while g.pid.is_some() {
        let left = end.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return false;
        }
        g = sup.changed.wait_timeout(g, left.min(Duration::from_secs(1))).unwrap().0;
    }
    true
}

// ---------------------------------------------------------------- Anfragen

fn handle_client(stream: std::net::TcpStream, sup: &Sup, token: &str) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    });
    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return;
    }
    let resp = match serde_json::from_str::<Request>(&line) {
        Ok(req) if req.token == token => match dispatch(sup, &req.cmd, &req.arg) {
            Ok(data) => Response {
                ok: true,
                error: None,
                data,
            },
            Err(e) => Response {
                ok: false,
                error: Some(e.0),
                data: Value::Null,
            },
        },
        _ => Response {
            ok: false,
            error: Some("Nicht erlaubt".into()),
            data: Value::Null,
        },
    };
    let mut out = stream;
    let _ = out.write_all((serde_json::to_string(&resp).unwrap_or_default() + "\n").as_bytes());
}

fn dispatch(sup: &Sup, cmd: &str, arg: &Value) -> Result<Value> {
    match cmd {
        "ping" => Ok(json!("pong")),
        "status" => Ok(status(sup)),
        "start" => {
            start(sup)?;
            Ok(Value::Null)
        }
        "stop" | "restart" => {
            let warn = arg.get("warn").and_then(|v| v.as_u64()).unwrap_or(0);
            let st = sup.inner.lock().unwrap().state;
            if !matches!(st, Some(State::Running | State::Starting | State::Restarting)) {
                if st == Some(State::Failed) {
                    let mut g = sup.inner.lock().unwrap();
                    g.state = Some(State::Stopped);
                    sup.changed.notify_all();
                    return Ok(Value::Null);
                }
                return err("Der Server läuft gerade nicht.");
            }
            let sup2 = sup.clone();
            let restart = cmd == "restart";
            std::thread::spawn(move || stop(&sup2, warn, restart));
            Ok(Value::Null)
        }
        "quit" => {
            let sup2 = sup.clone();
            std::thread::spawn(move || shutdown(&sup2));
            Ok(Value::Null)
        }
        _ => err(format!("Unbekannter Befehl: {cmd}")),
    }
}

fn status(sup: &Sup) -> Value {
    let g = sup.inner.lock().unwrap();
    json!({
        "state": g.state.unwrap_or(State::Stopped),
        "dir": g.dir,
        "pid": g.pid,
        "started_at": g.started_at,
        "restarts": g.restarts,
        "last_exit": g.last_exit,
        "countdown": g.countdown,
        "job": g.job,
        "adopted": g.adopted,
        "output": g.output.iter().rev().take(40).rev().cloned().collect::<Vec<_>>(),
        "service_pid": std::process::id(),
        "version": env!("CARGO_PKG_VERSION"),
    })
}

// ---------------------------------------------------------------- Starten

/// Läuft beim Dienststart schon ein Server (z. B. Dienst wurde neu gestartet)? Übernehmen.
fn adopt_running(sup: &Sup) {
    let Ok(text) = fs::read_to_string(pid_file()) else {
        return;
    };
    let v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    let (Some(pid), Some(dir)) = (
        v.get("pid").and_then(|p| p.as_u64()),
        v.get("dir").and_then(|d| d.as_str()),
    ) else {
        return;
    };
    let pid = pid as u32;
    let dir = PathBuf::from(dir);
    if procs::java_in_dir(&dir, None) != Some(pid) {
        let _ = fs::remove_file(pid_file());
        return;
    }
    log(&format!("Laufenden Server übernommen (PID {pid})."));
    let generation = {
        let mut g = sup.inner.lock().unwrap();
        g.state = Some(if rcon::once(&dir, "list").is_ok() {
            State::Running
        } else {
            State::Starting
        });
        g.dir = Some(dir.clone());
        g.pid = Some(pid);
        g.adopted = true;
        g.started_at = procs::start_time(pid).unwrap_or_else(now_secs);
        g.generation += 1;
        g.generation
    };
    let sup = sup.clone();
    std::thread::spawn(move || watch_adopted(sup, pid, dir, generation));
}

fn watch_adopted(sup: Sup, pid: u32, dir: PathBuf, generation: u64) {
    loop {
        std::thread::sleep(Duration::from_secs(2));
        if sup.inner.lock().unwrap().generation != generation {
            return;
        }
        if !procs::pid_alive(pid) {
            on_exit(&sup, generation, None);
            return;
        }
        let mut g = sup.inner.lock().unwrap();
        if g.state == Some(State::Starting) && rcon::once(&dir, "list").is_ok() {
            g.state = Some(State::Running);
            sup.changed.notify_all();
        }
    }
}

fn start(sup: &Sup) -> Result<()> {
    {
        let g = sup.inner.lock().unwrap();
        if g.pid.is_some() {
            return Ok(()); // läuft schon
        }
    }
    let cfg = Config::load();
    let Some(dir) = cfg.active_path() else {
        return err("Kein aktiver Server ausgewählt.");
    };
    if !dir.is_dir() {
        return err(format!("Server-Ordner fehlt: {}", dir.display()));
    }
    if !server::eula_accepted(&dir) {
        return err("Die Minecraft-EULA ist für diesen Server noch nicht akzeptiert.");
    }
    if let Some(pid) = procs::java_in_dir(&dir, None) {
        return err(format!(
            "In diesem Ordner läuft schon ein Server (PID {pid}), der nicht von Redcraftserver gestartet wurde. \
             Bitte den zuerst beenden (z. B. altes Bedienfenster)."
        ));
    }
    let props = server::read_properties(&dir);
    for key in ["server-port", "rcon.port"] {
        let default = if key == "rcon.port" { 25575 } else { 25565 };
        let port = props.get(key).and_then(|p| p.parse().ok()).unwrap_or(default);
        if !procs::port_free(port) {
            return err(format!(
                "Port {port} ({key}) ist schon belegt. Läuft noch ein anderer Minecraft-Server auf diesem PC?"
            ));
        }
    }
    spawn_server(sup, &dir, false)
}

fn spawn_server(sup: &Sup, dir: &Path, is_restart: bool) -> Result<()> {
    let (java, args) = server::launch_command(dir)?;
    log(&format!(
        "Starte Server in {}: {java} {}",
        dir.display(),
        args.join(" ")
    ));
    let out_path = config::state_dir().join("server-ausgabe.log");
    let out_file = fs::File::create(&out_path).ok().map(|f| Arc::new(Mutex::new(f)));

    let mut cmd = util::command(&java);
    cmd.args(&args)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0); // Strg+C im Terminal soll nicht direkt Java treffen
    }
    let mut child: Child = cmd
        .spawn()
        .map_err(|e| util::Error(format!("Java konnte nicht gestartet werden ({java}): {e}")))?;
    let pid = child.id();
    let _ = fs::write(pid_file(), json!({"pid": pid, "dir": dir}).to_string());

    let generation = {
        let mut g = sup.inner.lock().unwrap();
        g.state = Some(State::Starting);
        g.dir = Some(dir.to_path_buf());
        g.pid = Some(pid);
        g.adopted = false;
        g.started_at = now_secs();
        g.stdin = child.stdin.take();
        g.stop_requested = false;
        g.restart_after_stop = false;
        g.countdown = None;
        g.output.clear();
        if !is_restart {
            g.restarts = 0;
            g.crash_times.clear();
            g.last_exit = None;
        }
        g.generation += 1;
        sup.changed.notify_all();
        g.generation
    };

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
        let sup = sup.clone();
        let out_file = out_file.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stream);
            for line in reader.split(b'\n').map_while(|l| l.ok()) {
                let line = String::from_utf8_lossy(&line).trim_end().to_string();
                if let Some(f) = &out_file {
                    let _ = writeln!(f.lock().unwrap(), "{line}");
                }
                let mut g = sup.inner.lock().unwrap();
                if g.generation != generation {
                    continue;
                }
                if g.state == Some(State::Starting) && line.contains("]: Done (") {
                    g.state = Some(State::Running);
                    sup.changed.notify_all();
                }
                g.output.push_back(line);
                while g.output.len() > 300 {
                    g.output.pop_front();
                }
            }
        });
    }

    let sup = sup.clone();
    std::thread::spawn(move || {
        let status = child.wait();
        let code = status.as_ref().ok().and_then(|s| s.code());
        on_exit(&sup, generation, Some(code));
    });
    Ok(())
}

/// Server-Prozess ist beendet. `code`: None = unbekannt (übernommener Prozess).
fn on_exit(sup: &Sup, generation: u64, code: Option<Option<i32>>) {
    let mut g = sup.inner.lock().unwrap();
    if g.generation != generation {
        return;
    }
    let _ = fs::remove_file(pid_file());
    g.pid = None;
    g.stdin = None;
    g.countdown = None;
    let dir = g.dir.clone();
    let wanted = g.stop_requested;
    let restart = g.restart_after_stop;
    let clean = matches!(code, Some(Some(0)) | None);
    let desc = match code {
        Some(Some(c)) => format!("Exit-Code {c}"),
        Some(None) => "durch Signal beendet".into(),
        None => "beendet".into(),
    };
    log(&format!(
        "Server beendet ({desc}){}",
        if wanted { " – wie gewünscht" } else { "" }
    ));

    if wanted || clean {
        g.state = Some(State::Stopped);
        g.last_exit = if wanted || clean { None } else { Some(desc) };
        sup.changed.notify_all();
        if restart {
            drop(g);
            if let Some(dir) = dir
                && let Err(e) = spawn_server(sup, &dir, false) {
                    log(&format!("Neustart fehlgeschlagen: {e}"));
                }
        }
        return;
    }

    // Absturz
    let why = crash_reason(&g.output);
    g.last_exit = Some(format!("Abgestürzt ({desc}).\n{why}"));
    let now = Instant::now();
    g.crash_times.push_back(now);
    while g
        .crash_times
        .front()
        .map(|t| now.duration_since(*t) > Duration::from_secs(600))
        .unwrap_or(false)
    {
        g.crash_times.pop_front();
    }
    if g.crash_times.len() > MAX_RESTARTS {
        log("Zu viele Abstürze hintereinander – kein automatischer Neustart mehr.");
        g.state = Some(State::Failed);
        sup.changed.notify_all();
        return;
    }
    g.state = Some(State::Restarting);
    g.restarts += 1;
    g.generation += 1;
    let my_gen = g.generation;
    sup.changed.notify_all();
    drop(g);
    let sup = sup.clone();
    std::thread::spawn(move || {
        std::thread::sleep(RESTART_DELAY);
        let g = sup.inner.lock().unwrap();
        // Inzwischen gestoppt oder anders gestartet?
        if g.generation != my_gen || g.state != Some(State::Restarting) {
            return;
        }
        drop(g);
        log("Starte nach Absturz neu …");
        if let Some(dir) = dir
            && let Err(e) = spawn_server(&sup, &dir, true) {
                log(&format!("Neustart fehlgeschlagen: {e}"));
                let mut g = sup.inner.lock().unwrap();
                g.state = Some(State::Failed);
                g.last_exit = Some(e.0);
                sup.changed.notify_all();
            }
    });
}

/// Die aussagekräftigen Zeilen einer Absturz-Ausgabe (statt Stacktrace-Zeilen).
fn crash_reason(output: &VecDeque<String>) -> String {
    let keys = [
        "ERROR",
        "Exception",
        "requires",
        "Install ",
        "missing",
        "incompatible",
        "Incompatible",
        "Caused by",
    ];
    let mut picked: Vec<String> = vec![];
    for line in output.iter() {
        let t = line.trim();
        if t.starts_with("at ") || t.is_empty() {
            continue;
        }
        if keys.iter().any(|k| t.contains(k)) && !picked.iter().any(|p| p == t) {
            picked.push(t.chars().take(300).collect());
        }
    }
    if picked.is_empty() {
        picked = output.iter().rev().take(5).rev().cloned().collect();
    }
    let skip = picked.len().saturating_sub(8);
    picked.into_iter().skip(skip).collect::<Vec<_>>().join("\n")
}

// ---------------------------------------------------------------- Stoppen

/// Stoppt den Server: optional Countdown-Warnung, dann RCON „stop“ und warten.
/// Erst wenn das nicht klappt: „stop“ über die Konsole, ganz zuletzt hart beenden.
fn stop(sup: &Sup, warn_secs: u64, restart: bool) {
    let (dir, pid) = {
        let mut g = sup.inner.lock().unwrap();
        if g.state == Some(State::Restarting) {
            // Wartet gerade auf Neustart nach Absturz – einfach abbrechen
            g.state = Some(State::Stopped);
            g.generation += 1;
            sup.changed.notify_all();
            return;
        }
        if g.stop_requested {
            return;
        }
        let (Some(dir), Some(pid)) = (g.dir.clone(), g.pid) else {
            return;
        };
        g.stop_requested = true;
        g.restart_after_stop = restart;
        (dir, pid)
    };
    let word = if restart { "startet neu" } else { "stoppt" };
    let mut rcon = Rcon::new(&dir).with_timeout(Duration::from_secs(10));

    if warn_secs > 0 {
        let mut remaining = warn_secs;
        for mark in [60u64, 30, 10, 5, 3, 2, 1] {
            if mark > remaining {
                continue;
            }
            countdown_sleep(sup, remaining - mark, mark, word);
            remaining = mark;
            let unit = if mark == 1 { "Sekunde" } else { "Sekunden" };
            let _ = rcon.command(&format!("say Server {word} in {mark} {unit}!"));
            set_countdown(sup, Some(format!("Server {word} in {mark} s")));
        }
        countdown_sleep(sup, remaining, 0, word);
    }

    {
        let mut g = sup.inner.lock().unwrap();
        g.state = Some(State::Stopping);
        g.countdown = None;
        sup.changed.notify_all();
    }
    log(&format!("Stoppe Server (PID {pid}) per RCON …"));
    let mut sent = rcon.command("stop").is_ok();
    if !sent {
        let mut g = sup.inner.lock().unwrap();
        if let Some(stdin) = g.stdin.as_mut() {
            log("RCON nicht erreichbar – sende „stop“ über die Konsole.");
            sent = stdin.write_all(b"stop\n").and_then(|_| stdin.flush()).is_ok();
        }
    }
    rcon.close();
    if !sent {
        log("Konnte „stop“ nicht senden.");
    }
    if !wait_until_stopped(sup, if sent { STOP_TIMEOUT } else { Duration::from_secs(20) }) {
        log("Server reagiert nicht – wird hart beendet.");
        procs::kill(pid);
        wait_until_stopped(sup, Duration::from_secs(15));
    }
}

/// `secs` Sekunden warten; angezeigt wird die Restzeit bis zum Stopp (`then` + Rest).
fn countdown_sleep(sup: &Sup, secs: u64, then: u64, word: &str) {
    for left in (1..=secs).rev() {
        set_countdown(sup, Some(format!("Server {word} in {} s", then + left)));
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn set_countdown(sup: &Sup, text: Option<String>) {
    sup.inner.lock().unwrap().countdown = text;
}

// ---------------------------------------------------------------- Tägliches Backup

fn daily_state_file() -> PathBuf {
    config::state_dir().join("daily.json")
}

fn daily_backup_loop(sup: Sup) {
    use chrono::{Local, NaiveTime, TimeZone};
    let mut last: i64 = fs::read_to_string(daily_state_file())
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.get("last").and_then(|l| l.as_i64()))
        .unwrap_or_else(|| {
            let now = Local::now().timestamp();
            let _ = fs::write(daily_state_file(), json!({"last": now}).to_string());
            now
        });
    loop {
        std::thread::sleep(Duration::from_secs(30));
        let cfg = Config::load();
        if !cfg.daily_backup {
            continue;
        }
        let time = NaiveTime::parse_from_str(&cfg.daily_backup_time, "%H:%M")
            .unwrap_or_else(|_| NaiveTime::from_hms_opt(5, 0, 0).unwrap());
        let now = Local::now();
        let Some(due) = Local.from_local_datetime(&now.date_naive().and_time(time)).earliest() else {
            continue;
        };
        // Fällig, wenn der heutige Termin vorbei ist und seitdem keins lief
        // (holt es nach, falls der PC zur Backup-Zeit aus war)
        if now < due || last >= due.timestamp() {
            continue;
        }
        let Some(dir) = cfg.active_path() else {
            continue;
        };
        let Ok(_lock) = ops::JobLock::acquire("Tägliches Backup") else {
            continue; // später nochmal versuchen
        };
        let running = {
            let mut g = sup.inner.lock().unwrap();
            g.job = Some("Tägliches Backup".into());
            g.state == Some(State::Running) && g.dir.as_deref() == Some(dir.as_path())
        };
        log("Tägliches Backup startet …");
        let noop = |_: f64, _: &str| {};
        match ops::backup_with_save(&dir, &cfg, running, &noop) {
            Ok(p) => log(&format!("Tägliches Backup fertig: {}", p.display())),
            Err(e) => log(&format!("Tägliches Backup fehlgeschlagen: {e}")),
        }
        sup.inner.lock().unwrap().job = None;
        last = Local::now().timestamp();
        let _ = fs::write(daily_state_file(), json!({"last": last}).to_string());
    }
}
