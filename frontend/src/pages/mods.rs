//! Mods/Plugins und Datapacks: ein-/ausschalten, hinzufügen (Modrinth/CurseForge), aufräumen.

use leptos::prelude::*;
use serde_json::{Value, json};
use std::rc::Rc;
use std::time::Duration;

use crate::dialog::{self, Btn, ask_view, error, info};
use crate::state::{AppState, Page, use_app};
use crate::tasks::task;
use crate::tauri;
use crate::util::{J, fmt_bytes, fmt_count, open_folder, open_link, thousands};

fn mod_row(
    app: AppState,
    m: Value,
    reload: impl Fn() + Copy + 'static,
    hint: RwSignal<String>,
) -> AnyView {
    let enabled = m.b("enabled");
    let src = m.s("source/source");
    let sub = [
        if m.s("version").is_empty() {
            String::new()
        } else {
            crate::sprache::t!("about-version", version = m.s("version"))
        },
        m.s("file"),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" · ");
    let reason = m.s("reason");
    let (rel, title) = (m.s("rel"), m.s("title"));
    let toggle = move |e: leptos::ev::Event| {
        let on = event_target_checked(&e);
        let (rel, title) = (rel.clone(), title.clone());
        leptos::task::spawn_local(async move {
            match tauri::invoke::<Value>("mod_set_enabled", json!({ "rel": rel, "on": on })).await {
                Ok(r) => {
                    app.add_event(if on {
                        crate::sprache::t!("mods-ev-an", name = title)
                    } else {
                        crate::sprache::t!("mods-ev-aus", name = title)
                    });
                    hint.set(if r.b("restart_needed") {
                        crate::sprache::t!("mods-neustart-noetig")
                    } else {
                        String::new()
                    });
                }
                Err(e) => error(crate::sprache::t!("hdr-err-umschalten"), e).await,
            }
            reload();
        });
    };
    view! {
        <div class="item" class:off=!enabled>
            <div class="grow">
                <div>{m.s("title")}
                    {m.b("client_only").then(|| view! { <span class="badge">{crate::sprache::t!("mods-nur-client")}</span> })}
                    {m.b("folder").then(|| view! { <span class="badge">{crate::sprache::t!("gem-ordner")}</span> })}
                    {(src == "curseforge").then(|| view! { <span class="badge src-cf">"CurseForge"</span> })}
                    {(src == "modrinth").then(|| view! { <span class="badge src-mr">"Modrinth"</span> })}
                </div>
                <div class="sub">{sub}</div>
                {(!enabled && !reason.is_empty()).then(|| view! { <div class="reason">{crate::sprache::t!("mods-aus-grund", grund = reason.clone())}</div> })}
            </div>
            {(!m.b("folder")).then(|| view! {
                <label class="switch" title=if enabled { crate::sprache::t!("mods-an-tipp") } else { crate::sprache::t!("mods-aus-tipp") }>
                    <input type="checkbox" prop:checked=enabled on:change=toggle/><span></span>
                </label>
            })}
        </div>
    }.into_any()
}

