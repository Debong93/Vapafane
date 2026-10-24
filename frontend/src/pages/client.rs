//! Client-Bereich: Einladung einlösen, Minecraft einrichten, eingerichtete Server (mit Wächter).

use leptos::prelude::*;
use serde_json::{Value, json};
use std::time::Duration;

use crate::dialog::{Btn, ask, confirm, error, info, show_error};
use crate::state::{AppState, Mode, Page, document_visible, use_app};
use crate::tasks::task;
use crate::tauri;
use crate::util::{J, fmt_bytes, open_link};

fn loader_name(l: &str) -> String {
    match l {
        "fabric" => "Fabric",
        "neoforge" => "NeoForge",
        "paper" => "Paper",
        "vanilla" => "Vanilla",
        other => return other.to_string(),
    }
    .into()
}

use crate::sprache::t;

fn ping_pill(p: &Value, ok: bool) -> AnyView {
    if !p.b("online") {
        return view! { <span class="pill bad">{t!("cli-nicht-erreichbar")}</span> }.into_any();
    }
    if ok {
        view! { <span class="pill ok">{t!("cli-online", version = p.s("version"), spieler = p.s("players"), max = p.s("max"))}</span> }.into_any()
    } else {
        view! { <span class="pill warn">{t!("cli-version-passt-nicht", version = p.s("version"))}</span> }.into_any()
    }
}

fn client_pill(s: &Value) -> AnyView {
    if s.b("online") {
        if s.b("version_ok") {
            ping_pill(&s.o("ping"), true)
        } else {
            view! { <span class="pill warn">{t!("cli-aktualisiert", version = s.s("ping/version"))}</span> }.into_any()
        }
    } else {
        let hint = s.s("hint");
        view! { <span class="pill bad">{t!("cli-gerade-nicht")}</span>{(!hint.is_empty()).then(|| view! { <div class="dim small">{hint}</div> })} }.into_any()
    }
}

/// Launcher bzw. Modrinth-App öffnen; wenn nicht vorhanden, zur Download-Seite anbieten.
fn play(target: String) {
    leptos::task::spawn_local(async move {
        let t = if target.is_empty() {
            "launcher".to_string()
        } else {
            target
        };
        if let Err(e) = tauri::invoke::<()>("client_play", json!({ "target": t })).await {
            let modrinth = t == "modrinth";
            let r = ask(
                if modrinth {
                    t!("cli-modrinth-fehlt")
                } else {
                    t!("cli-launcher-fehlt")
                },
                e,
                vec![
                    Btn::new(t!("gem-schliessen"), ""),
                    Btn::primary(t!("cli-download-seite"), "ok"),
                ],
            )
            .await;
            if r == "ok" {
                open_link(if modrinth {
                    "modrinth-download"
                } else {
                    "launcher-download"
                });
            }
        }
    });
}

#[derive(Clone, Copy)]
struct Client {
    code: RwSignal<String>,
    info: RwSignal<Value>,
    list: RwSignal<Vec<Value>>,
    status: RwSignal<Vec<Value>>,
    source: RwSignal<Option<String>>,
    preview: RwSignal<Option<Result<Value, String>>>,
    checking: RwSignal<bool>,
    selected: RwSignal<Vec<String>>,
    dest: RwSignal<String>,
    dir: RwSignal<Option<String>>,
    installing: RwSignal<bool>,
    result: RwSignal<Option<Result<Value, String>>>,
    watching: RwSignal<bool>,
    /// „Einstellungen übernehmen“: mögliche Quellen, gewählte Quelle („“ = nichts) und Teile
    sources: RwSignal<Vec<Value>>,
    take_dir: RwSignal<String>,
    take_parts: RwSignal<Vec<String>>,
    /// Modrinth-App: vorhandene Instanz aktualisieren statt neu anlegen
    update: RwSignal<bool>,
    /// Hinweis über der Vorschau (z. B. „gespeicherte Einladung geladen“)
    notice: RwSignal<Option<String>>,
}

