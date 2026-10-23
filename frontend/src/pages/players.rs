//! Spieler: wer gerade online ist (mit Schnellbefehlen) und die Whitelist.

use leptos::prelude::*;
use serde_json::{Value, json};
use std::time::Duration;

use crate::dialog::{confirm, error};
use crate::pages::console::run_command;
use crate::state::{AppState, Page, use_app};
use crate::tauri;
use crate::util::J;

fn names(v: &Value, key: &str) -> Vec<String> {
    v.a(key)
        .iter()
        .filter_map(|p| p.as_str().map(String::from))
        .collect()
}

fn valid(name: &str) -> bool {
    (3..=16).contains(&name.len()) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[component]
pub fn Players() -> impl IntoView {
    let app = use_app();
    let wl = RwSignal::new(Vec::<String>::new());
    let hint = RwSignal::new(String::new());
    let input = RwSignal::new(String::new());
    let reload = move || {
        leptos::task::spawn_local(async move {
            wl.set(
                tauri::call::<Vec<String>>("get_whitelist")
                    .await
                    .unwrap_or_default(),
            );
        });
    };
    Effect::new(move |_| {
        app.server_path.track();
        if app.page.get() == Page::Players {
            reload();
            app.refresh_live();
        }
    });
    let reload_soon = move || set_timeout(reload, Duration::from_millis(700));
    let running = move || {
        matches!(
            app.status.get_untracked().state.as_str(),
            "running" | "external"
        )
    };

    let add = move || {
        let name = input.get_untracked().trim().to_string();
        if !valid(&name) {
            hint.set("Bitte einen gültigen Minecraft-Namen eingeben (3–16 Zeichen).".into());
            return;
        }
        if !running() {
            hint.set(
                "Hinzufügen geht nur, während der Server läuft (er prüft den Namen bei Mojang)."
                    .into(),
            );
            return;
        }
        input.set(String::new());
        leptos::task::spawn_local(async move {
            hint.set(
                run_command(&format!("whitelist add {name}"), true)
                    .await
                    .unwrap_or_else(|e| e),
            );
            reload_soon();
        });
    };
    let remove = move |name: String| {
        leptos::task::spawn_local(async move {
            let r = if running() {
                run_command(&format!("whitelist remove {name}"), true).await
            } else {
                tauri::invoke::<()>("whitelist_remove_offline", json!({ "name": name }))
                    .await
                    .map(|_| format!("{name} entfernt."))
            };
            hint.set(r.unwrap_or_else(|e| format!("Fehler: {e}")));
            reload_soon();
        });
    };

    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Players>
            <h3>"Gerade online"</h3>
            <div class="list">{move || online_rows(app)}</div>
            <h3>"Whitelist (wer darf rein)"</h3>
            <div class="row">
                <input placeholder="Minecraft-Name" autocomplete="off" spellcheck="false" prop:value=input
                    on:input=move |e| input.set(event_target_value(&e))
                    on:keydown=move |e| if e.key() == "Enter" { add() }/>
                <button class="primary" on:click=move |_| add()>"Hinzufügen"</button>
            </div>
            <p class="dim small">{hint}</p>
            <div class="list">
                {move || {
                    let list = wl.get();
                    if list.is_empty() {
                        view! { <div class="empty">"Die Whitelist ist leer."</div> }.into_any()
                    } else {
                        list.into_iter().map(|n| {
                            let n2 = n.clone();
                            view! { <div class="item"><div class="grow">{n}</div>
                                <button on:click=move |_| remove(n2.clone())>"Entfernen"</button></div> }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </section>
    }
}

fn online_rows(app: AppState) -> AnyView {
    let live = app.live.get();
    let players = names(&live, "players");
    if players.is_empty() {
        return view! { <div class="empty">"Niemand online"</div> }.into_any();
    }
    let ops: Vec<String> = names(&live, "ops")
        .into_iter()
        .map(|o| o.to_lowercase())
        .collect();
    players.into_iter().map(|name| {
        let op = ops.contains(&name.to_lowercase());
        let actions = [
            ("Kreativ", format!("gamemode creative {name}")),
            ("Überleben", format!("gamemode survival {name}")),
            (if op { "OP entziehen" } else { "OP geben" }, format!("{} {name}", if op { "deop" } else { "op" })),
            ("Rauswerfen", format!("kick {name}")),
        ];
        let n = name.clone();
        view! {
            <div class="item">
                <div class="grow">{name.clone()}{op.then(|| view! { <span class="badge">"OP"</span> })}</div>
                {actions.into_iter().map(|(label, cmd)| {
                    let n = n.clone();
                    view! { <button on:click=move |_| {
                        let (n, cmd) = (n.clone(), cmd.clone());
                        leptos::task::spawn_local(async move {
                            if label == "Rauswerfen" && !confirm(format!("{n} rauswerfen?"), "Ein erneuter Beitritt ist sofort möglich.", "Rauswerfen").await {
                                return;
                            }
                            if let Err(e) = run_command(&cmd, false).await {
                                error("Befehl fehlgeschlagen", e).await;
                            }
                            set_timeout(move || app.refresh_live(), Duration::from_millis(500));
                        });
                    }>{label}</button> }
                }).collect_view()}
            </div>
        }
    }).collect_view().into_any()
}