#[component]
pub fn Mods() -> impl IntoView {
    let app = use_app();
    let list = RwSignal::new(Value::Null);
    let hint = RwSignal::new(String::new());
    let reload = move || {
        leptos::task::spawn_local(async move {
            list.set(
                tauri::call::<Value>("mods_list")
                    .await
                    .unwrap_or(Value::Null),
            );
        });
    };
    Effect::new(move |_| {
        app.server_path.track();
        if app.page.get() == Page::Mods {
            reload();
        }
    });
    let word = move || {
        if list.get().s("mod_dir") == "plugins" {
            "Plugins"
        } else {
            "Mods"
        }
    };
    let vanilla = move || app.details.get().s("info/type") == "vanilla";
    let count = move || {
        let l = list.get();
        let mods = l.a("mods");
        let on = mods.iter().filter(|m| m.b("enabled")).count();
        let packs = l.a("datapacks").len();
        format!(
            "{}{}",
            crate::sprache::t!("mods-anzahl", an = on as u32, gesamt = mods.len() as u32, wort = word()),
            if packs > 0 {
                format!(" · {}", crate::sprache::t!("mods-datapacks-anzahl", anzahl = packs as u32))
            } else {
                String::new()
            }
        )
    };
    let rows = move |key: &'static str| {
        let l = list.get();
        let items = l.a(key);
        if items.is_empty() {
            let empty = if key == "mods" {
                crate::sprache::t!("mods-leer", wort = word(), ordner = l.s("mod_dir"))
            } else {
                crate::sprache::t!("mods-datapacks-leer")
            };
            return view! { <div class="empty">{empty}</div> }.into_any();
        }
        items
            .into_iter()
            .map(|m| mod_row(app, m, reload, hint))
            .collect_view()
            .into_any()
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Mods id="tab-mods">
            <p class="dim small">{crate::sprache::t!("mods-erklaerung")}</p>
            <div class="row">
                <span class="dim">{count}</span>
                <span class="spacer"></span>
                <button class="primary" on:click=move |_| open_catalog(app, list.get_untracked(), reload)>
                    {move || crate::sprache::t!("mods-hinzufuegen", wort = if vanilla() { "Datapacks" } else { word() })}</button>
                <button on:click=|_| open_folder("mods")>{crate::sprache::t!("mods-ordner-oeffnen")}</button>
                <button title=crate::sprache::t!("mods-aufraeumen-tipp") on:click=move |_| cleanup(app, reload)>{crate::sprache::t!("mods-aufraeumen")}</button>
            </div>
            {move || (!hint.get().is_empty()).then(|| view! { <p class="hint">{hint.get()}</p> })}
            <h3>{word}</h3>
            <div class="list">{move || rows("mods")}</div>
            <h3>{crate::sprache::t!("mods-datapacks-titel")}</h3>
            <div class="list">{move || rows("datapacks")}</div>
        </section>
    }
}

// ---------------------------------------------------------------- Aufräumen

fn cleanup(app: AppState, reload: impl Fn() + Copy + 'static) {
    leptos::task::spawn_local(async move {
        let p = match tauri::call::<Value>("cleanup_preview").await {
            Ok(p) => p,
            Err(e) => return error(crate::sprache::t!("mods-aufraeumen-titel"), e).await,
        };
        let items = p.a("items");
        if items.is_empty() {
            return info(
                crate::sprache::t!("mods-aufraeumen-titel"),
                crate::sprache::t!("mods-alles-sauber"),
            )
            .await;
        }
        let stopped = matches!(
            app.status.get_untracked().state.as_str(),
            "stopped" | "failed" | "none"
        );
        let total = fmt_bytes(p.f("total"));
        let t2 = total.clone();
        let body: dialog::BodyFn = Rc::new(move || {
            let items = items.clone();
            view! {
                <p>{crate::sprache::t!("mods-aufraeumen-text-1")}" " <b>{t2.clone()}</b>
                    {format!("). {}", crate::sprache::t!("mods-aufraeumen-text-2"))}</p>
                <div class="cleanup-list">
                    {items.into_iter().map(|c| view! {
                        <div><span class="sz">{fmt_bytes(c.f("size"))}</span>
                            <span><code>{c.s("rel")}</code><br/><span class="dim small">{c.s("what")}</span></span></div>
                    }).collect_view()}
                </div>
                {(!stopped).then(|| view! { <p class="hint">{crate::sprache::t!("mods-erst-stoppen")}</p> })}
            }.into_any()
        });
        let buttons = if stopped {
            vec![
                Btn::new(crate::sprache::t!("gem-abbrechen"), ""),
                Btn::primary(crate::sprache::t!("mods-loeschen-knopf", groesse = total.clone()), "ok"),
            ]
        } else {
            vec![Btn::primary(crate::sprache::t!("gem-schliessen"), "")]
        };
        if ask_view(crate::sprache::t!("mods-aufraeumen-titel"), body, buttons, false).await != "ok" {
            return;
        }
        match task("cleanup_run", json!({})).await {
            Ok(r) => {
                app.add_event(crate::sprache::t!("mods-ev-aufgeraeumt", groesse = fmt_bytes(r.f("freed"))));
                let failed: Vec<String> = r
                    .a("failed")
                    .iter()
                    .filter_map(|f| f.as_str().map(String::from))
                    .collect();
                info(
                    crate::sprache::t!("mods-aufgeraeumt-titel"),
                    format!(
                        "{}{}",
                        crate::sprache::t!("mods-aufgeraeumt", anzahl = r.s("count"), groesse = fmt_bytes(r.f("freed"))),
                        if failed.is_empty() {
                            String::new()
                        } else {
                            format!("\n\n{}\n{}", crate::sprache::t!("mods-nicht-loeschbar"), failed.join("\n"))
                        }
                    ),
                )
                .await;
            }
            Err(e) => error(crate::sprache::t!("mods-err-aufraeumen"), e).await,
        }
        reload();
    });
}

