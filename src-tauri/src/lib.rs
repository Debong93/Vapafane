//! Redcraftserver – Minecraft-Serververwaltung.
//!
//! Eine Programmdatei mit zwei Rollen:
//! - ohne Argumente: das Fenster
//! - `--ctl <befehl>`: Befehlszeile (siehe ctl.rs)
//! - `--service [--start]`: der Hintergrund-Dienst, der den Server startet,
//!   überwacht, nach Absturz neu startet und das tägliche Backup macht.

mod archive;
mod autostart;
mod config;
mod ctl;
mod gui;
mod ipc;
mod java;
mod launcher;
mod ops;
mod procs;
mod rcon;
mod server;
mod service;
mod util;

pub fn main() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(i) = args.iter().position(|a| a == "--ctl") {
        return ctl::run(&args[i + 1..]);
    }
    if args.iter().any(|a| a == "--service") {
        return service::run(args.iter().any(|a| a == "--start"));
    }
    gui::run();
    0
}
