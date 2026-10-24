//! Server hinzufügen (Ordner, ZIP/Backup, neu erstellen) und aus der Liste entfernen.

use leptos::prelude::*;
use serde_json::{Value, json};
use std::rc::Rc;

use crate::dialog::{self, Btn, confirm, error, info};
use crate::pages::overview::ask_eula;
use crate::state::{AppState, use_app};
use crate::tasks::task;
use crate::tauri;
use crate::util::{J, file_name};

const KINDS: [(&str, &str); 4] = [
    ("fabric", "Fabric (Mods)"),
    ("paper", "Paper (Plugins)"),
    ("neoforge", "NeoForge (Mods)"),
    ("vanilla", "Vanilla (ohne Mods)"),
];

#[derive(Clone, Copy)]
struct Form {
    name: RwSignal<String>,
    kind: RwSignal<String>,
    versions: RwSignal<Result<Vec<String>, String>>,
    version: RwSignal<String>,
    dir: RwSignal<String>,
    custom_dir: RwSignal<bool>,
    dir_hint: RwSignal<String>,
    eula: RwSignal<bool>,
}

struct Details {
    name: String,
    kind: String,
    version: String,
    target: String,
    eula: bool,
}

fn suggest(f: Form) {
    if f.custom_dir.get_untracked() {
        return;
    }
    let name = f.name.get_untracked().trim().to_string();
    leptos::task::spawn_local(async move {
        let n = if name.is_empty() {
            "Server".to_string()
        } else {
            name
        };
        f.dir.set(
            tauri::invoke::<String>("suggest_dir", json!({ "name": n }))
                .await
                .unwrap_or_default(),
        );
    });
}

fn load_versions(f: Form) {
    f.versions.set(Ok(vec![]));
    f.version.set(String::new());
    let kind = f.kind.get_untracked();
    leptos::task::spawn_local(async move {
        match tauri::invoke::<Vec<String>>("server_versions", json!({ "kind": kind })).await {
            Ok(v) => {
                let v: Vec<String> = v.into_iter().take(40).collect();
                f.version.set(v.first().cloned().unwrap_or_default());
                f.versions.set(Ok(v));
            }
            Err(e) => f.versions.set(Err(e)),
        }
    });
}

