//! Schreibt frontend/style/textures.css neu: `cargo run --bin texturen [ZIELDATEI]`

#![deny(unsafe_code, clippy::unwrap_used, clippy::expect_used)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let target = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "../frontend/style/textures.css".into());
    match std::fs::write(&target, vapafane_werkzeuge::texturen::css()) {
        Ok(()) => {
            println!("ok: {target}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Fehler beim Schreiben von {target}: {e}");
            ExitCode::FAILURE
        }
    }
}
