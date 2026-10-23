//! Zustand der Oberfläche: Bereich (Client/Server), Seite, Server-Status, Thema.

use leptos::prelude::*;
use serde_json::Value;
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

/// Ein Eintrag unter „Letzte Ereignisse“.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: u64,
    pub time: String,
    pub text: String,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub mode: RwSignal<Mode>,
    pub page: RwSignal<Page>,
    /// Zuletzt gezeigte Seite im Server-Bereich
    pub server_page: RwSignal<Page>,
    pub status: RwSignal<Status>,
    /// Pfad des aktiven Servers (ändert sich nur beim Wechsel – anders als `status`)
    pub server_path: Memo<Option<String>>,
    /// Ändert sich bei jedem Themenwechsel (damit abhängige Anzeigen neu rechnen)
    pub theme_tick: RwSignal<u32>,
    pub events: RwSignal<Vec<Event>>,
    /// Spieler online, TPS … (`get_live`)
    pub live: RwSignal<Value>,
    /// Ordnergröße, Adresse, letztes Backup … (`get_details`)
    pub details: RwSignal<Value>,
    /// Erhöht sich, wenn sich Backups geändert haben könnten
    pub backups_tick: RwSignal<u32>,
    /// Erreichbarkeit: letztes Ergebnis (`reach_check`)
    pub reach: RwSignal<Option<Value>>,
    pub reach_server: RwSignal<bool>,
    pub reach_busy: RwSignal<bool>,
    pub reach_checked: RwSignal<bool>,
    /// Gelb/Rot schon angeschaut (dann kein Blinken mehr)
    pub reach_seen: RwSignal<String>,
    /// Verlauf für die kleinen Kurven (TPS, CPU, RAM in GB)
    pub spark_tps: RwSignal<Vec<f64>>,
    pub spark_cpu: RwSignal<Vec<f64>>,
    pub spark_ram: RwSignal<Vec<f64>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let status = RwSignal::new(Status::default());
        AppState {
            mode: RwSignal::new(Mode::Client),
            page: RwSignal::new(Page::Client),
            server_page: RwSignal::new(Page::Overview),
            status,
            server_path: Memo::new(move |_| {
                status.with(|s| s.server.as_ref().map(|s| s.path.clone()))
            }),
            theme_tick: RwSignal::new(0),
            events: RwSignal::new(vec![]),
            live: RwSignal::new(Value::Null),
            details: RwSignal::new(Value::Null),
            backups_tick: RwSignal::new(0),
            reach: RwSignal::new(None),
            reach_server: RwSignal::new(true),
            reach_busy: RwSignal::new(false),
            reach_checked: RwSignal::new(false),
            reach_seen: RwSignal::new(stored("reach_seen").unwrap_or_default()),
            spark_tps: RwSignal::new(vec![]),
            spark_cpu: RwSignal::new(vec![]),
            spark_ram: RwSignal::new(vec![]),
        }
    }

    pub fn set_mode(&self, m: Mode) {
        self.mode.set(m);
        self.page.set(match m {
            Mode::Client => Page::Client,
            Mode::Server => self.server_page.get_untracked(),
        });
        if m == Mode::Server {
            crate::pages::reach::reach_init(*self);
        }
    }

    pub fn show(&self, p: Page) {
        if self.mode.get_untracked() == Mode::Server {
            self.server_page.set(p);
        }
        self.page.set(p);
        if p == Page::Reach {
            crate::pages::reach::mark_seen(*self);
        }
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

    pub fn add_event(&self, text: impl Into<String>) {
        let text = text.into();
        self.events.update(|v| {
            let id = v.first().map(|e| e.id + 1).unwrap_or(1);
            v.insert(
                0,
                Event {
                    id,
                    time: crate::util::clock(),
                    text,
                },
            );
            v.truncate(50);
        });
    }

    pub fn refresh_status(&self) {
        let me = *self;
        leptos::task::spawn_local(async move {
            if let Ok(st) = tauri::call::<Status>("get_status").await {
                crate::pages::overview::on_status(me, &st);
                me.status.set(st);
            }
        });
    }

    pub fn refresh_details(&self) {
        let details = self.details;
        leptos::task::spawn_local(async move {
            if let Ok(d) = tauri::call::<Value>("get_details").await {
                details.set(d);
            }
        });
    }

    pub fn refresh_live(&self) {
        let me = *self;
        leptos::task::spawn_local(async move { crate::pages::overview::refresh_live(me).await });
    }

    /// Status alle 2 s, Spieler/TPS alle 5 s, Details jede Minute.
    pub fn start_polling(&self) {
        let me = *self;
        me.refresh_status();
        me.refresh_details();
        me.refresh_live();
        set_interval(move || me.refresh_status(), Duration::from_secs(2));
        set_interval(move || me.refresh_live(), Duration::from_secs(5));
        set_interval(move || me.refresh_details(), Duration::from_secs(60));
    }
}

/// Wert an eine Kurve anhängen (höchstens 150 Punkte).
pub fn push_spark(s: RwSignal<Vec<f64>>, v: f64) {
    s.update(|x| {
        x.push(v);
        if x.len() > 150 {
            x.remove(0);
        }
    });
}

thread_local! {
    static APP: std::cell::OnceCell<AppState> = const { std::cell::OnceCell::new() };
}

/// Der eine Zustand der Oberfläche – überall erreichbar (auch in Hintergrundaufgaben).
/// Wird beim ersten Aufruf in `App` angelegt und gehört damit der App.
pub fn use_app() -> AppState {
    APP.with(|a| *a.get_or_init(AppState::new))
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