/// Was übernommen werden kann (wie im Backend, transfer::PARTS); Beschriftung als Sprach-Schlüssel.
const PARTS: [(&str, &str); 6] = [
    ("options", "cli-teil-options"),
    ("config", "cli-teil-config"),
    ("shaders", "cli-teil-shaders"),
    ("resourcepacks", "cli-teil-resourcepacks"),
    ("servers", "cli-teil-servers"),
    ("worlds", "cli-teil-worlds"),
];

fn refresh(c: Client) {
    leptos::task::spawn_local(async move {
        c.info.set(
            tauri::call::<Value>("client_info")
                .await
                .unwrap_or(Value::Null),
        );
        let list = tauri::call::<Vec<Value>>("client_list")
            .await
            .unwrap_or_default();
        let any = !list.is_empty();
        c.list.set(list);
        if any {
            watch(c);
        }
    });
}

/// Alle eingerichteten Server prüfen (und eine gerade angezeigte Einladung).
fn watch(c: Client) {
    if c.watching.get_untracked() {
        return;
    }
    c.watching.set(true);
    leptos::task::spawn_local(async move {
        if let Ok(s) = tauri::call::<Vec<Value>>("client_watch").await {
            c.status.set(s);
        }
        if let (Some(src), Some(Ok(_))) = (c.source.get_untracked(), c.preview.get_untracked())
            && let Ok(p) =
                tauri::invoke::<Value>("client_preview_ping", json!({ "source": src })).await
        {
            c.preview.update(|pr| {
                if let Some(Ok(v)) = pr {
                    v["ping"] = p.o("ping");
                    v["version_ok"] = json!(p.b("version_ok"));
                }
            });
        }
        c.watching.set(false);
    });
}

fn check(c: Client, source: String) {
    c.source.set(Some(source.clone()));
    c.checking.set(true);
    c.result.set(None);
    c.dir.set(None);
    c.selected.set(vec![]);
    c.take_dir.set(String::new());
    c.take_parts.set(vec![]);
    leptos::task::spawn_local(async move {
        let r = tauri::invoke::<Value>("client_preview", json!({ "source": source })).await;
        let target = c.info.get_untracked().s("target");
        c.dest.set(if target.is_empty() {
            "launcher".into()
        } else {
            target
        });
        let exclude = r.as_ref().map(|p| p.s("default_dir")).unwrap_or_default();
        c.update.set(r.as_ref().is_ok_and(|p| !p.a("modrinth_existing").is_empty()));
        c.preview.set(Some(r));
        c.checking.set(false);
        let sources = tauri::invoke::<Vec<Value>>("client_sources", json!({ "exclude": exclude }))
            .await
            .unwrap_or_default();
        c.sources.set(sources);
    });
}

/// Quelle wählen: alles vorhandene außer Welten ist vorausgewählt.
fn select_source(c: Client, dir: String) {
    let parts = c
        .sources
        .get_untracked()
        .into_iter()
        .find(|s| s.s("dir") == dir)
        .and_then(|s| s.o("parts").as_object().map(|o| o.keys().filter(|k| *k != "worlds").cloned().collect()))
        .unwrap_or_default();
    c.take_parts.set(parts);
    c.take_dir.set(dir);
}

/// Wird gerade eine vorhandene Instanz der Modrinth-App aktualisiert? (Dann nichts übernehmen.)
fn updating(c: Client) -> bool {
    c.dest.get() == "modrinth"
        && c.update.get()
        && c.preview.with(|p| matches!(p, Some(Ok(v)) if !v.a("modrinth_existing").is_empty()))
}

