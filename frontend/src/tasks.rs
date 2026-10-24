//! Lange Aufgaben im Backend (Backup, Update, Einrichten …): Fortschrittsleiste unten
//! und Warten auf das Ende über das Ereignis „task“.

use futures_channel::oneshot;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::VecDeque;

use crate::state::use_app;
use crate::tauri;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
struct TaskEvent {
    title: String,
    frac: Option<f64>,
    text: Option<String>,
    done: bool,
    error: Option<String>,
    result: Value,
}

#[derive(Clone, Default, PartialEq)]
struct Bar {
    visible: bool,
    title: String,
    frac: f64,
    text: String,
}

type Waiter = oneshot::Sender<Result<Value, String>>;

thread_local! {
    static BAR: RwSignal<Bar> = RwSignal::new(Bar::default());
    static WAITERS: RefCell<VecDeque<Waiter>> = const { RefCell::new(VecDeque::new()) };
}

/// Beim Start (im Besitz der App) anlegen, damit der Zustand nie weggeräumt wird.
pub fn init() {
    let _ = bar();
}

fn bar() -> RwSignal<Bar> {
    BAR.with(|b| *b)
}

/// Aufgabe starten und auf ihr Ergebnis warten.
pub async fn task(cmd: &str, args: impl Serialize) -> Result<Value, String> {
    tauri::invoke::<Value>(cmd, args).await?;
    bar().update(|b| {
        b.visible = true;
        b.frac = 0.0;
        b.text.clear();
    });
    use_app().refresh_status();
    let (tx, rx) = oneshot::channel();
    WAITERS.with(|w| w.borrow_mut().push_back(tx));
    rx.await.unwrap_or_else(|_| Err(crate::sprache::t!("task-abgebrochen")))
}

/// Einmal beim Start: auf Fortschritt und Ende der Aufgaben hören.
pub fn listen_tasks() {
    let app = use_app();
    tauri::listen::<TaskEvent>("task", move |t| {
        bar().update(|b| {
            b.visible = !t.done;
            b.title = t.title.clone();
            if let Some(f) = t.frac {
                b.frac = f;
            }
            if let Some(x) = &t.text {
                b.text = x.clone();
            }
        });
        if t.done {
            if let Some(w) = WAITERS.with(|w| w.borrow_mut().pop_front()) {
                let _ = w.send(match t.error {
                    Some(e) => Err(e),
                    None => Ok(t.result),
                });
            }
            app.refresh_status();
            app.refresh_details();
            app.backups_tick.update(|n| *n += 1);
        }
    });
}

#[component]
pub fn Taskbar() -> impl IntoView {
    let b = bar();
    view! {
        <footer class="taskbar" class:hidden=move || !b.get().visible>
            <div class="spinner"></div>
            <div class="task-texts">
                <div>{move || b.get().title}</div>
                <div class="dim small">{move || b.get().text}</div>
            </div>
            <div class="progress"><div style:width=move || format!("{}%", (b.get().frac * 100.0).round())></div></div>
        </footer>
    }
}
