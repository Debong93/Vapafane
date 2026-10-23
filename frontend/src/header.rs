//! Kopfzeile: Marke, Client/Server, Server-Auswahl mit „+“-Menü, Zahnrad, Hell/Dunkel.

use leptos::ev;
use leptos::prelude::*;
use vapafane_shared::ServerItem;

use crate::mobs::Mobs;
use crate::pages::server_dialogs;
use crate::state::{Mode, Page, toggle_light_dark, use_app};
use crate::tauri;

#[component]
pub fn Header() -> impl IntoView {
    let app = use_app();
    let menu_open = RwSignal::new(false);
    // Klick irgendwo anders schließt das Menü
    let _ = window_event_listener(ev::click, move |_| menu_open.set(false));

    let servers = move || app.status.get().servers;
    let active_path = move || {
        app.status
            .get()
            .servers
            .iter()
            .find(|s| s.active)
            .map(|s| s.path.clone())
            .unwrap_or_default()
    };
    let no_server = move || app.status.get().servers.is_empty();
    let busy = move || {
        let st = app.status.get();
        st.busy.is_some() || st.job.is_some()
    };

    let select_server = move |ev: ev::Event| {
        let path = event_target_value(&ev);
        if path.is_empty() || path == active_path() {
            return;
        }
        let name = app
            .status
            .get_untracked()
            .servers
            .iter()
            .find(|s| s.path == path)
            .map(|s| s.name.clone())
            .unwrap_or_default();
        leptos::task::spawn_local(async move {
            match tauri::invoke::<()>("select_server", serde_json::json!({ "path": path })).await {
                Ok(()) => app.add_event(format!("Server gewechselt: {name}")),
                Err(e) => crate::dialog::error("Umschalten nicht möglich", e).await,
            }
            app.refresh_status();
        });
    };

    let menu_item = move |label: &'static str, action: fn()| {
        view! {
            <button on:click=move |_| {
                menu_open.set(false);
                action();
            }>{label}</button>
        }
    };

    view! {
        <header class="topbar">
            <div class="brand">
                <img src="icon.png" alt=""/>
                <span class="brand-text">
                    <span class="brand-name">"Vapafane"</span>
                    <small>"Minecraft-Werkbank für Server & Client"</small>
                </span>
            </div>
            <div class="mode-switch" role="tablist" aria-label="Bereich">
                <button id="mode-client" role="tab"
                    class:active=move || app.mode.get() == Mode::Client
                    on:click=move |_| app.set_mode(Mode::Client)>"Client"</button>
                <button id="mode-server" role="tab"
                    class:active=move || app.mode.get() == Mode::Server
                    on:click=move |_| app.set_mode(Mode::Server)>"Server"</button>
            </div>
            <div class="picker">
                <select id="server-select" title="Aktiver Server (es läuft immer nur einer)"
                    prop:value=active_path disabled=busy on:change=select_server>
                    <Show when=no_server>
                        <option value="">"(kein Server)"</option>
                    </Show>
                    <For each=servers key=|s: &ServerItem| s.path.clone()
                        children=move |s: ServerItem| view! { <option value=s.path.clone()>{s.name.clone()}</option> }/>
                </select>
                <button id="btn-add-server" class="icon-btn"
                    class:glow=no_server
                    title=move || if no_server() { "Hier neuen Server erstellen" } else { "Server hinzufügen" }
                    on:click=move |e| {
                        e.stop_propagation();
                        menu_open.update(|o| *o = !*o);
                    }>"＋"</button>
                <div id="add-menu" class="menu" class:hidden=move || !menu_open.get()
                    on:click=|e| e.stop_propagation()>
                    {menu_item("Vorhandenen Ordner hinzufügen …", server_dialogs::add_folder)}
                    {menu_item("Server aus ZIP/Backup importieren …", server_dialogs::add_archive)}
                    {menu_item("Neuen Server erstellen (Fabric, Paper, NeoForge, Vanilla) …", server_dialogs::add_new)}
                    <hr/>
                    {menu_item("Ausgewählten Server aus der Liste entfernen", server_dialogs::remove_server)}
                </div>
            </div>
            <Mobs/>
            <button id="btn-gear" class="icon-btn" title="Einstellungen" aria-label="Einstellungen"
                class:on=move || matches!(app.page.get(), Page::Settings | Page::ClientSettings)
                on:click=move |_| app.toggle_settings()>
                <span class="px-ico ico-settings"></span>
            </button>
            <button id="btn-theme" class="icon-btn" title="Hell/Dunkel umschalten" aria-label="Hell/Dunkel umschalten"
                on:click=move |_| toggle_light_dark(&app)>
                <span class="px-ico"></span>
            </button>
        </header>
    }
}
