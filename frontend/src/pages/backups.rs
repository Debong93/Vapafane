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
        if !confirm(format!("„{name}“ auf {file} zurücksetzen?"),
            "Der Server wird dafür gestoppt. Der jetzige Stand wird nicht gelöscht, sondern bleibt als Ordner „…_vor_Wiederherstellung_<Datum>“ daneben liegen.",
            "Wiederherstellen").await
        {
            return;
        }
        match task("restore_backup", json!({ "path": path })).await {
            Ok(r) => {
                app.add_event("Backup wiederhergestellt");
                let aside = r.s("aside");
                info("Wiederhergestellt", format!("Der Server ist auf dem Stand von {}. Du kannst ihn jetzt starten.{}", r.s("label"),
                    if aside.is_empty() { String::new() } else { format!("\n\nDer vorherige Stand liegt in:\n{aside}\n(kann gelöscht werden, wenn alles passt)") })).await;
            }
            Err(e) => error("Wiederherstellen fehlgeschlagen", e).await,
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
                json!({ "title": "Backup wählen", "start": start }),
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
                error("Speichern", er).await;
            }
            app.backups_tick.update(|n| *n += 1);
        });
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Backups id="tab-backups">
            <p class="dim">{move || {
                let b = data.get();
                if b.is_null() { String::new() } else {
                    format!("Jedes Backup ist der komplette Server-Ordner als ZIP, mit Datum im Namen. Gespeichert in {}. Die neuesten {} bleiben, ältere werden gelöscht.", b.s("dir"), b.s("keep"))
                }
            }}</p>
            <div class="row">
                <button class="primary" disabled=move || !free() on:click=move |_| crate::pages::overview::backup_now(app)>"Jetzt sichern"</button>
                <button on:click=|_| open_folder("backups")>"Backup-Ordner öffnen"</button>
                <button on:click=pick>"Andere ZIP wiederherstellen …"</button>
                <span class="spacer"></span>
                <label>"Aufheben:"</label>
                <input type="number" min="1" max="100" class="num" prop:value=keep on:change=set_keep/>
            </div>
            <div class="list">
                {move || {
                    let items = data.get().a("items");
                    if items.is_empty() {
                        view! { <div class="empty">"Noch keine Backups für diesen Server."</div> }.into_any()
                    } else {
                        items.into_iter().map(|it| {
                            let (path, file) = (it.s("path"), it.s("file"));
                            view! {
                                <div class="item">
                                    <div class="grow"><div>{it.s("label")}</div>
                                        <div class="sub">{format!("{} · {}", it.s("file"), fmt_bytes(it.f("size")))}</div></div>
                                    <button on:click=move |_| restore(app, path.clone(), file.clone())>"Wiederherstellen"</button>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </section>
    }
}
