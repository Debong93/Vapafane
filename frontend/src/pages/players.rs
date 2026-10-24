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

/// Hinweise zur Whitelist für Einsteiger. `page`: auf welcher Seite der Hinweis steht (dann wird
/// neu geladen); `tips`: auch den sanften Tipp zeigen, wenn nichts dringend ist.
#[component]
pub fn WhitelistNote(page: Page, #[prop(optional)] tips: bool) -> impl IntoView {
    let app = use_app();
    let st = RwSignal::new(Value::Null);
    let load = move || {
        leptos::task::spawn_local(async move {
            st.set(tauri::call::<Value>("whitelist_state").await.unwrap_or(Value::Null));
        })
    };
    Effect::new(move |_| {
        app.server_path.track();
        app.status.track();
        if app.page.get() == page {
            load();
        }
    });
    let enable = move |_| {
        leptos::task::spawn_local(async move {
            if let Err(e) = tauri::invoke::<()>("set_whitelist", json!({ "on": true })).await {
                error("Whitelist", e).await;
                return;
            }
            if matches!(app.status.get_untracked().state.as_str(), "running" | "external") {
                let _ = run_command("whitelist on", true).await;
            }
            load();
            if st.get_untracked().f("count").unwrap_or(0.0) < 1.0 {
                app.show(Page::Players);
            }
        })
    };
    move || {
        let s = st.get();
        if s.is_null() {
            return None;
        }
        let on = s.b("enabled");
        let empty = s.f("count").unwrap_or(0.0) < 1.0;
        let outside = matches!(crate::pages::reach::light(app).as_str(), "green" | "yellow");
        Some(if !on && outside {
            view! { <div class="hint warn-box">"Achtung: Dein Server ist aus dem Internet erreichbar und die "<b>"Whitelist ist aus"</b>
                " – jeder, der die Adresse kennt oder zufällig findet, kann beitreten. Mit Whitelist kommen nur Spieler rein, die du einträgst."
                <div class="row"><button class="primary" on:click=enable>"Whitelist einschalten"</button></div></div> }.into_any()
        } else if on && empty {
            view! { <div class="hint warn-box">"Die Whitelist ist an, aber noch leer – gerade kommt "<b>"niemand"</b>" auf den Server, auch du nicht. "
                "Trag unter „Spieler“ deinen Minecraft-Namen und die deiner Freunde ein."
                {(page != Page::Players).then(|| view! { <div class="row"><button class="primary" on:click=move |_| app.show(Page::Players)>"Zu den Spielern"</button></div> })}</div> }.into_any()
        } else if !on && tips {
            view! { <div class="hint">"Tipp: Die Whitelist ist aus – jeder mit der Adresse kann beitreten. Empfohlen ist sie, sobald Freunde von außen spielen: "
                "Dann kommen nur Spieler rein, die hier eingetragen sind."
                <div class="row"><button on:click=enable>"Whitelist einschalten"</button></div></div> }.into_any()
        } else {
            return None;
        })
    }
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
            <h3>{crate::sprache::t!("spi-online-titel")}</h3>
            <div class="list">{move || online_rows(app)}</div>
            <h3>{crate::sprache::t!("spi-wl-titel")}</h3>
            <p class="dim small">"Ist die Whitelist an (Einstellungen → „Nur Whitelist darf rein“), kommen nur die Spieler hier auf den Server. Das schützt vor Fremden – trag dich selbst auch ein."</p>
            <WhitelistNote page=Page::Players tips=true/>
            <div class="row">
                <input placeholder=crate::sprache::t!("spi-name-platzhalter") autocomplete="off" spellcheck="false" prop:value=input
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
        return view! { <div class="empty">{crate::sprache::t!("spi-niemand")}</div> }.into_any();
    }
    let ops: Vec<String> = names(&live, "ops")
        .into_iter()
        .map(|o| o.to_lowercase())
        .collect();
    players.into_iter().map(|name| {
        let op = ops.contains(&name.to_lowercase());
        let actions = [
            (crate::sprache::t!("spi-kreativ"), format!("gamemode creative {name}"), false),
            (crate::sprache::t!("spi-ueberleben"), format!("gamemode survival {name}"), false),
            (if op { crate::sprache::t!("spi-op-weg") } else { crate::sprache::t!("spi-op") }, format!("{} {name}", if op { "deop" } else { "op" }), false),
            (crate::sprache::t!("spi-kick"), format!("kick {name}"), true),
        ];
        let n = name.clone();
        view! {
            <div class="item">
                <div class="grow">{name.clone()}{op.then(|| view! { <span class="badge">"OP"</span> })}</div>
                {actions.into_iter().map(|(label, cmd, is_kick)| {
                    let n = n.clone();
                    view! { <button on:click=move |_| {
                        let (n, cmd) = (n.clone(), cmd.clone());
                        leptos::task::spawn_local(async move {
                            if is_kick && !confirm(crate::sprache::t!("spi-kick-frage", name = n), crate::sprache::t!("spi-kick-text"), crate::sprache::t!("spi-kick")).await {
                                return;
                            }
                            if let Err(e) = run_command(&cmd, false).await {
                                error(crate::sprache::t!("spi-err-befehl"), e).await;
                            }
                            set_timeout(move || app.refresh_live(), Duration::from_millis(500));
                        });
                    }>{label.clone()}</button> }
                }).collect_view()}
            </div>
        }
    }).collect_view().into_any()
}
