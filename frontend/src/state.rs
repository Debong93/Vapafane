//! Zustand der Oberfläche: Bereich (Client/Server), Seite, Server-Status, Thema.

use leptos::prelude::*;
use std::time::Duration;
use vapafane_shared::Status;

use crate::tauri;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Client,
    Server,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Overview,
    Console,
    Players,
    Mods,
    Updates,
    Backups,
    Reach,
    Settings,
    Client,
    ClientSettings,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub mode: RwSignal<Mode>,
    pub page: RwSignal<Page>,
    /// Zuletzt gezeigte Seite im Server-Bereich
    pub server_page: RwSignal<Page>,
    pub status: RwSignal<Status>,
    /// Ändert sich bei jedem Themenwechsel (damit abhängige Anzeigen neu rechnen)
    pub theme_tick: RwSignal<u32>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            mode: RwSignal::new(Mode::Client),
            page: RwSignal::new(Page::Client),
            server_page: RwSignal::new(Page::Overview),
            status: RwSignal::new(Status::default()),
            theme_tick: RwSignal::new(0),
        }
    }

    pub fn set_mode(&self, m: Mode) {
        self.mode.set(m);
        self.page.set(match m {
            Mode::Client => Page::Client,
            Mode::Server => self.server_page.get_untracked(),
        });
    }

    pub fn show(&self, p: Page) {
        if self.mode.get_untracked() == Mode::Server {
            self.server_page.set(p);
        }
        self.page.set(p);
    }

    /// Zahnrad: Einstellungen des jeweiligen Bereichs ein- bzw. ausblenden.
    pub fn toggle_settings(&self) {
        match (self.mode.get_untracked(), self.page.get_untracked()) {
            (Mode::Client, Page::ClientSettings) => self.show(Page::Client),
            (Mode::Client, _) => self.show(Page::ClientSettings),
            (Mode::Server, Page::Settings) => self.show(Page::Overview),
            (Mode::Server, _) => self.show(Page::Settings),
        }
    }

    pub fn refresh_status(&self) {
        let status = self.status;
        leptos::task::spawn_local(async move {
            if let Ok(st) = tauri::call::<Status>("get_status").await {
                status.set(st);
            }
        });
    }

    /// Status beim Start und danach alle 2 Sekunden holen.
    pub fn start_polling(&self) {
        let me = *self;
        me.refresh_status();
        set_interval(move || me.refresh_status(), Duration::from_secs(2));
    }
}

pub fn use_app() -> AppState {
    expect_context::<AppState>()
}

// ---------------------------------------------------------------- Thema (wie bisher im localStorage)

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

pub fn stored(key: &str) -> Option<String> {
    storage()?.get_item(key).ok().flatten()
}

pub fn store(key: &str, value: Option<&str>) {
    if let Some(s) = storage() {
        let _ = match value {
            Some(v) => s.set_item(key, v),
            None => s.remove_item(key),
        };
    }
}

fn root() -> Option<web_sys::HtmlElement> {
    web_sys::window()?
        .document()?
        .document_element()?
        .dyn_into()
        .ok()
}

use wasm_bindgen::JsCast;

/// Thema aus dem Speicher auf <html> übertragen (data-theme, data-skin).
pub fn apply_theme() {
    let Some(r) = root() else { return };
    let data = r.dataset();
    match stored("theme") {
        Some(t) => {
            let _ = data.set("theme", &t);
        }
        None => data.delete("theme"),
    }
    if stored("skin").as_deref() == Some("obsidian") {
        let _ = data.set("skin", "obsidian");
    } else {
        data.delete("skin");
    }
}

pub fn system_dark() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .is_some_and(|m| m.matches())
}

pub fn reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches())
}

pub fn is_obsidian() -> bool {
    stored("skin").as_deref() == Some("obsidian")
}

/// Ist gerade „dunkel“ (Nacht) – gespeichert oder vom System?
pub fn is_dark() -> bool {
    is_obsidian()
        || match stored("theme").as_deref() {
            Some("dark") => true,
            Some("light") => false,
            _ => system_dark(),
        }
}

/// Knopf Hell/Dunkel: umschalten und merken.
pub fn toggle_light_dark(app: &AppState) {
    store("theme", Some(if is_dark() { "light" } else { "dark" }));
    apply_theme();
    app.theme_tick.update(|t| *t += 1);
}

pub fn set_skin(app: &AppState, skin: &str) {
    store("skin", Some(skin));
    apply_theme();
    app.theme_tick.update(|t| *t += 1);
}

pub fn document_visible() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .is_some_and(|d| d.visibility_state() == web_sys::VisibilityState::Visible)
}
