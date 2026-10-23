//! Seiten. Im ersten Schritt nur Platzhalter – sie werden Schritt für Schritt aus der
//! bisherigen JavaScript-Oberfläche nach Rust übertragen.

use leptos::prelude::*;

use crate::state::{Page, set_skin, use_app};

/// Hinweis für Funktionen, die im Umbau noch fehlen.
pub fn not_yet(what: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.alert_with_message(&format!(
            "„{what}“ wird im nächsten Schritt auf Rust umgebaut."
        ));
    }
}

fn title(p: Page) -> &'static str {
    match p {
        Page::Overview => "Übersicht",
        Page::Console => "Konsole",
        Page::Players => "Spieler",
        Page::Mods => "Mods/Plugins",
        Page::Updates => "Version & Updates",
        Page::Backups => "Backups",
        Page::Reach => "Erreichbarkeit",
        Page::Settings => "Einstellungen",
        Page::Client => "Einladung einlösen",
        Page::ClientSettings => "Client-Einstellungen",
    }
}

#[component]
fn Placeholder(page: Page) -> impl IntoView {
    view! {
        <section class="tab active">
            <h3>{title(page)}</h3>
            <p class="notice">"Diese Seite wird gerade von JavaScript auf Rust umgebaut (Vapafane 0.9.0)."</p>
        </section>
    }
}

/// Aussehen-Auswahl (Grasblock / Crying Obsidian) – im Client- und im Server-Bereich gleich.
#[component]
pub fn SkinChoice() -> impl IntoView {
    let app = use_app();
    let obsidian = move || {
        app.theme_tick.track();
        crate::state::is_obsidian()
    };
    view! {
        <h3>"Aussehen"</h3>
        <div class="skins" role="radiogroup" aria-label="Aussehen">
            <label class="skin-choice"><span class="swatch grass"></span>
                <span><input type="radio" name="skin" value="grass" prop:checked=move || !obsidian()
                    on:change=move |_| set_skin(&app, "grass")/>" Grasblock"</span>
                <span class="dim small">"Hell oder dunkel (Knopf oben rechts)"</span></label>
            <label class="skin-choice"><span class="swatch obsidian"></span>
                <span><input type="radio" name="skin" value="obsidian" prop:checked=obsidian
                    on:change=move |_| set_skin(&app, "obsidian")/>" Crying Obsidian"</span>
                <span class="dim small">"Immer dunkel"</span></label>
        </div>
    }
}

#[component]
pub fn CurrentPage() -> impl IntoView {
    let app = use_app();
    move || match app.page.get() {
        Page::ClientSettings => view! {
            <section class="tab active"><SkinChoice/>
                <p class="notice">"Weitere Client-Einstellungen folgen in Schritt 4."</p></section>
        }
        .into_any(),
        Page::Settings => view! {
            <section class="tab active"><SkinChoice/>
                <p class="notice">"Weitere Einstellungen folgen in Schritt 3."</p></section>
        }
        .into_any(),
        p => view! { <Placeholder page=p/> }.into_any(),
    }
}
