//! Dialoge (Modal): Text- oder eigene Inhalte, Knöpfe, Ergebnis per `await`.

use futures_channel::oneshot;
use leptos::ev;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Ein Knopf im Dialog. `value` kommt als Ergebnis zurück.
#[derive(Clone)]
pub struct Btn {
    pub label: String,
    pub value: &'static str,
    pub kind: &'static str,
    /// Nur klickbar, wenn `true` (z. B. erst nach dem EULA-Häkchen)
    pub enabled: Option<Signal<bool>>,
}

impl Btn {
    pub fn new(label: impl Into<String>, value: &'static str) -> Btn {
        Btn {
            label: label.into(),
            value,
            kind: "",
            enabled: None,
        }
    }
    pub fn primary(label: impl Into<String>, value: &'static str) -> Btn {
        Btn {
            kind: "primary",
            ..Btn::new(label, value)
        }
    }
    pub fn when(mut self, s: Signal<bool>) -> Btn {
        self.enabled = Some(s);
        self
    }
}

pub type BodyFn = Rc<dyn Fn() -> AnyView>;

#[derive(Clone)]
struct Shown {
    title: String,
    text: Option<String>,
    body: Option<BodyFn>,
    buttons: Vec<Btn>,
    wide: bool,
}

thread_local! {
    static OPEN: RwSignal<Option<Shown>, LocalStorage> = RwSignal::new_local(None);
    static RESOLVE: RefCell<Option<oneshot::Sender<String>>> = const { RefCell::new(None) };
}

/// Beim Start (im Besitz der App) anlegen, damit der Zustand nie weggeräumt wird.
pub fn init() {
    let _ = open();
}

fn open() -> RwSignal<Option<Shown>, LocalStorage> {
    OPEN.with(|o| *o)
}

/// Dialog schließen und `value` zurückgeben ("" = abgebrochen).
pub fn close(value: &str) {
    open().set(None);
    if let Some(tx) = RESOLVE.with(|r| r.borrow_mut().take()) {
        let _ = tx.send(value.to_string());
    }
}

pub fn is_open() -> bool {
    open().with_untracked(|o| o.is_some())
}

async fn show(s: Shown) -> String {
    // Ein noch offener Dialog wird abgebrochen
    close("");
    let (tx, rx) = oneshot::channel();
    RESOLVE.with(|r| *r.borrow_mut() = Some(tx));
    open().set(Some(s));
    rx.await.unwrap_or_default()
}

/// Dialog mit Text.
pub async fn ask(title: impl Into<String>, text: impl Into<String>, buttons: Vec<Btn>) -> String {
    show(Shown {
        title: title.into(),
        text: Some(text.into()),
        body: None,
        buttons,
        wide: false,
    })
    .await
}

/// Dialog mit eigenem Inhalt.
pub async fn ask_view(
    title: impl Into<String>,
    body: BodyFn,
    buttons: Vec<Btn>,
    wide: bool,
) -> String {
    show(Shown {
        title: title.into(),
        text: None,
        body: Some(body),
        buttons,
        wide,
    })
    .await
}

pub async fn info(title: impl Into<String>, text: impl Into<String>) {
    ask(title, text, vec![Btn::primary("OK", "ok")]).await;
}

pub async fn error(title: impl Into<String>, text: impl Into<String>) {
    ask(title, text, vec![Btn::primary("Schließen", "ok")]).await;
}

pub async fn confirm(title: impl Into<String>, text: impl Into<String>, yes: &str) -> bool {
    ask(
        title,
        text,
        vec![Btn::new("Abbrechen", ""), Btn::primary(yes, "yes")],
    )
    .await
        == "yes"
}

/// Fehler zeigen, ohne darauf zu warten (für Klick-Handler).
pub fn show_error(title: &'static str, text: impl Into<String>) {
    let text = text.into();
    leptos::task::spawn_local(async move { error(title, text).await });
}

#[component]
pub fn Modal() -> impl IntoView {
    let state = open();
    let _ = window_event_listener(ev::keydown, move |e| {
        if e.key() == "Escape" && is_open() {
            close("");
        }
    });
    move || {
        state.get().map(|s| {
            let body = match (&s.text, &s.body) {
                (_, Some(f)) => f(),
                (Some(t), None) => view! { <div class="modal-text">{t.clone()}</div> }.into_any(),
                _ => ().into_any(),
            };
            let buttons = s
                .buttons
                .iter()
                .map(|b| {
                    let v = b.value;
                    let en = b.enabled;
                    view! {
                        <button class=b.kind disabled=move || en.is_some_and(|e| !e.get())
                            on:click=move |_| close(v)>{b.label.clone()}</button>
                    }
                })
                .collect_view();
            view! {
                <div class="modal-back">
                    <div class="modal" class:wide=s.wide role="dialog" aria-modal="true">
                        <h2>{s.title.clone()}</h2>
                        <div id="modal-body">{body}</div>
                        <div class="modal-buttons">{buttons}</div>
                    </div>
                </div>
            }
        })
    }
}