/// „Einstellungen übernehmen von …“
fn take_view(c: Client) -> AnyView {
    let pick = move |_| {
        leptos::task::spawn_local(async move {
            let Ok(Some(d)) = tauri::invoke::<Option<String>>(
                "pick_folder",
                json!({ "title": t!("cli-quelle-waehlen") }),
            )
            .await
            else {
                return;
            };
            match tauri::invoke::<Value>("client_source", json!({ "dir": d })).await {
                Ok(src) => {
                    let dir = src.s("dir");
                    c.sources.update(|l| {
                        l.retain(|s| s.s("dir") != dir);
                        l.insert(0, src);
                    });
                    select_source(c, dir);
                }
                Err(e) => error(t!("cli-nichts-uebernehmen"), e).await,
            }
        })
    };
    let parts = move || {
        let dir = c.take_dir.get();
        let src = c.sources.get().into_iter().find(|s| s.s("dir") == dir)?;
        let avail = src.o("parts");
        Some(view! {
            <div class="take-parts">
                {PARTS.iter().filter(|(k, _)| avail.get(*k).is_some()).map(|(k, label)| {
                    let (k1, k2) = (k.to_string(), k.to_string());
                    let size = fmt_bytes(avail.f(k));
                    view! { <label class="check"><input type="checkbox" prop:checked=move || c.take_parts.get().contains(&k1)
                        on:change=move |e| {
                            let on = event_target_checked(&e);
                            let k = k2.clone();
                            c.take_parts.update(|p| { p.retain(|x| *x != k); if on { p.push(k); } });
                        }/>{format!(" {} ({size})", crate::sprache::text(label, None))}</label> }
                }).collect_view()}
            </div>
            {move || c.take_parts.get().iter().any(|p| p == "worlds").then(|| view! {
                <p class="dim small">{t!("cli-welten-warnung")}</p> })}
            {move || (c.dest.get() == "launcher").then(|| view! {
                <p class="dim small">{t!("cli-nur-fehlendes")}</p> })}
        })
    };
    view! {
        <div class="take">
            <div class="row">
                <b>{t!("cli-uebernehmen-von")}</b>
                <select id="cl-take" class="grow" prop:value=move || c.take_dir.get() on:change=move |e| select_source(c, event_target_value(&e))>
                    <option value="" selected=move || c.take_dir.get().is_empty()>{t!("cli-nichts")}</option>
                    {move || c.sources.get().into_iter().map(|s| {
                        let dir = s.s("dir");
                        let d = dir.clone();
                        view! { <option value=dir selected=move || c.take_dir.get() == d>{s.s("label")}</option> }
                    }).collect_view()}
                </select>
                <button on:click=pick>{t!("gem-anderer-ordner")}</button>
            </div>
            {parts}
            <p class="dim small">{t!("cli-uebernehmen-erklaerung")}</p>
        </div>
    }.into_any()
}

fn install(app: AppState, c: Client) {
    let Some(Ok(p)) = c.preview.get_untracked() else {
        return;
    };
    c.installing.set(true);
    leptos::task::spawn_local(async move {
        let take_dir = c.take_dir.get_untracked();
        let take = (!take_dir.is_empty() && !untrack(move || updating(c)))
            .then(|| json!({ "dir": take_dir, "parts": c.take_parts.get_untracked() }));
        let args = json!({ "source": p.s("source"), "target": c.dir.get_untracked(), "selected": c.selected.get_untracked(),
            "destination": c.dest.get_untracked(), "take": take, "update": c.update.get_untracked() });
        let r = task("client_install", args).await;
        c.installing.set(false);
        if r.is_ok() {
            app.add_event(t!("cli-ev-eingerichtet", name = p.s("invite/n")));
            c.info
                .update(|i| i["target"] = json!(c.dest.get_untracked()));
            refresh(c);
        }
        c.result.set(Some(r));
    });
}

/// „Aktualisieren“ bei einem eingerichteten Server:
/// - Server wurde auf eine andere Minecraft-Version gebracht → erklären, dass es dafür einen neuen
///   Einladungs-Code braucht (Mods und Version stehen nur in der Einladung).
/// - sonst → gespeicherte Einladung oben laden; „Einrichten“ bringt alles wieder in Ordnung
///   (fehlende Mods, verschwundenes Profil, gelöschte Instanz).
fn refresh_client(c: Client, dir: String, name: String, modrinth: bool) {
    let st = c.status.get_untracked().into_iter().find(|s| s.s("dir") == dir);
    let updated = st.as_ref().is_some_and(|s| s.b("online") && !s.b("version_ok"));
    let version = st.map(|s| s.s("ping/version")).unwrap_or_default();
    leptos::task::spawn_local(async move {
        if updated {
            info(
                t!("cli-upd-titel"),
                format!(
                    "{}\n\n{} {}",
                    t!("cli-upd-text", name = name, version = version),
                    t!("cli-upd-code"),
                    if modrinth {
                        t!("cli-upd-modrinth")
                    } else {
                        t!("cli-upd-launcher")
                    }
                ),
            )
            .await;
            focus_code();
            return;
        }
        match tauri::invoke::<String>("client_saved_code", json!({ "dir": dir })).await {
            Ok(code) => {
                c.code.set(code.clone());
                check(c, code);
                c.notice.set(Some(t!("cli-gespeicherte-einladung", name = name)));
                focus_code();
            }
            Err(e) => error(t!("cli-aktualisieren"), e).await,
        }
    });
}

