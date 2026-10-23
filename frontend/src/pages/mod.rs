//! Seiten der Oberfläche.

pub mod backups;
pub mod client;
pub mod console;
pub mod invite;
pub mod mods;
pub mod overview;
pub mod players;
pub mod reach;
pub mod server_dialogs;
pub mod settings;
pub mod updates;

use leptos::prelude::*;
use serde_json::{Value, json};

use crate::dialog::show_error;
use crate::state::{Page, set_skin, use_app};
use crate::tauri;
use crate::util::{J, open_link};

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

/// Einstellungen im Client-Bereich (nur, was ein Spieler braucht).
#[component]
fn ClientSettings() -> impl IntoView {
    let app = use_app();
    let s = RwSignal::new(Value::Null);
    Effect::new(move |_| {
        if app.page.get() == Page::ClientSettings {
            leptos::task::spawn_local(async move {
                if let Ok(v) = tauri::call::<Value>("client_settings").await {
                    s.set(v);
                }
            });
        }
    });
    let set_target = move |e: leptos::ev::Event| {
        let t = event_target_value(&e);
        leptos::task::spawn_local(async move {
            if let Err(er) = tauri::invoke::<()>("set_client_target", json!({ "target": t })).await
            {
                show_error("Einstellung", er);
            }
        });
    };
    let open_base = move |_| {
        leptos::task::spawn_local(async move {
            if let Err(e) = tauri::call::<()>("client_open_base").await {
                show_error("Ordner", e);
            }
        })
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::ClientSettings>
            <SkinChoice/>
            <h3>"Einrichten"</h3>
            <div class="switches">
                <div class="sw-row"><div><div>"Standard-Ziel"</div><div class="dim small">"Wo neue Server eingerichtet werden"</div></div>
                    <select prop:value=move || { let t = s.get().s("target"); if t.is_empty() { "launcher".into() } else { t } } on:change=set_target>
                        <option value="launcher">"Offizieller Minecraft-Launcher"</option><option value="modrinth">"Modrinth-App"</option>
                    </select></div>
                <div class="sw-row"><div><div>"Ordner der eingerichteten Server"</div><div class="dim small">"Mods und Einstellungen je Server"</div></div>
                    <button on:click=open_base>"Ordner öffnen"</button></div>
            </div>
            <h3>"Über Vapafane"</h3>
            <div class="switches">
                <div class="sw-row"><div><div>{move || format!("Version {}", s.get().s("version"))}</div>
                    <div class="dim small">"Kein offizielles Minecraft-Produkt; nicht von Mojang oder Microsoft genehmigt oder mit ihnen verbunden."</div></div>
                    <button on:click=|_| open_link("issues")>"Kontakt (GitHub) ↗"</button></div>
            </div>
        </section>
    }
}

/// Alle Seiten bleiben bestehen und werden nur ein-/ausgeblendet (wie Reiter) –
/// so behalten sie ihren Zustand, und laufende Abfragen finden ihre Daten noch vor.
#[component]
pub fn CurrentPage() -> impl IntoView {
    view! {
        <overview::Overview/>
        <console::Console/>
        <players::Players/>
        <mods::Mods/>
        <updates::Updates/>
        <backups::Backups/>
        <reach::Reach/>
        <settings::Settings/>
        <client::ClientPage/>
        <ClientSettings/>
    }
}
