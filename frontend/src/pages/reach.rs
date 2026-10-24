//! Erreichbarkeit: Ampel für den eigenen Server (die Prüfung selbst macht das Backend).

use leptos::prelude::*;
use serde_json::Value;

use crate::state::{AppState, Page, store, use_app};
use crate::tauri;
use crate::util::{J, clock_of, open_link};

/// Beim Wechsel in den Server-Bereich: letztes Ergebnis zeigen und einmal pro Start still prüfen.
pub fn reach_init(app: AppState) {
    leptos::task::spawn_local(async move {
        let Ok(d) = tauri::call::<Value>("reach_last").await else {
            return;
        };
        let r = d.o("report");
        app.reach.set((!r.is_null()).then_some(r));
        app.reach_server.set(d.b("server"));
        if d.b("auto") && !app.reach_checked.get_untracked() {
            app.reach_checked.set(true);
            run_reach(app);
        }
    });
}

pub fn run_reach(app: AppState) {
    if app.reach_busy.get_untracked() {
        return;
    }
    app.reach_busy.set(true);
    leptos::task::spawn_local(async move {
        match tauri::call::<Value>("reach_check").await {
            Ok(r) => app.reach.set(Some(r)),
            Err(e) => app.reach.update(|r| {
                if r.is_none() {
                    *r = Some(serde_json::json!({ "light": "gray", "title": crate::sprache::t!("rch-fehlgeschlagen"), "text": e }));
                }
            }),
        }
        app.reach_busy.set(false);
        if app.page.get_untracked() == Page::Reach {
            mark_seen(app);
        }
    });
}

/// Reiter angeschaut: Blinken für diesen Zustand beenden.
pub fn mark_seen(app: AppState) {
    let light = app
        .reach
        .get_untracked()
        .map(|r| r.s("light"))
        .unwrap_or_default();
    store("reach_seen", Some(&light));
    app.reach_seen.set(light);
}

/// Licht für Punkt und Blinken in der Seitenleiste.
pub fn light(app: AppState) -> String {
    app.reach
        .get()
        .map(|r| r.s("light"))
        .unwrap_or_else(|| "none".into())
}

pub fn alarm(app: AppState) -> bool {
    let l = light(app);
    matches!(l.as_str(), "yellow" | "red") && app.reach_seen.get() != l
}

fn pill(state: &str) -> &'static str {
    match state {
        "ok" => "pill ok",
        "warn" => "pill warn",
        "bad" => "pill bad",
        _ => "pill",
    }
}

#[component]
pub fn Reach() -> impl IntoView {
    let app = use_app();
    let r = app.reach;
    let title = move || match r.get() {
        Some(r) => r.s("title"),
        None if app.reach_server.get() => crate::sprache::t!("rch-noch-nicht"),
        None => crate::sprache::t!("rch-kein-server"),
    };
    let text = move || match r.get() {
        Some(r) => r.s("text"),
        None if app.reach_server.get() => String::new(),
        None => crate::sprache::t!("rch-kein-server-text"),
    };
    let row = move |k: &'static str| {
        view! {
            <span class=move || r.get().map(|r| pill(&r.s(&format!("{k}_state")))).unwrap_or("pill")>
                {move || r.get().map(|r| r.s(k)).unwrap_or_else(|| "–".into())}
            </span>
        }
    };
    let when = move || {
        if app.reach_busy.get() {
            return crate::sprache::t!("rch-prueft");
        }
        r.get()
            .and_then(|r| r.f("checked"))
            .map(|c| crate::sprache::t!("rch-geprueft-um", zeit = clock_of(c)))
            .unwrap_or_default()
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Reach>
            <div class="reach-top">
                <div class="ampel" aria-hidden="true" data-light=move || light(app)>
                    <span class="lamp red"></span><span class="lamp yellow"></span><span class="lamp green"></span>
                </div>
                <div class="reach-msg">
                    <div class="reach-title">{title}</div>
                    <div class="dim">{text}</div>
                </div>
            </div>
            <crate::pages::players::WhitelistNote page=Page::Reach/>
            <div class="reach-rows">
                <div class="reach-row"><span>"IPv4"</span>{row("v4")}</div>
                <div class="reach-row"><span>"IPv6"</span>{row("v6")}</div>
                <p class="dim small">{crate::sprache::t!("rch-ip-erklaerung")}</p>
            </div>
            <div class="row">
                <button class="primary" disabled=move || app.reach_busy.get() || !app.reach_server.get()
                    on:click=move |_| run_reach(app)>{crate::sprache::t!("rch-erneut")}</button>
                <Show when=move || r.get().is_some_and(|r| r.b("guide"))>
                    <button on:click=|_| open_link("portforward")>{crate::sprache::t!("rch-portfreigabe")}</button>
                </Show>
                <span class="dim small">{when}</span>
            </div>
            <p class="dim small">{crate::sprache::t!("rch-dienst-hinweis")}</p>
        </section>
    }
}