/// Zum Feld für den Einladungs-Code springen.
fn focus_code() {
    use wasm_bindgen::JsCast;
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("cl-code"))
        .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    {
        el.scroll_into_view();
        let _ = el.focus();
    }
}

fn result_view(c: Client, r: Result<Value, String>) -> AnyView {
    let r = match r {
        Err(e) => return view! { <p class="hint error">{e}</p> }.into_any(),
        Ok(r) => r,
    };
    let name = c
        .preview
        .get_untracked()
        .and_then(|p| p.ok())
        .map(|p| p.s("invite/n"))
        .unwrap_or_default();
    let target = r.s("target");
    let dir = r.s("dir");
    let modrinth = target == "modrinth";
    let notes: Vec<String> = r
        .a("notes")
        .iter()
        .filter_map(|n| n.as_str().map(String::from))
        .collect();
    let manual = r.a("manual");
    let update = r.o("update");
    let taken = r.o("taken");
    let (d1, d2, d3) = (dir.clone(), dir.clone(), dir.clone());
    let open_dir = move |d: String| {
        leptos::task::spawn_local(async move {
            if let Err(e) = tauri::invoke::<()>("client_open_folder", json!({ "dir": d })).await {
                show_error(crate::sprache::t!("gem-ordner"), e);
            }
        })
    };
    let guide = {
        let (t, d) = (target.clone(), dir.clone());
        let name = name.clone();
        let loader = c.preview.get_untracked().and_then(|p| p.ok()).map(|p| p.s("invite/l")).unwrap_or_default();
        move |_| crate::pages::extras::show_guide(t.clone(), name.clone(), d.clone(), loader.clone())
    };
    if modrinth && !update.is_null() {
        let inst = crate::sprache::t!("gem-zitat", text = update.s("name"));
        let inst2 = inst.clone();
        return view! {
            <p class="hint">"✓ "{t!("cli-paket-update-1")}" "<b>{inst}</b>" – "{t!("cli-paket-update-2")}</p>
            <ol class="steps">
                <li>{t!("cli-schritt-instanz")}" "<b>{inst2}</b>" "{t!("cli-schritt-anklicken")}</li>
                <li>{t!("cli-schritt-zahnrad")}</li>
                <li><b>"„"{t!("cli-neu-installieren")}"“"</b>" "{t!("cli-schritt-datei")}<br/><code>{r.s("mrpack")}</code></li>
            </ol>
            <div class="row">
                <button class="primary" on:click=move |_| play("modrinth".into())>{t!("cli-modrinth-oeffnen")}</button>
                <button on:click=move |_| open_dir(d3.clone())>{t!("cli-paket-ordner")}</button>
                <button on:click=guide>{t!("ext-knopf-so-gehts")}</button>
            </div>
            <p class="dim small">{t!("cli-update-sicher")}</p>
        }.into_any();
    }
    let taken_note = (!taken.is_null()).then(|| {
        let from = taken.s("from");
        let text = if modrinth {
            t!("cli-uebernommen-paket", quelle = from)
        } else {
            let kept = taken.f("kept").unwrap_or(0.0);
            format!(
                "{}{}",
                t!("cli-uebernommen", quelle = from, anzahl = taken.f("copied").unwrap_or(0.0)),
                if kept > 0.0 { format!(" {}", t!("cli-uebernommen-da", anzahl = kept)) } else { String::new() }
            )
        };
        let failed: Vec<String> = taken.a("failed").iter().filter_map(|f| f.as_str().map(String::from)).collect();
        let failed_note = (!failed.is_empty()).then(|| {
            let mut list = failed.iter().take(3).cloned().collect::<Vec<_>>().join(", ");
            if failed.len() > 3 {
                list.push_str(" …");
            }
            format!(" {}", t!("cli-uebernommen-fehler", anzahl = failed.len() as u32, dateien = list))
        });
        view! { <p class="hint">{text}{failed_note}</p> }
    });
    view! {
        {if modrinth {
            if r.b("opened") {
                view! { <p class="hint">"✓ "{t!("cli-paket-offen")}" "<b>{format!("„{name} (Vapafane)“")}</b>"."</p> }.into_any()
            } else {
                view! {
                    <p class="hint">"✓ "{t!("cli-paket-fertig")}" "<code>{r.s("mrpack")}</code><br/>
                        {t!("cli-paket-von-hand")}</p>
                    <button on:click=move |_| open_dir(d1.clone())>{t!("cli-paket-ordner")}</button>
                }.into_any()
            }
        } else {
            view! { <p class="hint">{format!("✓ {}", t!("cli-eingerichtet", anzahl = r.a("installed").len() as u32))}
                {r.b("profile").then(|| view! { " "{t!("cli-profil", name = format!("Vapafane – {name}"))} })}</p> }.into_any()
        }}
        {taken_note}
        {notes.into_iter().map(|n| view! { <p class="hint">{n}</p> }).collect_view()}
        {(!manual.is_empty()).then(|| view! {
            <p><b>{t!("cli-selbst-laden")}</b></p>
            <ul>{manual.iter().map(|m| { let page = m.s("page"); view! {
                <li>{format!("{} – {} ", m.s("title"), m.s("reason"))}
                    {(!page.is_empty()).then(|| view! { <button on:click=move |_| open_link(page.clone())>{t!("kat-browser")}</button> })}</li>
            } }).collect_view()}</ul>
            <button on:click=move |_| open_dir(d2.clone())>{t!("gem-ordner-oeffnen")}</button>
        })}
        <div class="row"><button class="primary" on:click=move |_| play(target.clone())>
            {if modrinth { t!("cli-spielen-modrinth") } else { t!("cli-spielen-launcher") }}</button>
            <button on:click=guide>{t!("ext-knopf-so-gehts")}</button></div>
    }.into_any()
}

