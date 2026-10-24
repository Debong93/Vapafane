//! Vapafane – Minecraft-Serververwaltung.
//!
//! Eine Programmdatei mit zwei Rollen:
//! - ohne Argumente: das Fenster
//! - `--ctl <befehl>`: Befehlszeile (siehe ctl.rs)
//! - `--service [--start]`: der Hintergrund-Dienst, der den Server startet,
//!   überwacht, nach Absturz neu startet und das tägliche Backup macht.

// Kein unwrap/expect und kein unsafe – Fehler werden als Meldung weitergegeben.
// Einzige Ausnahme: das Anhängen an die Konsole unter Windows (siehe `attach_console`).
#![deny(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

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
mod mods;
mod nbt;
mod net;
mod notify;
mod ops;
mod procs;
mod rcon;
mod reach;
mod server;
mod service;
mod transfer;
mod updater;
mod updates_gui;
mod util;

/// Windows: Das Programm hat kein eigenes Konsolenfenster – für `--ctl` die Ausgabe ins
/// aufrufende Terminal leiten. Dafür gibt es keinen sicheren Weg ohne Systemaufruf; der
/// Aufruf ist harmlos (keine Zeiger, schlägt höchstens fehl, wenn es kein Terminal gibt).
#[allow(unsafe_code)]
fn attach_console() {
    #[cfg(windows)]
    // SAFETY: AttachConsole bekommt nur eine Konstante und gibt nichts zurück, was wir benutzen.
    unsafe {
        windows_sys::Win32::System::Console::AttachConsole(windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS);
    }
}

pub fn main() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Mod-Notizen unter dem früheren Dateinamen einmalig übernehmen
    mods::convert_old_meta_all();
    if let Some(i) = args.iter().position(|a| a == "--ctl") {
        attach_console();
        return ctl::run(&args[i + 1..]);
    }
    if args.iter().any(|a| a == "--service") {
        return service::run(args.iter().any(|a| a == "--start"));
    }
    gui::run()
}
