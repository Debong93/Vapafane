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
}

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
    leptos::task::spawn_local(async move {
        let r = tauri::invoke::<Value>("client_preview", json!({ "source": source })).await;
        let target = c.info.get_untracked().s("target");
        c.dest.set(if target.is_empty() {
            "launcher".into()
        } else {
            target
        });
        c.preview.set(Some(r));
        c.checking.set(false);
    });
}

fn install(app: AppState, c: Client) {
    let Some(Ok(p)) = c.preview.get_untracked() else {
        return;
    };
    c.installing.set(true);
    leptos::task::spawn_local(async move {
        let args = json!({ "source": p.s("source"), "target": c.dir.get_untracked(), "selected": c.selected.get_untracked(), "destination": c.dest.get_untracked() });
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
    let (d1, d2) = (dir.clone(), dir.clone());
    let open_dir = move |d: String| {
        leptos::task::spawn_local(async move {
            if let Err(e) = tauri::invoke::<()>("client_open_folder", json!({ "dir": d })).await {
                show_error("Ordner", e);
            }
        })
    };
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
        <Show when=move || c.dest.get() == "launcher" fallback=|| view! {
            <p class="dim small">"Die Modrinth-App öffnet sich – dort nur noch den Import bestätigen. Nach einem Server-Update entsteht dort eine neue Instanz; die alte dann löschen."</p>
        }>
            <div class="row"><span>"Ordner:"</span><code class="grow">{let d = default_dir.clone(); move || c.dir.get().unwrap_or_else(|| d.clone())}</code>
                <button on:click=pick>"Anderer Ordner …"</button></div>
            <p class="dim small">"Bitte den Minecraft-Launcher vor dem Einrichten schließen, damit er die Änderung nicht überschreibt."</p>
        </Show>
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
        let (d_open, d_rm, d_st, t_rm) = (dir.clone(), dir.clone(), dir.clone(), target.clone());
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
                <button on:click=move |_| leptos::task::spawn_local(async {
                    info("Aktualisieren", "Neuen Einladungs-Code (oder Datei) vom Admin oben einfügen und einlösen – die alten Mods werden dabei sauber ersetzt.").await;
                })>"Aktualisieren"</button>
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
