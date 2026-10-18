//! Vapafane – Minecraft-Serververwaltung.
//!
//! Eine Programmdatei mit zwei Rollen:
//! - ohne Argumente: das Fenster
//! - `--ctl <befehl>`: Befehlszeile (siehe ctl.rs)
//! - `--service [--start]`: der Hintergrund-Dienst, der den Server startet,
//!   überwacht, nach Absturz neu startet und das tägliche Backup macht.

mod archive;
mod autostart;
mod catalog;
mod cleanup;
mod client;
mod clients_gui;
mod config;
mod ctl;
mod curseforge;
mod flavor;
mod gui;
mod invite;
mod ipc;
mod java;
mod launcher;
mod manage;
mod migrate;
mod mods;
mod net;
mod ops;
mod procs;
mod rcon;
mod server;
mod service;
mod updater;
mod updates_gui;
mod util;

pub fn main() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(i) = args.iter().position(|a| a == "--ctl") {
        // Das Programm hat unter Windows kein eigenes Konsolenfenster – Ausgabe ins Terminal leiten
        #[cfg(windows)]
        unsafe {
            windows_sys::Win32::System::Console::AttachConsole(
                windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS,
            );
        }
        if let Some(msg) = migrate::run() {
            eprintln!("{msg}");
        }
        return ctl::run(&args[i + 1..]);
    }
    if args.iter().any(|a| a == "--service") {
        return service::run(args.iter().any(|a| a == "--start"));
    }
    gui::run();
    0
}