fn preview_view(app: AppState, c: Client, p: Value) -> AnyView {
    let inv = p.o("invite");
    let loader = format!(
        "{}{}",
        loader_name(&inv.s("l")),
        if inv.s("lv").is_empty() {
            String::new()
        } else {
            format!(" {}", inv.s("lv"))
        }
    );
    let mods = p.a("mods");
    let hint = p.s("hint");
    let ping = p.o("ping");
    let version_ok = p.b("version_ok");
    let info = c.info.get_untracked();
    let found = |b: bool| {
        if b {
            view! { <span class="pill ok">{t!("cli-gefunden")}</span> }.into_any()
        } else {
            view! { <span class="pill warn">{t!("cli-nicht-gefunden")}</span> }.into_any()
        }
    };
    let rows = if mods.is_empty() {
        view! { <tr><td colspan="4" class="dim">{t!("cli-keine-mods", version = inv.s("m"))}</td></tr> }.into_any()
    } else {
        mods.iter().map(|m| {
            let key = m.s("key");
            view! {
                <tr>
                    <td>{if m.b("optional") { let k0 = key.clone(); view! { <input type="checkbox" class="opt-mod"
                        prop:checked=move || c.selected.get().contains(&k0) on:change=move |e| {
                        let on = event_target_checked(&e);
                        let k = key.clone();
                        c.selected.update(|s| { s.retain(|x| *x != k); if on { s.push(k); } });
                    }/> }.into_any() } else { "✓".into_any() }}</td>
                    <td>{m.s("title")}
                        {m.b("optional").then(|| view! { " "<span class="badge">{t!("cli-optional")}</span> })}
                        {(m.s("source") == "curseforge").then(|| view! { " "<span class="badge src-cf">"CurseForge"</span> })}
                        {m.b("manual").then(|| view! { <br/><span class="dim small">{t!("cli-nur-browser")}</span> })}
                        {m.b("missing_in_file").then(|| view! { <br/><span class="dim small">{t!("cli-nur-datei")}</span> })}</td>
                    <td>{m.s("version")}</td>
                    <td class="size">{m.f("size").filter(|s| *s > 0.0).map(|s| fmt_bytes(Some(s))).unwrap_or_else(|| "–".into())}</td>
                </tr>
            }
        }).collect_view().into_any()
    };
    // Hinweise zum Minecraft-Launcher – nur, wenn er auch das Ziel ist
    let (has_launcher, has_profiles) = (p.b("launcher"), p.b("profiles"));
    let warn = move || {
        if c.dest.get() != "launcher" {
            None
        } else if !has_launcher {
            Some(view! { <p class="hint">{t!("cli-launcher-fehlt-text")}" "
                <a href="#" on:click=|e| { e.prevent_default(); open_link("launcher-download"); }>{t!("cli-launcher-laden")}</a></p> }.into_any())
        } else if !has_profiles {
            Some(view! { <p class="hint">{t!("cli-nie-gestartet")}</p> }.into_any())
        } else {
            None
        }
    };
    let default_dir = p.s("default_dir");
    let pick = move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(Some(d)) = tauri::invoke::<Option<String>>(
                "pick_folder",
                json!({ "title": t!("cli-ordner-waehlen") }),
            )
            .await
            {
                c.dir.set(Some(d));
            }
        })
    };
    let ping_version = ping.s("version");
    let existing: Vec<String> = p
        .a("modrinth_existing")
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    // so heißt die Instanz in der Modrinth-App (Name aus dem Paket)
    let first = format!("{} (Vapafane)", inv.s("n"));
    view! {
        <div class="preview-head"><span class="big">{inv.s("n")}</span>" "<code>{inv.s("a")}</code>" "
            {move || c.preview.get().and_then(|r| r.ok()).map(|p| ping_pill(&p.o("ping"), p.b("version_ok")))}</div>
        <p>"Minecraft "<b>{inv.s("m")}</b>{format!(" · {loader} · {}", t!("cli-mods-zusammen", anzahl = mods.len() as u32, groesse = fmt_bytes(p.f("total"))))}</p>
        {(!hint.is_empty()).then(|| view! { <p class="hint">{hint}</p> })}
        {(ping.b("online") && !version_ok).then(|| view! { <p class="hint">{t!("cli-andere-version", version = ping_version.clone())}</p> })}
        {warn}
        <div class="table-wrap"><table class="modtable"><thead><tr><th></th><th>"Mod"</th><th>{t!("cli-spalte-version")}</th><th>{t!("cli-spalte-groesse")}</th></tr></thead><tbody>{rows}</tbody></table></div>
        <div class="dest">
            <b>{t!("cli-einrichten-fuer")}</b>
            <label class="check"><input type="radio" name="cl-dest" prop:checked=move || c.dest.get() == "launcher" on:change=move |_| c.dest.set("launcher".into())/>
                " "{t!("gem-launcher")}" "{found(info.b("launcher"))}</label>
            <label class="check"><input type="radio" name="cl-dest" prop:checked=move || c.dest.get() == "modrinth" on:change=move |_| c.dest.set("modrinth".into())/>
                " "{t!("gem-modrinth-app")}" "{found(info.b("modrinth"))}</label>
        </div>
        {(!existing.is_empty()).then(|| view! {
            <Show when=move || c.dest.get() == "modrinth">
                <div class="dest">
                    <b>{t!("cli-in-modrinth")}</b>
                    <label class="check"><input type="radio" name="cl-mr" prop:checked=move || c.update.get() on:change=move |_| c.update.set(true)/>
                        <span>" "{t!("cli-instanz-update", name = first.clone())}<br/>
                        <span class="dim small">{t!("cli-instanz-update-sub")}</span></span></label>
                    <label class="check"><input type="radio" name="cl-mr" prop:checked=move || !c.update.get() on:change=move |_| c.update.set(false)/>
                        <span>" "{t!("cli-instanz-neu")}<br/>
                        <span class="dim small">{t!("cli-instanz-neu-sub")}</span></span></label>
                </div>
            </Show>
        })}
        <Show when=move || c.dest.get() == "launcher" fallback=move || view! {
            <p class="dim small">{move || if updating(c) {
                t!("cli-update-bleibt")
            } else {
                t!("cli-import-bestaetigen")
            }}</p>
        }>
            <div class="row"><span>{t!("cli-ordner-label")}</span><code class="grow">{let d = default_dir.clone(); move || c.dir.get().unwrap_or_else(|| d.clone())}</code>
                <button on:click=pick>{t!("gem-anderer-ordner")}</button></div>
            <p class="dim small">{t!("cli-launcher-schliessen")}</p>
        </Show>
        <Show when=move || !updating(c)>{take_view(c)}</Show>
        <div class="row"><button class="primary" disabled=move || c.installing.get() on:click=move |_| install(app, c)>{t!("cli-einrichten")}</button></div>
        <div>{move || c.result.get().map(|r| result_view(c, r))}</div>
    }.into_any()
}

