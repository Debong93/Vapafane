//! Vapafane-Oberfläche in Rust (Leptos). Läuft als WebAssembly im Tauri-Fenster und
//! spricht über Tauri-Befehle mit dem Rust-Backend.

// Kein unwrap/expect und kein unsafe – Fehler werden sauber behandelt.
#![deny(unsafe_code, clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod dialog;
mod header;
mod mobs;
mod pages;
mod sidebar;
mod sprache;
mod state;
mod tasks;
mod tauri;
mod util;

use leptos::prelude::*;

use dialog::Modal;
use header::Header;
use pages::CurrentPage;
use sidebar::Sidebar;
use state::{Mode, apply_theme, store, stored};
use tasks::Taskbar;

#[component]
fn App() -> impl IntoView {
    let app = state::use_app();
    // Gemeinsame Zustände jetzt anlegen (gehören damit der App und leben so lange wie sie)
    dialog::init();
    tasks::init();
    pages::console::init();
    apply_theme();
    tasks::listen_tasks();
    pages::console::listen_log();
    app.start_polling();
    sprache::abgleichen();

    // Im Client-Bereich sind Seitenleiste und Server-Auswahl ausgeblendet (CSS: body.client-mode)
    Effect::new(move |_| {
        let client = app.mode.get() == Mode::Client;
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body.class_list().toggle_with_force("client-mode", client);
        }
    });

    // Einmal beim ersten Start: Hinweis, dass Vapafane kein offizielles Produkt ist
    if stored("hinweis_nicht_offiziell").is_none() {
        leptos::task::spawn_local(async {
            dialog::info("Willkommen bei Vapafane", "Vapafane ist kein offizielles Minecraft-Produkt. Es ist nicht von Mojang oder Microsoft genehmigt und steht in keiner Verbindung zu ihnen.\n\nFragen, Ideen oder Fehler: über die GitHub-Seite des Projekts (Einstellungen → Über Vapafane).").await;
            store("hinweis_nicht_offiziell", Some("1"));
        });
    }

    view! {
        <Header/>
        <div class="shell">
            <Sidebar/>
            <main><CurrentPage/></main>
        </div>
        <Taskbar/>
        <Modal/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    // Sprache zuerst – alle Seiten bauen ihre Texte damit auf
    sprache::init();
    leptos::mount::mount_to_body(App);
}
