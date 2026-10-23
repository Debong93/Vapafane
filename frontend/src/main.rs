//! Vapafane-Oberfläche in Rust (Leptos). Läuft als WebAssembly im Tauri-Fenster und
//! spricht über Tauri-Befehle mit dem Rust-Backend.

mod header;
mod mobs;
mod pages;
mod sidebar;
mod state;
mod tauri;

use leptos::prelude::*;

use header::Header;
use pages::CurrentPage;
use sidebar::Sidebar;
use state::{AppState, Mode, apply_theme};

#[component]
fn App() -> impl IntoView {
    let app = AppState::new();
    provide_context(app);
    apply_theme();
    app.start_polling();

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

    view! {
        <Header/>
        <div class="shell">
            <Sidebar/>
            <main><CurrentPage/></main>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
