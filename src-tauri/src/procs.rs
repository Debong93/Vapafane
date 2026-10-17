//! Prozesse: Java-Server finden, CPU/RAM messen, notfalls beenden.

use std::path::Path;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

use crate::util;

pub fn pid_alive(pid: u32) -> bool {
    let mut sys = System::new();
    let p = Pid::from_u32(pid);
    sys.refresh_processes_specifics(ProcessesToUpdate::Some(&[p]), true, ProcessRefreshKind::nothing());
    sys.process(p).is_some()
}

pub fn kill(pid: u32) -> bool {
    let mut sys = System::new();
    let p = Pid::from_u32(pid);
    sys.refresh_processes_specifics(ProcessesToUpdate::Some(&[p]), true, ProcessRefreshKind::nothing());
    sys.process(p).map(|p| p.kill()).unwrap_or(false)
}

fn is_java(name: &str) -> bool {
    let n = name.to_lowercase();
    n == "java" || n == "java.exe" || n == "javaw.exe" || n.starts_with("java")
}

/// Läuft schon ein Java-Prozess in diesem Ordner (z. B. vom alten Panel oder per Terminal)?
pub fn java_in_dir(dir: &Path, except: Option<u32>) -> Option<u32> {
    let dir = util::canonical(dir);
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_cwd(UpdateKind::Always)
            .with_exe(UpdateKind::OnlyIfNotSet),
    );
    for (pid, p) in sys.processes() {
        // Unter Linux stehen auch einzelne Threads in der Liste – die überspringen
        if p.thread_kind().is_some() || Some(pid.as_u32()) == except || !is_java(&p.name().to_string_lossy()) {
            continue;
        }
        if let Some(cwd) = p.cwd()
            && util::canonical(cwd) == dir
        {
            return Some(pid.as_u32());
        }
    }
    None
}

/// Startzeit eines Prozesses (Unix-Sekunden).
pub fn start_time(pid: u32) -> Option<u64> {
    let mut sys = System::new();
    let p = Pid::from_u32(pid);
    sys.refresh_processes_specifics(ProcessesToUpdate::Some(&[p]), true, ProcessRefreshKind::nothing());
    sys.process(p).map(|p| p.start_time())
}

/// Misst CPU (in % aller Kerne) und RAM eines Prozesses über mehrere Aufrufe.
pub struct Meter {
    sys: System,
    pid: Option<Pid>,
    ncpu: usize,
}

impl Meter {
    pub fn new() -> Meter {
        let ncpu = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        Meter {
            sys: System::new(),
            pid: None,
            ncpu,
        }
    }

    pub fn ncpu(&self) -> usize {
        self.ncpu
    }

    /// (CPU-%, RAM in Bytes). CPU ist beim ersten Aufruf für eine PID noch None.
    pub fn sample(&mut self, pid: u32) -> (Option<f32>, Option<u64>) {
        let p = Pid::from_u32(pid);
        let first = self.pid != Some(p);
        self.pid = Some(p);
        self.sys.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[p]),
            true,
            ProcessRefreshKind::nothing().with_cpu().with_memory(),
        );
        match self.sys.process(p) {
            Some(proc_) => {
                let cpu = if first {
                    None
                } else {
                    Some(proc_.cpu_usage() / self.ncpu as f32)
                };
                (cpu, Some(proc_.memory()))
            }
            None => (None, None),
        }
    }
}

/// Gesamter Arbeitsspeicher des PCs in Bytes.
pub fn total_memory() -> u64 {
    let mut sys = System::new();
    sys.refresh_memory();
    sys.total_memory()
}

/// Ist ein TCP-Port auf diesem PC frei?
pub fn port_free(port: u16) -> bool {
    std::net::TcpListener::bind(("0.0.0.0", port)).is_ok()
}

/// Gerade freier Arbeitsspeicher in Bytes.
pub fn available_memory() -> u64 {
    let mut sys = System::new();
    sys.refresh_memory();
    sys.available_memory()
}