// ---------------------------------------------------------------- Katalog (Modrinth/CurseForge)

#[derive(Clone, Copy)]
struct Catalog {
    query: RwSignal<String>,
    datapacks: RwSignal<bool>,
    hits: RwSignal<Vec<Value>>,
    cf_hits: RwSignal<Vec<Value>>,
    info: RwSignal<String>,
    cf_note: RwSignal<String>,
    msg: RwSignal<Option<Value>>,
    installed: RwSignal<Vec<String>>,
    offset: RwSignal<usize>,
    cf_offset: RwSignal<Option<u64>>,
    total: RwSignal<u64>,
    loading: RwSignal<bool>,
    seq: RwSignal<u32>,
}

fn load(c: Catalog, word: &'static str, reset: bool) {
    if reset {
        c.offset.set(0);
        c.cf_offset.set(Some(0));
        c.hits.set(vec![]);
        c.cf_hits.set(vec![]);
    }
    c.loading.set(true);
    let seq = c.seq.get_untracked() + 1;
    c.seq.set(seq);
    leptos::task::spawn_local(async move {
        let args = json!({
            "query": c.query.get_untracked(), "offset": c.offset.get_untracked(),
            "cfOffset": c.cf_offset.get_untracked().unwrap_or(0), "datapacks": c.datapacks.get_untracked(),
        });
        let r = tauri::invoke::<Value>("catalog_search", args).await;
        if c.seq.get_untracked() != seq {
            return; // neuere Suche läuft schon
        }
        c.loading.set(false);
        let r = match r {
            Ok(r) => r,
            Err(e) => return c.info.set(crate::sprache::t!("kon-fehler", fehler = e)),
        };
        if r.b("curseforge") {
            let cf = r.a("cf_hits");
            c.cf_hits.update(|h| h.extend(cf));
            let (off, tot) = (
                r.f("cf_offset").unwrap_or(0.0),
                r.f("cf_total").unwrap_or(0.0),
            );
            c.cf_offset
                .set((off > 0.0 && off < tot).then_some(off as u64));
            c.cf_note.set(if r.s("cf_error").is_empty() {
                String::new()
            } else {
                format!("CurseForge: {}", r.s("cf_error"))
            });
        } else {
            c.cf_offset.set(None);
            c.cf_note.set(crate::sprache::t!("kat-cf-tipp"));
        }
        let total = r.f("total").unwrap_or(0.0) as u64;
        c.total.set(total);
        c.info.set(format!(
            "{}{}",
            crate::sprache::t!("kat-treffer", anzahl = thousands(total),
                wort = if c.datapacks.get_untracked() { "Datapacks".to_string() } else { word.to_string() },
                mc = r.s("mc")),
            if c.query.get_untracked().trim().is_empty() {
                format!(" – {}", crate::sprache::t!("kat-beliebtheit"))
            } else {
                String::new()
            }
        ));
        let hits = r.a("hits");
        c.offset.update(|o| *o += hits.len());
        c.hits.update(|h| h.extend(hits));
    });
}