#[component]
pub fn ClientPage() -> impl IntoView {
    let app = use_app();
    let c = Client {
        code: RwSignal::new(String::new()),
        info: RwSignal::new(Value::Null),
        list: RwSignal::new(vec![]),
        status: RwSignal::new(vec![]),
        source: RwSignal::new(None),
        preview: RwSignal::new(None),
        checking: RwSignal::new(false),
        selected: RwSignal::new(vec![]),
        dest: RwSignal::new("launcher".into()),
        dir: RwSignal::new(None),
        installing: RwSignal::new(false),
        result: RwSignal::new(None),
        watching: RwSignal::new(false),
        sources: RwSignal::new(vec![]),
        take_dir: RwSignal::new(String::new()),
        take_parts: RwSignal::new(vec![]),
        update: RwSignal::new(false),
        notice: RwSignal::new(None),
    };
    Effect::new(move |_| {
        if app.page.get() == Page::Client {
            refresh(c);
        }
    });
    // Still im Hintergrund: alle 20 s, nur wenn die Client-Ansicht sichtbar ist
    let handle = set_interval_with_handle(
        move || {
            if app.mode.get_untracked() == Mode::Client
                && app.page.get_untracked() == Page::Client
                && document_visible()
            {
                watch(c);
            }
        },
        Duration::from_secs(20),
    )
    .ok();
    on_cleanup(move || {
        if let Some(h) = handle {
            h.clear()
        }
    });

    let from_file = move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(Some(f)) = tauri::call::<Option<String>>("client_pick_file").await {
                check(c, f);
            }
        })
    };
    let launcher = move || {
        let i = c.info.get();
        view! {
            {if i.b("launcher") { view! { <span class="pill ok">{t!("cli-launcher-da")}</span> }.into_any() } else {
                view! { <span class="pill warn">{t!("cli-kein-launcher")}</span>" "<a href="#" on:click=|e| { e.prevent_default(); open_link("launcher-download"); }>"Download"</a> }.into_any()
            }}
            " "{i.b("modrinth").then(|| view! { <span class="pill ok">{t!("cli-modrinth-da")}</span> })}
        }
    };
    let row = move |e: Value| {
        let dir = e.s("dir");
        let target = e.s("target");
        let (d_open, d_rm, d_st, t_rm) = (dir.clone(), dir.clone(), dir.clone(), target.clone());
        let extras = {
            let (t, n, d, l) = (target.clone(), e.s("name"), dir.clone(), e.s("loader"));
            move |_| crate::pages::extras::show_guide(t.clone(), n.clone(), d.clone(), l.clone())
        };
        let refresh_btn = {
            let (d, n) = (dir.clone(), e.s("name"));
            let modrinth = target == "modrinth";
            move |_| refresh_client(c, d.clone(), n.clone(), modrinth)
        };
        let name = e.s("name");
        let remove = move |_| {
            let (dir, name, modrinth) = (d_rm.clone(), name.clone(), t_rm == "modrinth");
            leptos::task::spawn_local(async move {
                let text = if modrinth {
                    t!("cli-entfernen-modrinth")
                } else {
                    t!("cli-entfernen-launcher")
                };
                if !confirm(t!("cli-entfernen-frage", name = name), text, t!("gem-entfernen")).await {
                    return;
                }
                if let Err(er) = tauri::invoke::<()>("client_remove", json!({ "dir": dir })).await {
                    error(t!("gem-entfernen"), er).await;
                }
                refresh(c);
            });
        };
        view! {
            <div class="item">
                <div class="grow">
                    <div><b>{e.s("name")}</b>" "<code>{e.s("address")}</code>" "
                        <span class="st">{move || c.status.get().into_iter().find(|s| s.s("dir") == d_st).map(|s| client_pill(&s))
                            .unwrap_or_else(|| view! { <span class="pill">{t!("cli-prueft")}</span> }.into_any())}</span></div>
                    <div class="sub">{if e.b("incomplete") {
                        t!("cli-unvollstaendig")
                    } else {
                        format!("Minecraft {} · {} · {} · {}", e.s("mc"), loader_name(&e.s("loader")),
                            t!("cli-mods-kurz", anzahl = e.s("mods")),
                            t!("cli-eingerichtet-am", ziel = if target == "modrinth" { t!("gem-modrinth-app") } else { t!("gem-launcher") }, datum = e.s("updated")))
                    }}</div>
                </div>
                <button class="primary" on:click=move |_| play(target.clone())>{t!("cli-spielen")}</button>
                <button title=t!("cli-aktualisieren-tipp")
                    on:click=refresh_btn>{t!("cli-aktualisieren")}</button>
                <button on:click=extras>{t!("ext-knopf")}</button>
                <button on:click=move |_| { let d = d_open.clone(); leptos::task::spawn_local(async move {
                    if let Err(er) = tauri::invoke::<()>("client_open_folder", json!({ "dir": d })).await { error(t!("gem-ordner"), er).await; }
                }); }>{t!("gem-ordner-oeffnen")}</button>
                <button class="danger" on:click=remove>{t!("gem-entfernen")}</button>
            </div>
        }
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Client>
            <div class="cf-box">
                <h2 class="client-h">{t!("cli-titel")}</h2>
                <p class="dim">{t!("cli-code-erklaerung-1")}<code>"VPF1-"</code>{t!("cli-code-erklaerung-2")}<code>".vapafane"</code>{t!("cli-code-erklaerung-3")}</p>
                <textarea id="cl-code" rows="3" placeholder=t!("cli-code-platzhalter") spellcheck="false"
                    prop:value=c.code on:input=move |e| c.code.set(event_target_value(&e))></textarea>
                <div class="row">
                    <button class="primary" on:click=move |_| { let code = c.code.get_untracked().trim().to_string(); if !code.is_empty() { c.notice.set(None); check(c, code) } }>{t!("cli-pruefen")}</button>
                    <button on:click=from_file>{t!("cli-datei-oeffnen")}</button>
                    <span class="spacer"></span>
                    <span class="small">{launcher}</span>
                </div>
                {move || c.notice.get().map(|n| view! { <p class="hint" id="cl-notice">{n}</p> })}
            </div>
            {move || {
                if c.checking.get() {
                    return Some(view! { <div class="cf-box"><p class="dim">{t!("cli-pruefe-einladung")}</p></div> }.into_any());
                }
                // nur bei neuer Einladung neu aufbauen (nicht bei jedem Ping)
                c.source.track();
                c.preview.with_untracked(|p| p.clone()).map(|r| match r {
                    Err(e) => view! { <div class="cf-box"><p class="hint error">{e}</p></div> }.into_any(),
                    Ok(p) => view! { <div class="cf-box">{preview_view(app, c, p)}</div> }.into_any(),
                })
            }}
            <h3>{t!("cli-meine-server")}</h3>
            <div class="list">
                {move || {
                    let list = c.list.get();
                    if list.is_empty() {
                        view! { <div class="empty">{t!("cli-leer")}</div> }.into_any()
                    } else {
                        list.into_iter().map(row).collect_view().into_any()
                    }
                }}
            </div>
            <p class="notice small">{t!("cli-kein-konto")}</p>
        </section>
    }
}