/// Dialog: Name, evtl. Server-Typ und Version, evtl. Ordner, evtl. EULA-Häkchen.
async fn ask_details(
    title: &str,
    name: &str,
    need_eula: bool,
    kinds: bool,
    folder: bool,
) -> Option<Details> {
    let f = Form {
        name: RwSignal::new(name.to_string()),
        kind: RwSignal::new("fabric".into()),
        versions: RwSignal::new(Ok(vec![])),
        version: RwSignal::new(String::new()),
        dir: RwSignal::new(String::new()),
        custom_dir: RwSignal::new(false),
        dir_hint: RwSignal::new(String::new()),
        eula: RwSignal::new(false),
    };
    if kinds {
        load_versions(f);
    }
    if folder {
        suggest(f);
    }
    let ok = Signal::derive(move || {
        !f.name.get().trim().is_empty()
            && (!need_eula || f.eula.get())
            && (!kinds || !f.version.get().is_empty())
            && (!folder || !f.dir.get().is_empty())
    });
    let pick_dir = move |_| {
        leptos::task::spawn_local(async move {
            let Ok(Some(picked)) = tauri::invoke::<Option<String>>(
                "pick_folder",
                json!({ "title": "Ordner für den Server wählen" }),
            )
            .await
            else {
                return;
            };
            let name = f.name.get_untracked().trim().to_string();
            let c = tauri::invoke::<Value>("check_dir", json!({ "path": picked, "name": if name.is_empty() { "Server".into() } else { name } })).await.unwrap_or(Value::Null);
            match c.s("state").as_str() {
                "nonempty" => {
                    f.dir.set(c.s("suggestion"));
                    f.dir_hint.set("Der gewählte Ordner ist nicht leer – der Server kommt in einen neuen Unterordner.".into());
                }
                "invalid" => {
                    f.dir_hint
                        .set("Dieser Ordner lässt sich nicht verwenden.".into());
                    return;
                }
                _ => {
                    f.dir.set(picked);
                    f.dir_hint.set(String::new());
                }
            }
            f.custom_dir.set(true);
        });
    };
    let body: dialog::BodyFn = Rc::new(move || {
        view! {
        <label>"Name in der Server-Liste"</label>
        <input prop:value=f.name on:input=move |e| { f.name.set(event_target_value(&e)); suggest(f); }
            on:keydown=move |e| if e.key() == "Enter" && ok.get_untracked() { dialog::close("ok") }/>
        {kinds.then(|| view! {
            <label>"Server-Typ"</label>
            <select prop:value=f.kind on:change=move |e| { f.kind.set(event_target_value(&e)); load_versions(f); }>
                {KINDS.iter().map(|(k, l)| view! { <option value=*k>{*l}</option> }).collect_view()}
            </select>
            <label>"Minecraft-Version"</label>
            <select prop:value=f.version on:change=move |e| f.version.set(event_target_value(&e))>
                {move || match f.versions.get() {
                    Ok(v) if v.is_empty() => view! { <option value="">"lädt …"</option> }.into_any(),
                    Ok(v) => v.into_iter().map(|x| view! { <option value=x.clone()>{x.clone()}</option> }).collect_view().into_any(),
                    Err(e) => view! { <option value="">{format!("Keine Verbindung: {e}")}</option> }.into_any(),
                }}
            </select>
        })}
        {folder.then(|| view! {
            <label>"Ordner"</label>
            <div class="row" style="margin:0"><code class="grow" style="word-break:break-all">{move || f.dir.get()}</code>
                <button type="button" on:click=pick_dir>"Anderer Ordner …"</button></div>
            <div class="dim small">{move || f.dir_hint.get()}</div>
        })}
        {need_eula.then(|| view! {
            <label class="check"><input type="checkbox" on:change=move |e| f.eula.set(event_target_checked(&e))/>
                " Ich akzeptiere die "
                <a href="#" on:click=|e| { e.prevent_default(); leptos::task::spawn_local(async { let _ = tauri::call::<()>("open_eula").await; }); }>"Minecraft-EULA"</a>
                " (ohne startet kein Server)"</label>
        })}
    }.into_any()
    });
    if dialog::ask_view(
        title,
        body,
        vec![
            Btn::new("Abbrechen", ""),
            Btn::primary("Weiter", "ok").when(ok),
        ],
        false,
    )
    .await
        != "ok"
    {
        return None;
    }
    Some(Details {
        name: f.name.get_untracked().trim().to_string(),
        kind: f.kind.get_untracked(),
        version: f.version.get_untracked(),
        target: f.dir.get_untracked(),
        eula: f.eula.get_untracked(),
    })
}

async fn after_added(app: AppState, r: Value, name: &str) {
    let notes: Vec<String> = r
        .a("notes")
        .iter()
        .filter_map(|n| n.as_str().map(String::from))
        .collect();
    if !notes.is_empty() {
        info("Hinweis", notes.join("\n")).await;
    }
    app.add_event(format!("Server hinzugefügt: {name}"));
    app.refresh_status();
    if !r.b("activated") {
        info(
            "Server hinzugefügt",
            format!("„{name}“ ist in der Liste. Stoppe den laufenden Server, um umzuschalten."),
        )
        .await;
    }
}

async fn add(app: AppState, path: String, d_name: String, eula: bool) {
    match tauri::invoke::<Value>(
        "add_server",
        json!({ "path": path, "name": d_name, "eula": eula }),
    )
    .await
    {
        Ok(r) => after_added(app, r, &d_name).await,
        Err(e) => error("Hinzufügen fehlgeschlagen", e).await,
    }
}

