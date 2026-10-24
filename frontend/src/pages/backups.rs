//! Backups: Liste, jetzt sichern, wiederherstellen, wie viele aufheben.

use leptos::prelude::*;
use serde_json::{Value, json};

use crate::dialog::{confirm, error, info};
use crate::state::{AppState, Page, use_app};
use crate::tasks::task;
use crate::tauri;
use crate::util::{J, file_name, fmt_bytes, open_folder};

/// Backup (ZIP) wiederherstellen – mit Rückfrage.
pub fn restore(app: AppState, path: String, file: String) {
    leptos::task::spawn_local(async move {
        let name = app
            .status
            .get_untracked()
            .server
            .map(|s| s.name)
            .unwrap_or_default();
        if !confirm(crate::sprache::t!("bak-restore-frage", name = name, datei = file),
            crate::sprache::t!("bak-restore-text"),
            crate::sprache::t!("bak-wiederherstellen")).await
        {
            return;
        }
        match task("restore_backup", json!({ "path": path })).await {
            Ok(r) => {
                app.add_event(crate::sprache::t!("bak-ev-wiederhergestellt"));
                let aside = r.s("aside");
                info(crate::sprache::t!("bak-restore-fertig-titel"), format!("{}{}", crate::sprache::t!("bak-restore-fertig", label = r.s("label")),
                    if aside.is_empty() { String::new() } else { format!("\n\n{}", crate::sprache::t!("bak-restore-daneben", ordner = aside)) })).await;
            }
            Err(e) => error(crate::sprache::t!("bak-err-restore"), e).await,
        }
        crate::pages::console::reset_log();
    });
}

#[component]
pub fn Backups() -> impl IntoView {
    let app = use_app();
    let data = RwSignal::new(Value::Null);
    let keep = RwSignal::new(String::new());
    Effect::new(move |_| {
        app.backups_tick.track();
        app.server_path.track();
        if app.page.get() != Page::Backups && !data.get_untracked().is_null() {
            return;
        }
        leptos::task::spawn_local(async move {
            let b = tauri::call::<Value>("list_backups")
                .await
                .unwrap_or(Value::Null);
            keep.set(b.s("keep"));
            data.set(b);
        });
    });
    let free = move || {
        let s = app.status.get();
        s.server.is_some() && s.busy.is_none() && s.job.is_none()
    };
    let pick = move |_| {
        leptos::task::spawn_local(async move {
            let start = data.get_untracked().s("dir");
            let start = (!start.is_empty()).then_some(start);
            if let Ok(Some(path)) = tauri::invoke::<Option<String>>(
                "pick_archive",
                json!({ "title": crate::sprache::t!("bak-waehlen"), "start": start }),
            )
            .await
            {
                let f = file_name(&path);
                restore(app, path, f);
            }
        });
    };
    let set_keep = move |e: leptos::ev::Event| {
        let k = event_target_value(&e).parse::<u32>().unwrap_or(5).max(1);
        leptos::task::spawn_local(async move {
            if let Err(er) = tauri::invoke::<()>("set_backup_keep", json!({ "keep": k })).await {
                error(crate::sprache::t!("gem-speichern"), er).await;
            }
            app.backups_tick.update(|n| *n += 1);
        });
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Backups id="tab-backups">
            <p class="dim">{move || {
                let b = data.get();
                if b.is_null() { String::new() } else {
                    crate::sprache::t!("bak-erklaerung", ordner = b.s("dir"), anzahl = b.s("keep"))
                }
            }}</p>
            <div class="row">
                <button class="primary" disabled=move || !free() on:click=move |_| crate::pages::overview::backup_now(app)>{crate::sprache::t!("bak-jetzt")}</button>
                <button on:click=|_| open_folder("backups")>{crate::sprache::t!("bak-ordner")}</button>
                <button on:click=pick>{crate::sprache::t!("bak-andere")}</button>
                <span class="spacer"></span>
                <label>{crate::sprache::t!("bak-aufheben")}</label>
                <input type="number" min="1" max="100" class="num" prop:value=keep on:change=set_keep/>
            </div>
            <div class="list">
                {move || {
                    let items = data.get().a("items");
                    if items.is_empty() {
                        view! { <div class="empty">{crate::sprache::t!("bak-leer")}</div> }.into_any()
                    } else {
                        items.into_iter().map(|it| {
                            let (path, file) = (it.s("path"), it.s("file"));
                            view! {
                                <div class="item">
                                    <div class="grow"><div>{it.s("label")}</div>
                                        <div class="sub">{format!("{} · {}", it.s("file"), fmt_bytes(it.f("size")))}</div></div>
                                    <button on:click=move |_| restore(app, path.clone(), file.clone())>{crate::sprache::t!("bak-wiederherstellen")}</button>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </section>
    }
}
