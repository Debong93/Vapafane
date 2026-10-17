// Ohne Konsolenfenster unter Windows (auch im Hintergrund-Modus)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    std::process::exit(redcraftserver_lib::main());
}