fn hit_row(app: AppState, c: Catalog, h: Value) -> AnyView {
    let id = h.s("id");
    let busy = RwSignal::new(false);
    let is_in = Memo::new(move |_| c.installed.get().contains(&id));
    let side = if h.s("source") == "curseforge" {
        view! { <span class="badge src-cf">"CurseForge"</span> }.into_any()
    } else if h.b("client_required") {
        view! { <span class="badge both" title=crate::sprache::t!("kat-beides-tipp")>{crate::sprache::t!("kat-beides")}</span> }.into_any()
    } else {
        view! { <span class="badge">{crate::sprache::t!("kat-nur-server")}</span> }.into_any()
    };
    let icon = h.s("icon");
    let hh = h.clone();
    let install = move |_| {
        let h = hh.clone();
        busy.set(true);
        leptos::task::spawn_local(async move {
            let r = task("catalog_install", json!({ "project": h.s("id"), "title": h.s("title"), "datapacks": c.datapacks.get_untracked() })).await;
            if let Ok(list) = tauri::call::<Vec<String>>("catalog_installed").await {
                c.installed.set(list);
            }
            // Dialog evtl. schon zu – dann gibt es den Knopf nicht mehr
            let _ = busy.try_set(false);
            match r {
                Ok(mut r) => {
                    app.add_event(crate::sprache::t!("kat-ev-installiert", name = r.s("title")));
                    r["client_required"] = json!(h.b("client_required"));
                    c.msg.set(Some(r));
                }
                Err(e) => c.msg.set(Some(json!({ "error": e }))),
            }
        });
    };
    view! {
        <div class="hit">
            {if icon.is_empty() { view! { <div class="noicon"></div> }.into_any() } else { view! { <img src=icon alt="" loading="lazy"/> }.into_any() }}
            <div class="txt">
                <div class="t">{h.s("title")} " " {side}</div>
                <div class="d">{h.s("description")}</div>
                <div class="meta">{crate::sprache::t!("kat-meta", downloads = fmt_count(h.f("downloads").unwrap_or(0.0)), autor = if h.s("author").is_empty() { "?".into() } else { h.s("author") })}</div>
            </div>
            <button class=move || if is_in.get() { "badge ok" } else { "primary" } disabled=move || is_in.get() || busy.get() on:click=install>
                {move || if busy.get() { crate::sprache::t!("kat-laedt") } else if is_in.get() { crate::sprache::t!("kat-installiert") } else { crate::sprache::t!("kat-installieren") }}
            </button>
        </div>
    }.into_any()
}

/// Meldung nach dem Installieren.
fn install_msg(app: AppState, r: Value) -> AnyView {
    if !r.s("error").is_empty() {
        return view! { <p class="hint">{crate::sprache::t!("kat-err-install", fehler = r.s("error"))}</p> }.into_any();
    }
    let folder = r.s("folder");
    let installed = r.a("installed");
    let extra: Vec<String> = installed.iter().skip(1).map(|x| x.s("title")).collect();
    let skipped: Vec<String> = r
        .a("skipped")
        .iter()
        .filter_map(|x| x.as_str().map(String::from))
        .collect();
    let missing: Vec<String> = r
        .a("missing")
        .iter()
        .filter_map(|x| x.as_str().map(String::from))
        .collect();
    let manual = r.a("manual");
    let dp = folder.contains("datapacks");
    view! {
        <div class="hint">
            {(!installed.is_empty()).then(|| view! {
                <p><b>{r.s("title")}</b>{format!(" {}{}.", crate::sprache::t!("kat-ist-installiert", ordner = folder.clone()), if extra.is_empty() { String::new() } else { format!(" {}", crate::sprache::t!("kat-abhaengigkeiten", namen = extra.join(", "))) })}
                    {(!skipped.is_empty()).then(|| format!(" {}", crate::sprache::t!("kat-schon-da", namen = skipped.join(", "))))}</p>
            })}
            {(!missing.is_empty()).then(|| view! { <p>{crate::sprache::t!("kat-fehlende", namen = missing.join(", "))}</p> })}
            {(r.b("client_required") && !installed.is_empty()).then(|| view! { <p>{crate::sprache::t!("kat-client-noetig")}</p> })}
            {(!manual.is_empty()).then(|| view! {
                {manual.iter().map(|m| view! { <p><b>{m.s("title")}</b>": "{crate::sprache::t!("kat-manuell-1")}" "
                    <code>{m.s("file")}</code>{format!(" {}", crate::sprache::t!("kat-manuell-2", ordner = r.s("folder")))}</p> }).collect_view()}
                <div class="row">
                    {manual.iter().map(|m| { let page = m.s("page"); view! { <button on:click=move |_| open_link(page.clone())>{crate::sprache::t!("kat-browser")}</button> } }).collect_view()}
                    <button on:click=move |_| open_folder(if dp { "datapacks" } else { "mods" })>{if dp { crate::sprache::t!("kat-dp-ordner") } else { crate::sprache::t!("mods-ordner-oeffnen") }}</button>
                </div>
            })}
            {(r.b("restart_needed") && !installed.is_empty()).then(|| view! {
                <p>{crate::sprache::t!("kat-neustart")}" "
                    <button on:click=move |_| { dialog::close(""); crate::pages::overview::ask_stop(app, true); }>{crate::sprache::t!("kat-jetzt-neustarten")}</button></p>
            })}
        </div>
    }.into_any()
}

