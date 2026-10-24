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

fn ping_pill(p: &Value, ok: bool) -> AnyView {
    if !p.b("online") {
        return view! { <span class="pill bad">"Server nicht erreichbar"</span> }.into_any();
    }
    if ok {
        view! { <span class="pill ok">{format!("Online · {} · {}/{} Spieler", p.s("version"), p.s("players"), p.s("max"))}</span> }.into_any()
    } else {
        view! { <span class="pill warn">{format!("Online, aber Version {} – passt nicht", p.s("version"))}</span> }.into_any()
    }
}

fn client_pill(s: &Value) -> AnyView {
    if s.b("online") {
        if s.b("version_ok") {
            ping_pill(&s.o("ping"), true)
        } else {
            view! { <span class="pill warn">{format!("Server wurde aktualisiert ({}) – neuen Einladungs-Code beim Admin holen", s.s("ping/version"))}</span> }.into_any()
        }
    } else {
        let hint = s.s("hint");
        view! { <span class="pill bad">"gerade nicht erreichbar"</span>{(!hint.is_empty()).then(|| view! { <div class="dim small">{hint}</div> })} }.into_any()
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
                    "Modrinth-App nicht gefunden"
                } else {
                    "Minecraft-Launcher nicht gefunden"
                },
                e,
                vec![
                    Btn::new("Schließen", ""),
                    Btn::primary("Zur Download-Seite", "ok"),
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
}

/// Was übernommen werden kann (wie im Backend, transfer::PARTS).
const PARTS: [(&str, &str); 6] = [
    ("options", "Optionen & Tastenbelegung"),
    ("config", "Mod-Einstellungen"),
    ("shaders", "Shader"),
    ("resourcepacks", "Ressourcenpakete"),
    ("servers", "Server-Liste"),
    ("worlds", "Welten"),
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
                json!({ "title": "Minecraft-Ordner wählen, aus dem übernommen wird" }),
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
                Err(e) => error("Nichts zu übernehmen", e).await,
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
                        }/>{format!(" {label} ({size})")}</label> }
                }).collect_view()}
            </div>
            {move || c.take_parts.get().iter().any(|p| p == "worlds").then(|| view! {
                <p class="dim small">"Welten aus einer neueren Minecraft-Version lassen sich in einer älteren nicht öffnen. Große Welten machen das Einrichten langsamer."</p> })}
            {move || (c.dest.get() == "launcher").then(|| view! {
                <p class="dim small">"Was im Ordner schon liegt, bleibt unverändert – übernommen wird nur, was fehlt."</p> })}
        })
    };
    view! {
        <div class="take">
            <div class="row">
                <b>"Einstellungen übernehmen von:"</b>
                <select id="cl-take" class="grow" prop:value=move || c.take_dir.get() on:change=move |e| select_source(c, event_target_value(&e))>
                    <option value="" selected=move || c.take_dir.get().is_empty()>"Nichts übernehmen"</option>
                    {move || c.sources.get().into_iter().map(|s| {
                        let dir = s.s("dir");
                        let d = dir.clone();
                        view! { <option value=dir selected=move || c.take_dir.get() == d>{s.s("label")}</option> }
                    }).collect_view()}
                </select>
                <button on:click=pick>"Anderer Ordner …"</button>
            </div>
            {parts}
            <p class="dim small">"Zum Beispiel aus deiner bisherigen Instanz: Optionen, Tastenbelegung, Mod-Einstellungen, Shader, Ressourcenpakete und Server-Liste. Mods kommen immer passend vom Server. Die Quelle wird nur gelesen, nicht verändert."</p>
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
            app.add_event(format!("Client eingerichtet: {}", p.s("invite/n")));
            c.info
                .update(|i| i["target"] = json!(c.dest.get_untracked()));
            refresh(c);
        }
        c.result.set(Some(r));
    });
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
                show_error("Ordner", e);
            }
        })
    };
    if modrinth && !update.is_null() {
        let inst = format!("„{}“", update.s("name"));
        let inst2 = inst.clone();
        return view! {
            <p class="hint">"✓ Das neue Modrinth-Paket ist fertig. So aktualisierst du deine Instanz "<b>{inst}</b>
                " – Welten, Optionen, Shader, Ressourcenpakete und deine Server-Liste bleiben dabei erhalten:"</p>
            <ol class="steps">
                <li>"Modrinth-App öffnen und die Instanz "<b>{inst2}</b>" anklicken."</li>
                <li>"Oben auf das Zahnrad "<b>"„Instanz-Einstellungen“"</b>" klicken und den Reiter "<b>"„Installation“"</b>" wählen."</li>
                <li><b>"„Modpack neu installieren“"</b>" klicken, bestätigen und diese Datei wählen:"<br/><code>{r.s("mrpack")}</code></li>
            </ol>
            <div class="row">
                <button class="primary" on:click=move |_| play("modrinth".into())>"Modrinth-App öffnen"</button>
                <button on:click=move |_| open_dir(d3.clone())>"Ordner mit dem Paket öffnen"</button>
            </div>
            <p class="dim small">"Die Modrinth-App sichert die Instanz vorher selbst. Mods, die du dort selbst zusätzlich hinzugefügt hast, entfernt sie dabei womöglich."</p>
        }.into_any();
    }
    let taken_note = (!taken.is_null()).then(|| {
        let from = taken.s("from");
        let text = if modrinth {
            format!("Die Einstellungen aus „{from}“ sind im Paket – die Modrinth-App übernimmt sie beim Import.")
        } else {
            let kept = taken.f("kept").unwrap_or(0.0);
            format!(
                "Aus „{from}“ übernommen: {} Dateien{}.",
                taken.f("copied").unwrap_or(0.0),
                if kept > 0.0 { format!(" ({kept} waren schon da und blieben unverändert)") } else { String::new() }
            )
        };
        let failed: Vec<String> = taken.a("failed").iter().filter_map(|f| f.as_str().map(String::from)).collect();
        let failed_note = (!failed.is_empty()).then(|| {
            let mut list = failed.iter().take(3).cloned().collect::<Vec<_>>().join(", ");
            if failed.len() > 3 {
                list.push_str(" …");
            }
            format!(
                " {} Dateien ließen sich nicht lesen und fehlen deshalb – ist Minecraft dort noch offen? ({list})",
                failed.len()
            )
        });
        view! { <p class="hint">{text}{failed_note}</p> }
    });
    view! {
        {if modrinth {
            if r.b("opened") {
                view! { <p class="hint">"✓ Das Modrinth-Paket ist fertig, und die Modrinth-App wurde damit geöffnet. Bestätige dort den Import – dann erscheint die Instanz "<b>{format!("„{name} (Vapafane)“")}</b>"."</p> }.into_any()
            } else {
                view! {
                    <p class="hint">"✓ Das Modrinth-Paket ist fertig: "<code>{r.s("mrpack")}</code><br/>
                        "Die Modrinth-App ließ sich nicht automatisch öffnen. So geht es von Hand: in der Modrinth-App auf "<b>"„+“"</b>" (Instanz erstellen) klicken, dann "<b>"„Aus Datei importieren“"</b>" (englisch „Import from file“) und diese Datei wählen."</p>
                    <button on:click=move |_| open_dir(d1.clone())>"Ordner mit dem Paket öffnen"</button>
                }.into_any()
            }
        } else {
            view! { <p class="hint">{format!("✓ Eingerichtet: {} Mods geladen.", r.a("installed").len())}
                {r.b("profile").then(|| view! { " Im Minecraft-Launcher gibt es jetzt das Profil "<b>{format!("„Vapafane – {name}“")}</b>"." })}</p> }.into_any()
        }}
        {taken_note}
        {notes.into_iter().map(|n| view! { <p class="hint">{n}</p> }).collect_view()}
        {(!manual.is_empty()).then(|| view! {
            <p><b>"Bitte selbst herunterladen"</b>" und in den Ordner "<code>"mods"</code>" legen:"</p>
            <ul>{manual.iter().map(|m| { let page = m.s("page"); view! {
                <li>{format!("{} – {} ", m.s("title"), m.s("reason"))}
                    {(!page.is_empty()).then(|| view! { <button on:click=move |_| open_link(page.clone())>"Im Browser herunterladen ↗"</button> })}</li>
            } }).collect_view()}</ul>
            <button on:click=move |_| open_dir(d2.clone())>"Ordner öffnen"</button>
        })}
        <div class="row"><button class="primary" on:click=move |_| play(target.clone())>
            {if modrinth { "Spielen (Modrinth-App öffnen)" } else { "Spielen (Launcher öffnen)" }}</button></div>
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
            view! { <span class="pill ok">"gefunden"</span> }.into_any()
        } else {
            view! { <span class="pill warn">"nicht gefunden"</span> }.into_any()
        }
    };
    let rows = if mods.is_empty() {
        view! { <tr><td colspan="4" class="dim">{format!("Keine Mods nötig – nur Minecraft {}.", inv.s("m"))}</td></tr> }.into_any()
    } else {
        mods.iter().map(|m| {
            let key = m.s("key");
            view! {
                <tr>
                    <td>{if m.b("optional") { view! { <input type="checkbox" on:change=move |e| {
                        let on = event_target_checked(&e);
                        let k = key.clone();
                        c.selected.update(|s| { s.retain(|x| *x != k); if on { s.push(k); } });
                    }/> }.into_any() } else { "✓".into_any() }}</td>
                    <td>{m.s("title")}
                        {m.b("optional").then(|| view! { " "<span class="badge">"optional"</span> })}
                        {(m.s("source") == "curseforge").then(|| view! { " "<span class="badge src-cf">"CurseForge"</span> })}
                        {m.b("manual").then(|| view! { <br/><span class="dim small">"nur im Browser ladbar (Link nach dem Einrichten)"</span> })}
                        {m.b("missing_in_file").then(|| view! { <br/><span class="dim small">"fehlt – nur in der Einladungs-Datei"</span> })}</td>
                    <td>{m.s("version")}</td>
                    <td class="size">{m.f("size").filter(|s| *s > 0.0).map(|s| fmt_bytes(Some(s))).unwrap_or_else(|| "–".into())}</td>
                </tr>
            }
        }).collect_view().into_any()
    };
    let warn = if !p.b("launcher") {
        Some(view! { <p class="hint">"Der offizielle Minecraft-Launcher wurde nicht gefunden. Du kannst trotzdem einrichten; das Profil wird angelegt, sobald der Launcher installiert ist und du hier „Aktualisieren“ drückst. "
            <a href="#" on:click=|e| { e.prevent_default(); open_link("launcher-download"); }>"Launcher herunterladen"</a></p> }.into_any())
    } else if !p.b("profiles") {
        Some(view! { <p class="hint">"Der Launcher wurde noch nie gestartet. Bitte einmal starten und wieder schließen, dann hier einrichten."</p> }.into_any())
    } else {
        None
    };
    let default_dir = p.s("default_dir");
    let pick = move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(Some(d)) = tauri::invoke::<Option<String>>(
                "pick_folder",
                json!({ "title": "Ordner für dieses Minecraft wählen" }),
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
        <p>"Minecraft "<b>{inv.s("m")}</b>{format!(" · {loader} · {} Mods, zusammen {}", mods.len(), fmt_bytes(p.f("total")))}</p>
        {(!hint.is_empty()).then(|| view! { <p class="hint">{hint}</p> })}
        {(ping.b("online") && !version_ok).then(|| view! { <p class="hint">{format!("Der Server meldet eine andere Version ({ping_version}). Wahrscheinlich wurde er aktualisiert – bitte beim Admin einen neuen Einladungs-Code holen.")}</p> })}
        {warn}
        <div class="table-wrap"><table class="modtable"><thead><tr><th></th><th>"Mod"</th><th>"Version"</th><th>"Größe"</th></tr></thead><tbody>{rows}</tbody></table></div>
        <div class="dest">
            <b>"Einrichten für:"</b>
            <label class="check"><input type="radio" name="cl-dest" prop:checked=move || c.dest.get() == "launcher" on:change=move |_| c.dest.set("launcher".into())/>
                " Offizieller Minecraft-Launcher "{found(info.b("launcher"))}</label>
            <label class="check"><input type="radio" name="cl-dest" prop:checked=move || c.dest.get() == "modrinth" on:change=move |_| c.dest.set("modrinth".into())/>
                " Modrinth-App "{found(info.b("modrinth"))}</label>
        </div>
        {(!existing.is_empty()).then(|| view! {
            <Show when=move || c.dest.get() == "modrinth">
                <div class="dest">
                    <b>"In der Modrinth-App:"</b>
                    <label class="check"><input type="radio" name="cl-mr" prop:checked=move || c.update.get() on:change=move |_| c.update.set(true)/>
                        {format!(" Vorhandene Instanz „{first}“ aktualisieren (empfohlen)")}</label>
                    <label class="check"><input type="radio" name="cl-mr" prop:checked=move || !c.update.get() on:change=move |_| c.update.set(false)/>
                        " Neue Instanz anlegen"</label>
                </div>
            </Show>
        })}
        <Show when=move || c.dest.get() == "launcher" fallback=move || view! {
            <p class="dim small">{move || if updating(c) {
                "Beim Aktualisieren bleibt in der Instanz alles erhalten: Welten, Optionen, Shader, Ressourcenpakete und deine Server-Liste."
            } else {
                "Die Modrinth-App öffnet sich – dort nur noch den Import bestätigen."
            }}</p>
        }>
            <div class="row"><span>"Ordner:"</span><code class="grow">{let d = default_dir.clone(); move || c.dir.get().unwrap_or_else(|| d.clone())}</code>
                <button on:click=pick>"Anderer Ordner …"</button></div>
            <p class="dim small">"Bitte den Minecraft-Launcher vor dem Einrichten schließen, damit er die Änderung nicht überschreibt."</p>
        </Show>
        <Show when=move || !updating(c)>{take_view(c)}</Show>
        <div class="row"><button class="primary" disabled=move || c.installing.get() on:click=move |_| install(app, c)>"Einrichten"</button></div>
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
            {if i.b("launcher") { view! { <span class="pill ok">"Minecraft-Launcher gefunden"</span> }.into_any() } else {
                view! { <span class="pill warn">"Kein Minecraft-Launcher"</span>" "<a href="#" on:click=|e| { e.prevent_default(); open_link("launcher-download"); }>"Download"</a> }.into_any()
            }}
            " "{i.b("modrinth").then(|| view! { <span class="pill ok">"Modrinth-App gefunden"</span> })}
        }
    };
    let row = move |e: Value| {
        let dir = e.s("dir");
        let target = e.s("target");
        let (d_open, d_rm, d_st, t_rm, t_up) = (dir.clone(), dir.clone(), dir.clone(), target.clone(), target.clone());
        let name = e.s("name");
        let remove = move |_| {
            let (dir, name, modrinth) = (d_rm.clone(), name.clone(), t_rm == "modrinth");
            leptos::task::spawn_local(async move {
                let text = if modrinth {
                    "Vapafane vergisst diesen Client und löscht sein Paket. Die Instanz in der Modrinth-App bleibt – die bitte dort selbst löschen."
                } else {
                    "Der Ordner mit Mods, Einstellungen und Einzelspieler-Welten dieses Profils wird gelöscht, und das Profil verschwindet aus dem Minecraft-Launcher."
                };
                if !confirm(format!("„{name}“ entfernen?"), text, "Entfernen").await {
                    return;
                }
                if let Err(er) = tauri::invoke::<()>("client_remove", json!({ "dir": dir })).await {
                    error("Entfernen", er).await;
                }
                refresh(c);
            });
        };
        view! {
            <div class="item">
                <div class="grow">
                    <div><b>{e.s("name")}</b>" "<code>{e.s("address")}</code>" "
                        <span class="st">{move || c.status.get().into_iter().find(|s| s.s("dir") == d_st).map(|s| client_pill(&s))
                            .unwrap_or_else(|| view! { <span class="pill">"prüfe …"</span> }.into_any())}</span></div>
                    <div class="sub">{format!("Minecraft {} · {} · {} Mods · {} · eingerichtet {}", e.s("mc"), loader_name(&e.s("loader")), e.s("mods"),
                        if target == "modrinth" { "Modrinth-App" } else { "Minecraft-Launcher" }, e.s("updated"))}</div>
                </div>
                <button class="primary" on:click=move |_| play(target.clone())>"Spielen"</button>
                <button on:click=move |_| { let modrinth = t_up == "modrinth"; leptos::task::spawn_local(async move {
                    info("Aktualisieren", if modrinth {
                        "Neuen Einladungs-Code (oder Datei) vom Admin oben einfügen und einlösen. Vapafane baut dann ein neues Paket für deine vorhandene Instanz in der Modrinth-App und zeigt, wie du sie damit aktualisierst – Welten, Optionen und Shader bleiben erhalten."
                    } else {
                        "Neuen Einladungs-Code (oder Datei) vom Admin oben einfügen und einlösen – die alten Mods werden dabei sauber ersetzt, deine Einstellungen und Welten bleiben."
                    }).await;
                }); }>"Aktualisieren"</button>
                <button on:click=move |_| { let d = d_open.clone(); leptos::task::spawn_local(async move {
                    if let Err(er) = tauri::invoke::<()>("client_open_folder", json!({ "dir": d })).await { error("Ordner", er).await; }
                }); }>"Ordner öffnen"</button>
                <button class="danger" on:click=remove>"Entfernen"</button>
            </div>
        }
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Client>
            <div class="cf-box">
                <h2 class="client-h">"Einladung einlösen"</h2>
                <p class="dim">"Code (beginnt mit "<code>"VPF1-"</code>") einfügen oder die "<code>".vapafane"</code>"-Datei öffnen – Vapafane richtet Minecraft mit allen Mods passend zum Server ein."</p>
                <textarea id="cl-code" rows="3" placeholder="Einladungs-Code hier einfügen (VPF1-…)" spellcheck="false"
                    prop:value=c.code on:input=move |e| c.code.set(event_target_value(&e))></textarea>
                <div class="row">
                    <button class="primary" on:click=move |_| { let code = c.code.get_untracked().trim().to_string(); if !code.is_empty() { check(c, code) } }>"Einladung prüfen"</button>
                    <button on:click=from_file>"Einladungs-Datei öffnen …"</button>
                    <span class="spacer"></span>
                    <span class="small">{launcher}</span>
                </div>
            </div>
            {move || {
                if c.checking.get() {
                    return Some(view! { <div class="cf-box"><p class="dim">"Prüfe die Einladung …"</p></div> }.into_any());
                }
                // nur bei neuer Einladung neu aufbauen (nicht bei jedem Ping)
                c.source.track();
                c.preview.with_untracked(|p| p.clone()).map(|r| match r {
                    Err(e) => view! { <div class="cf-box"><p class="hint error">{e}</p></div> }.into_any(),
                    Ok(p) => view! { <div class="cf-box">{preview_view(app, c, p)}</div> }.into_any(),
                })
            }}
            <h3>"Meine Server"</h3>
            <div class="list">
                {move || {
                    let list = c.list.get();
                    if list.is_empty() {
                        view! { <div class="empty">"Noch kein Server eingerichtet – oben eine Einladung einlösen."</div> }.into_any()
                    } else {
                        list.into_iter().map(row).collect_view().into_any()
                    }
                }}
            </div>
            <p class="notice small">"Vapafane startet Minecraft nicht selbst und fasst kein Microsoft-Konto an – „Spielen“ öffnet den offiziellen Launcher bzw. die Modrinth-App."</p>
        </section>
    }
}