pub fn add_folder() {
    let app = use_app();
    leptos::task::spawn_local(async move {
        let Ok(Some(path)) = tauri::invoke::<Option<String>>(
            "pick_folder",
            json!({ "title": "Server-Ordner wählen" }),
        )
        .await
        else {
            return;
        };
        let insp = match tauri::invoke::<Value>("inspect_folder", json!({ "path": path })).await {
            Ok(i) => i,
            Err(e) => return error("Kein Server gefunden", e).await,
        };
        let Some(d) = ask_details(
            "Server hinzufügen",
            &insp.s("name"),
            !insp.b("eula"),
            false,
            false,
        )
        .await
        else {
            return;
        };
        add(app, insp.s("root"), d.name, d.eula).await;
    });
}

/// Namensvorschlag aus dem Archivnamen (ohne Endung und Datum).
fn base_name(archive: &str) -> String {
    let mut b = file_name(archive);
    for ext in [".tar.gz", ".tar.zst", ".tgz", ".zip", ".gz", ".zst", ".tar"] {
        if b.to_lowercase().ends_with(ext) {
            b.truncate(b.len() - ext.len());
            break;
        }
    }
    // „_2026-10-21_05-00“ bzw. „…-00“ am Ende weg
    let bytes: Vec<char> = b.chars().collect();
    for len in [20usize, 17] {
        if bytes.len() > len {
            let tail: String = bytes[bytes.len() - len..].iter().collect();
            let pat = if len == 20 {
                "_dddd-dd-dd_dd-dd-dd"
            } else {
                "_dddd-dd-dd_dd-dd"
            };
            if tail
                .chars()
                .zip(pat.chars())
                .all(|(c, p)| if p == 'd' { c.is_ascii_digit() } else { c == p })
            {
                return bytes[..bytes.len() - len].iter().collect();
            }
        }
    }
    b
}

pub fn add_archive() {
    let app = use_app();
    leptos::task::spawn_local(async move {
        let Ok(Some(archive)) = tauri::invoke::<Option<String>>(
            "pick_archive",
            json!({ "title": "Server-Backup (ZIP oder tar) wählen", "start": null }),
        )
        .await
        else {
            return;
        };
        let Some(d) = ask_details(
            "Server importieren",
            &base_name(&archive),
            false,
            false,
            true,
        )
        .await
        else {
            return;
        };
        let r = match task(
            "import_server",
            json!({ "archive": archive, "name": d.name, "target": d.target }),
        )
        .await
        {
            Ok(r) => r,
            Err(e) => return error("Import fehlgeschlagen", e).await,
        };
        let eula = r.b("eula") || ask_eula(&d.name).await;
        add(app, r.s("path"), d.name, eula).await;
    });
}

pub fn add_new() {
    let app = use_app();
    leptos::task::spawn_local(async move {
        let Some(d) = ask_details("Neuen Server erstellen", "Mein Server", true, true, true).await
        else {
            return;
        };
        let r = match task(
            "create_server",
            json!({ "kind": d.kind, "name": d.name, "version": d.version, "target": d.target }),
        )
        .await
        {
            Ok(r) => r,
            Err(e) => return error("Erstellen fehlgeschlagen", e).await,
        };
        add(app, r.s("path"), d.name, true).await;
    });
}

pub fn remove_server() {
    let app = use_app();
    let Some(s) = app
        .status
        .get_untracked()
        .servers
        .into_iter()
        .find(|s| s.active)
    else {
        return;
    };
    leptos::task::spawn_local(async move {
        if !confirm(
            format!("„{}“ aus der Liste entfernen?", s.name),
            format!(
                "Die Dateien in {} bleiben erhalten, nur der Eintrag verschwindet.",
                s.path
            ),
            "Entfernen",
        )
        .await
        {
            return;
        }
        if let Err(e) = tauri::invoke::<()>("remove_server", json!({ "path": s.path })).await {
            return error("Entfernen fehlgeschlagen", e).await;
        }
        app.refresh_status();
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn name_aus_archiv() {
        assert_eq!(
            super::base_name("/x/MC_Server_2026-10-21_05-00.zip"),
            "MC_Server"
        );
        assert_eq!(
            super::base_name("C:\\a\\Welt_2026-10-21_05-00-12.tar.gz"),
            "Welt"
        );
        assert_eq!(super::base_name("Survival.zip"), "Survival");
    }
}
