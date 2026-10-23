//! Konsole: Server-Log live, Befehle senden, Schnellbefehle. Erkennt Beitritte im Log.

use leptos::html::Div;
use leptos::prelude::*;
use serde::Deserialize;
use serde_json::json;
use std::cell::RefCell;
use std::time::Duration;

use crate::state::{AppState, Page, use_app};
use crate::tauri;

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub id: u64,
    pub text: String,
    pub class: &'static str,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct LogEvent {
    reset: bool,
    initial: bool,
    lines: Vec<String>,
}

thread_local! {
    static LINES: RwSignal<Vec<Line>> = RwSignal::new(vec![]);
    static NEXT: RefCell<u64> = const { RefCell::new(0) };
    static HISTORY: RefCell<(Vec<String>, usize)> = const { RefCell::new((Vec::new(), 0)) };
    static VIEW: RefCell<Option<NodeRef<Div>>> = const { RefCell::new(None) };
}

/// Beim Start (im Besitz der App) anlegen, damit der Log nie weggeräumt wird.
pub fn init() {
    let _ = lines();
}

fn lines() -> RwSignal<Vec<Line>> {
    LINES.with(|l| *l)
}

/// Farbe einer Log-Zeile.
fn classify(line: &str) -> &'static str {
    if line.contains("/ERROR]") || line.contains("Exception") {
        "error"
    } else if join_of(line).is_some() {
        "join"
    } else if chat_of(line) {
        "chat"
    } else if line.contains("/WARN]") {
        "warn"
    } else {
        ""
    }
}

fn valid_name(n: &str) -> bool {
    (3..=16).contains(&n.len()) && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// „]: Name joined the game“ → (Name, beigetreten?)
fn join_of(line: &str) -> Option<(String, bool)> {
    let rest = &line[line.find("]: ")? + 3..];
    for (suffix, joined) in [(" joined the game", true), (" left the game", false)] {
        if let Some(name) = rest
            .strip_suffix(suffix)
            .or_else(|| rest.split_once(suffix).map(|(n, _)| n))
            && valid_name(name)
        {
            return Some((name.to_string(), joined));
        }
    }
    None
}

fn chat_of(line: &str) -> bool {
    line.find("]: <")
        .and_then(|i| line[i + 4..].split_once("> "))
        .is_some_and(|(n, _)| valid_name(n))
}

/// „]: Done (3.123s)!“ → „3,123“
fn done_of(line: &str) -> Option<String> {
    let rest = &line[line.find("]: Done (")? + 9..];
    let secs = rest.split_once("s)!")?.0;
    secs.chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == ',')
        .then(|| secs.replace('.', ","))
}

fn at_bottom(el: &web_sys::HtmlElement) -> bool {
    el.scroll_top() + el.client_height() >= el.scroll_height() - 30
}

fn scroll_down() {
    let r = VIEW.with(|v| *v.borrow());
    if let Some(el) = r.and_then(|r| r.get_untracked()) {
        el.set_scroll_top(el.scroll_height());
    }
}

/// Zeilen anhängen (höchstens 3000 behalten); bleibt unten kleben, wenn man unten war.
pub fn append(new: Vec<String>, class: Option<&'static str>) {
    let r = VIEW.with(|v| *v.borrow());
    let stick = r
        .and_then(|r| r.get_untracked())
        .map(|el| at_bottom(&el))
        .unwrap_or(true);
    lines().update(|l| {
        for text in new {
            let id = NEXT.with(|n| {
                *n.borrow_mut() += 1;
                *n.borrow()
            });
            let class = class.unwrap_or_else(|| classify(&text));
            l.push(Line { id, text, class });
        }
        let over = l.len().saturating_sub(3000);
        l.drain(..over);
    });
    if stick {
        request_animation_frame(scroll_down);
    }
}

/// Anderer Server: Log neu einlesen lassen.
pub fn reset_log() {
    leptos::task::spawn_local(async {
        let _ = tauri::call::<()>("log_reset").await;
    });
}

/// Ereignisse aus neuen Log-Zeilen (Beitritt, „Done“).
fn log_events(app: AppState, new: &[String]) {
    for line in new {
        if let Some((name, joined)) = join_of(line) {
            let text = if joined {
                format!("{name} ist beigetreten")
            } else {
                format!("{name} hat das Spiel verlassen")
            };
            app.add_event(text.clone());
            crate::util::notify(&text, "");
            set_timeout(move || app.refresh_live(), Duration::from_millis(800));
        } else if let Some(s) = done_of(line) {
            app.add_event(format!("Server bereit (Start in {s} s)"));
            set_timeout(move || app.refresh_live(), Duration::from_millis(500));
        }
    }
}