fn open_catalog(app: AppState, mods_info: Value, reload: impl Fn() + Copy + 'static) {
    let vanilla = app.details.get_untracked().s("info/type") == "vanilla";
    let word: &'static str = if vanilla {
        "Datapacks"
    } else if mods_info.s("mod_dir") == "plugins" {
        "Plugins"
    } else {
        "Mods"
    };
    let c = Catalog {
        query: RwSignal::new(String::new()),
        datapacks: RwSignal::new(vanilla),
        hits: RwSignal::new(vec![]),
        cf_hits: RwSignal::new(vec![]),
        info: RwSignal::new(String::new()),
        cf_note: RwSignal::new(String::new()),
        msg: RwSignal::new(None),
        installed: RwSignal::new(vec![]),
        offset: RwSignal::new(0),
        cf_offset: RwSignal::new(Some(0)),
        total: RwSignal::new(0),
        loading: RwSignal::new(false),
        seq: RwSignal::new(0),
    };
    let body: dialog::BodyFn = Rc::new(move || {
        let timer = RwSignal::new(0u32);
        let on_input = move |e: leptos::ev::Event| {
            c.query.set(event_target_value(&e));
            let t = timer.get_untracked() + 1;
            timer.set(t);
            set_timeout(
                move || {
                    if timer.try_get_untracked() == Some(t) {
                        load(c, word, true)
                    }
                },
                Duration::from_millis(350),
            );
        };
        view! {
            <div class="catalog-bar">
                <input type="search" placeholder=crate::sprache::t!("kat-suche") autocomplete="off" spellcheck="false"
                    prop:value=c.query on:input=on_input on:keydown=move |e| if e.key() == "Enter" { timer.update(|t| *t += 1); load(c, word, true) }/>
                {(!vanilla).then(|| view! {
                    <select on:change=move |e| { c.datapacks.set(event_target_value(&e) == "datapacks"); load(c, word, true); }>
                        <option value="mods">{word}</option><option value="datapacks">{crate::sprache::t!("kat-dp-option")}</option>
                    </select>
                })}
            </div>
            <div class="dim small">{move || c.info.get()}</div>
            {move || c.msg.get().map(|m| install_msg(app, m))}
            <div class="catalog-list">
                {move || {
                    let hits = c.hits.get();
                    if hits.is_empty() {
                        let t = if c.loading.get() { crate::sprache::t!("kat-sucht") } else { crate::sprache::t!("kat-nichts") };
                        view! { <div class="empty">{t}</div> }.into_any()
                    } else {
                        hits.into_iter().map(|h| hit_row(app, c, h)).collect_view().into_any()
                    }
                }}
            </div>
            <Show when=move || !c.cf_hits.get().is_empty()>
                <div>
                    <h3>{crate::sprache::t!("kat-nur-cf")}" " <span class="badge exp">{crate::sprache::t!("cf-experimentell")}</span></h3>
                    <p class="dim small">{crate::sprache::t!("kat-cf-erklaerung")}</p>
                    <div class="catalog-list">{move || c.cf_hits.get().into_iter().map(|h| hit_row(app, c, h)).collect_view()}</div>
                </div>
            </Show>
            <p class="dim small">{move || c.cf_note.get()}</p>
            <div class="row">
                <Show when=move || (c.offset.get() as u64) < c.total.get() || c.cf_offset.get().is_some_and(|o| o > 0)>
                    <button on:click=move |_| load(c, word, false)>{crate::sprache::t!("kat-mehr")}</button>
                </Show>
            </div>
        }.into_any()
    });
    leptos::task::spawn_local(async move {
        c.installed.set(
            tauri::call::<Vec<String>>("catalog_installed")
                .await
                .unwrap_or_default(),
        );
        load(c, word, true);
        ask_view(
            crate::sprache::t!("mods-hinzufuegen-titel", wort = word),
            body,
            vec![Btn::primary(crate::sprache::t!("gem-schliessen"), "")],
            true,
        )
        .await;
        reload();
    });
}