/// Einmal beim Start: auf neue Log-Zeilen hören.
pub fn listen_log() {
    let app = use_app();
    tauri::listen::<LogEvent>("log", move |ev| {
        if ev.reset && ev.initial {
            lines().set(vec![]);
        }
        if ev.reset && !ev.initial {
            append(
                vec!["––– neue Log-Datei (Server wurde neu gestartet) –––".into()],
                Some("sys"),
            );
        }
        let initial = ev.initial;
        if !initial {
            log_events(app, &ev.lines);
        }
        append(ev.lines, None);
        if initial {
            request_animation_frame(scroll_down);
        }
    });
}

/// Befehl per RCON schicken. `quiet`: nicht in der Konsole anzeigen.
pub async fn run_command(cmd: &str, quiet: bool) -> Result<String, String> {
    if !quiet {
        append(vec![format!("> {cmd}")], Some("cmd"));
    }
    match tauri::invoke::<String>("send_command", json!({ "cmd": cmd })).await {
        Ok(resp) => {
            if !quiet && !resp.is_empty() {
                append(resp.lines().map(String::from).collect(), Some("resp"));
            }
            Ok(resp)
        }
        Err(e) => {
            if !quiet {
                append(vec![format!("Fehler: {e}")], Some("error"));
            }
            Err(e)
        }
    }
}

pub fn send(cmd: String) {
    leptos::task::spawn_local(async move {
        let _ = run_command(&cmd, false).await;
    });
}

#[component]
pub fn Console() -> impl IntoView {
    let app = use_app();
    let view_ref = NodeRef::<Div>::new();
    VIEW.with(|v| *v.borrow_mut() = Some(view_ref));
    Effect::new(move |_| {
        if app.page.get() == Page::Console {
            request_animation_frame(scroll_down);
        }
    });
    let input = RwSignal::new(String::new());
    let can_send = move || matches!(app.status.get().state.as_str(), "running" | "external");
    let submit = move || {
        let cmd = input.get_untracked().trim().to_string();
        if cmd.is_empty() {
            return;
        }
        HISTORY.with(|h| {
            let mut h = h.borrow_mut();
            h.0.push(cmd.clone());
            h.1 = h.0.len();
        });
        input.set(String::new());
        send(cmd);
        request_animation_frame(scroll_down);
    };
    let on_key = move |e: web_sys::KeyboardEvent| {
        let step: i32 = match e.key().as_str() {
            "Enter" => return submit(),
            "ArrowUp" => -1,
            "ArrowDown" => 1,
            _ => return,
        };
        e.prevent_default();
        let v = HISTORY.with(|h| {
            let mut h = h.borrow_mut();
            if h.0.is_empty() {
                return None;
            }
            h.1 = (h.1 as i32 + step).clamp(0, h.0.len() as i32) as usize;
            Some(h.0.get(h.1).cloned().unwrap_or_default())
        });
        if let Some(v) = v {
            input.set(v);
        }
    };
    let quick = [
        ("Tag", "time set day"),
        ("Nacht", "time set night"),
        ("Klares Wetter", "weather clear"),
        ("Speichern", "save-all"),
        ("TPS", "tick query"),
        ("Wer ist online?", "list"),
    ];
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Console id="tab-console">
            <div class="console" node_ref=view_ref>
                <For each=move || lines().get() key=|l| l.id
                    children=|l| view! { <div class=l.class>{l.text}</div> }/>
            </div>
            <div class="cmd-row">
                <input placeholder="Befehl eingeben, z. B. say Hallo  ·  time set day  ·  weather clear"
                    autocomplete="off" spellcheck="false" disabled=move || !can_send()
                    prop:value=input on:input=move |e| input.set(event_target_value(&e)) on:keydown=on_key/>
                <button class="primary" disabled=move || !can_send() on:click=move |_| submit()>"Senden"</button>
            </div>
            <div class="quick">
                {quick.into_iter().map(|(l, c)| view! { <button on:click=move |_| send(c.to_string())>{l}</button> }).collect_view()}
                <span class="spacer"></span>
                <button on:click=|_| crate::util::open_folder("logs")>"Log-Ordner öffnen"</button>
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_erkennung() {
        assert_eq!(
            join_of("[12:00:01] [Server thread/INFO]: Steve joined the game"),
            Some(("Steve".into(), true))
        );
        assert_eq!(
            join_of("[12:00:01] [Server thread/INFO]: Alex_2 left the game"),
            Some(("Alex_2".into(), false))
        );
        assert_eq!(
            join_of("[12:00:01] [Server thread/INFO]: <Steve> joined the game"),
            None
        );
        assert!(chat_of("[12:00:01] [Server thread/INFO]: <Steve> hallo"));
        assert_eq!(
            done_of("[12:00:01] [Server thread/INFO]: Done (3.123s)! For help, type \"help\""),
            Some("3,123".into())
        );
        assert_eq!(classify("[x/ERROR]: kaputt"), "error");
        assert_eq!(classify("[x/WARN]: hm"), "warn");
    }
}
