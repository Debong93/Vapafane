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

/// Texte aus einer JSON-Liste (z. B. Namen der Mods, die eine Datei brauchen).
fn texte(v: &[Value]) -> Vec<String> {
    v.iter().filter_map(|x| x.as_str().map(String::from)).collect()
}

/// Symbol der Mod: Bild von Modrinth/CurseForge oder Platzhalter mit den Anfangsbuchstaben.
fn mod_icon(icon: String, title: &str) -> AnyView {
    if icon.is_empty() {
        let kurz: String = title.chars().filter(|c| c.is_alphanumeric()).take(2).collect();
        view! { <div class="mod-icon none" aria-hidden="true">{kurz}</div> }.into_any()
    } else {
        view! { <img class="mod-icon" src=icon alt="" loading="lazy"/> }.into_any()
    }
}

/// Schalter umlegen. Vor dem Ausschalten warnen, wenn andere eingeschaltete Mods ihn brauchen.
fn umschalten(app: AppState, rel: String, title: String, on: bool, reload: impl Fn() + Copy + 'static, hint: RwSignal<String>) {
    leptos::task::spawn_local(async move {
        if !on {
            let c = tauri::invoke::<Value>("mod_check", json!({ "rel": rel })).await.unwrap_or(Value::Null);
            let needed = texte(&c.a("needed_by"));
            if !needed.is_empty() {
                let text = crate::sprache::t!("mods-gebraucht-warnung", namen = needed.join(", "));
                let body: dialog::BodyFn = Rc::new(move || view! { <p class="hint warn">{text.clone()}</p> }.into_any());
                let wahl = ask_view(
                    crate::sprache::t!("mods-aus-frage", name = title.clone()),
                    body,
                    vec![
                        Btn::new(crate::sprache::t!("gem-abbrechen"), ""),
                        Btn::primary(crate::sprache::t!("mods-aus-knopf"), "ok"),
                    ],
                    false,
                )
                .await;
                if wahl != "ok" {
                    return reload(); // Schalter zurückstellen
                }
            }
        }
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
}

/// Endgültig entfernen – mit Warnung, wenn andere Mods die Datei brauchen, und dem
/// Angebot, Helfer mitzunehmen, die danach kein anderer Mod mehr braucht.
fn entfernen(app: AppState, rel: String, title: String, reload: impl Fn() + Copy + 'static, hint: RwSignal<String>) {
    leptos::task::spawn_local(async move {
        let c = tauri::invoke::<Value>("mod_check", json!({ "rel": rel })).await.unwrap_or(Value::Null);
        let needed = texte(&c.a("needed_by"));
        // (Datei, Name, Häkchen) – vorausgewählt nur, was als Abhängigkeit mitkam
        let helfer: Vec<(String, String, RwSignal<bool>)> = c
            .a("orphans")
            .iter()
            .map(|o| (o.s("rel"), o.s("title"), RwSignal::new(o.b("suggest"))))
            .collect();
        let h2 = helfer.clone();
        let body: dialog::BodyFn = Rc::new(move || {
            let warnung = (!needed.is_empty())
                .then(|| crate::sprache::t!("mods-gebraucht-warnung", namen = needed.join(", ")));
            view! {
                <p>{crate::sprache::t!("mods-entfernen-text")}</p>
                {warnung.map(|w| view! { <p class="hint warn">{w}</p> })}
                {(!h2.is_empty()).then(|| view! {
                    <p>{crate::sprache::t!("mods-waisen-frage")}</p>
                    {h2.iter().map(|(_, name, an)| {
                        let an = *an;
                        view! {
                            <label class="check">
                                <input type="checkbox" prop:checked=move || an.get() on:change=move |e| an.set(event_target_checked(&e))/>
                                " " {name.clone()}
                            </label>
                        }
                    }).collect_view()}
                })}
            }
            .into_any()
        });
        let wahl = ask_view(
            crate::sprache::t!("mods-entfernen-frage", name = title.clone()),
            body,
            vec![
                Btn::new(crate::sprache::t!("gem-abbrechen"), ""),
                Btn::primary(crate::sprache::t!("gem-entfernen"), "ok"),
            ],
            false,
        )
        .await;
        if wahl != "ok" {
            return;
        }
        let mut rels = vec![rel];
        rels.extend(helfer.iter().filter(|(_, _, an)| an.get_untracked()).map(|(r, _, _)| r.clone()));
        match tauri::invoke::<Value>("mod_remove", json!({ "rels": rels })).await {
            Ok(r) => {
                app.add_event(crate::sprache::t!("mods-ev-entfernt", name = title));
                hint.set(if r.b("restart_needed") {
                    crate::sprache::t!("mods-neustart-noetig")
                } else {
                    String::new()
                });
            }
            Err(e) => error(crate::sprache::t!("mods-err-entfernen"), e).await,
        }
        reload();
    });
}

fn mod_row(
    app: AppState,
    m: Value,
    d: Value,
    reload: impl Fn() + Copy + 'static,
    hint: RwSignal<String>,
) -> AnyView {
    let enabled = m.b("enabled");
    let folder = m.b("folder");
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
    // Kurzbeschreibung von Modrinth/CurseForge, sonst aus der Mod-Datei
    let desc = Some(d.s("desc")).filter(|x| !x.is_empty()).unwrap_or_else(|| m.s("desc"));
    let needed = texte(&d.a("needed_by"));
    let page = d.s("page");
    let reason = m.s("reason");
    let title = if m.s("kind") == "datapack" && !d.s("title").is_empty() { d.s("title") } else { m.s("title") };
    let rel = m.s("rel");
    let (rel2, title2) = (rel.clone(), title.clone());
    let anzeige = title.clone();
    let toggle = move |e: leptos::ev::Event| umschalten(app, rel.clone(), title.clone(), event_target_checked(&e), reload, hint);
    let remove = move |_| entfernen(app, rel2.clone(), title2.clone(), reload, hint);
    view! {
        <div class="item mod-row" class:off=!enabled>
            {mod_icon(d.s("icon"), &anzeige)}
            <div class="grow">
                <div class="mod-title">{anzeige.clone()}
                    {(!enabled).then(|| view! { <span class="badge">{crate::sprache::t!("set-aus")}</span> })}
                    {d.b("lib").then(|| view! { <span class="badge lib" title=crate::sprache::t!("mods-bibliothek-tipp")>{crate::sprache::t!("mods-bibliothek")}</span> })}
                    {m.b("client_only").then(|| view! { <span class="badge">{crate::sprache::t!("mods-nur-client")}</span> })}
                    {folder.then(|| view! { <span class="badge">{crate::sprache::t!("gem-ordner")}</span> })}
                    {(src == "curseforge").then(|| view! { <span class="badge src-cf">"CurseForge"</span> })}
                    {(src == "modrinth").then(|| view! { <span class="badge src-mr">"Modrinth"</span> })}
                </div>
                {(!desc.is_empty()).then(|| view! { <div class="mod-desc">{desc.clone()}</div> })}
                <div class="sub">{sub}</div>
                {(!needed.is_empty()).then(|| view! { <div class="sub needed">{crate::sprache::t!("mods-gebraucht-von", namen = needed.join(", "))}</div> })}
                {(!enabled && !reason.is_empty()).then(|| view! { <div class="reason">{crate::sprache::t!("mods-aus-grund", grund = reason.clone())}</div> })}
            </div>
            {(!page.is_empty()).then(|| view! {
                <button class="linkbtn" title=crate::sprache::t!("mods-seite-tipp") on:click=move |_| open_link(page.clone())>
                    {crate::sprache::t!("mods-seite")}" ↗"</button>
            })}
            {(!folder).then(|| view! {
                <label class="switch" title=if enabled { crate::sprache::t!("mods-an-tipp") } else { crate::sprache::t!("mods-aus-tipp") }>
                    <input type="checkbox" prop:checked=enabled on:change=toggle/><span></span>
                </label>
            })}
            <button class="icon-btn" title=crate::sprache::t!("mods-entfernen-tipp") aria-label=crate::sprache::t!("mods-entfernen-tipp") on:click=remove>
                <span class="px-ico ico-trash"></span>
            </button>
        </div>
    }.into_any()
}

#[component]
pub fn Mods() -> impl IntoView {
    let app = use_app();
    let list = RwSignal::new(Value::Null);
    // Datei → Symbol, Kurzbeschreibung, Link, Bibliothek?, gebraucht von (kommt kurz nach der Liste)
    let details = RwSignal::new(Value::Null);
    let filter = RwSignal::new(String::new());
    let hint = RwSignal::new(String::new());
    let reload = move || {
        leptos::task::spawn_local(async move {
            list.set(
                tauri::call::<Value>("mods_list")
                    .await
                    .unwrap_or(Value::Null),
            );
            if let Ok(d) = tauri::call::<Value>("mods_details").await {
                details.set(d);
            }
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
        if vanilla() {
            return crate::sprache::t!("mods-datapacks-anzahl", anzahl = l.a("datapacks").len() as u32);
        }
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
    // Suchfeld erst bei längeren Listen (z. B. Modpacks)
    let viele = move || {
        let l = list.get();
        l.a("mods").len() + l.a("datapacks").len() >= 8
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
        let d = details.get();
        let f = filter.get().trim().to_lowercase();
        let passt = |m: &Value| {
            let beschreibung = d.get(m.s("rel")).map(|x| x.s("desc")).unwrap_or_default();
            f.is_empty()
                || [m.s("title"), m.s("file"), m.s("desc"), beschreibung]
                    .iter()
                    .any(|t| t.to_lowercase().contains(&f))
        };
        let treffer: Vec<Value> = items.into_iter().filter(passt).collect();
        if treffer.is_empty() {
            return view! { <div class="empty">{crate::sprache::t!("kat-nichts")}</div> }.into_any();
        }
        treffer
            .into_iter()
            .map(|m| {
                let info = d.get(m.s("rel")).cloned().unwrap_or(Value::Null);
                mod_row(app, m, info, reload, hint)
            })
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
                <button on:click=move |_| open_folder(if vanilla() { "datapacks" } else { "mods" })>
                    {move || if vanilla() { crate::sprache::t!("kat-dp-ordner") } else { crate::sprache::t!("mods-ordner-oeffnen") }}</button>
                <button title=crate::sprache::t!("mods-aufraeumen-tipp") on:click=move |_| cleanup(app, reload)>{crate::sprache::t!("mods-aufraeumen")}</button>
            </div>
            {move || (!hint.get().is_empty()).then(|| view! { <p class="hint">{hint.get()}</p> })}
            <Show when=viele>
                <input type="search" class="mods-filter" placeholder=crate::sprache::t!("mods-suche") autocomplete="off" spellcheck="false"
                    prop:value=filter on:input=move |e| filter.set(event_target_value(&e))/>
            </Show>
            <Show when=move || !vanilla()>
                <h3>{word}</h3>
                <div class="list">{move || rows("mods")}</div>
            </Show>
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
    /// Empfohlene Leistungs-Mods, die es für diesen Server fertig gibt
    empf: RwSignal<Vec<Value>>,
    empf_busy: RwSignal<bool>,
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
    let id_empf = id.clone();
    let busy = RwSignal::new(false);
    let is_in = Memo::new(move |_| c.installed.get().contains(&id));
    // Empfohlen? Dann mit Abzeichen und der übersetzten Kurzbeschreibung statt der englischen
    let empf = Memo::new(move |_| c.empf.get().iter().find(|e| e.s("id") == id_empf).map(|e| e.s("why")));
    let desc = h.s("description");
    let side = if h.s("source") == "curseforge" {
        view! { <span class="badge src-cf">"CurseForge"</span> }.into_any()
    } else if h.b("client_required") {
        view! { <span class="badge both" title=crate::sprache::t!("kat-beides-tipp")>{crate::sprache::t!("kat-beides")}</span> }.into_any()
    } else {
        view! { <span class="badge">{crate::sprache::t!("kat-nur-server")}</span> }.into_any()
    };
    let icon = h.s("icon");
    // Titel öffnet die Mod-Seite – so lässt sich vor dem Installieren nachlesen, was die Mod macht
    let page = h.s("page");
    let titel = if page.is_empty() {
        view! { <span>{h.s("title")}</span> }.into_any()
    } else {
        view! {
            <a href="#" title=crate::sprache::t!("mods-seite-tipp")
                on:click=move |e: leptos::ev::MouseEvent| { e.prevent_default(); open_link(page.clone()); }>
                {h.s("title")}" ↗"</a>
        }
        .into_any()
    };
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
                <div class="t">{titel} " " {side}
                    {move || empf.get().map(|_| view! { <span class="badge empf" title=crate::sprache::t!("empf-badge-tipp")>{crate::sprache::t!("empf-badge")}</span> })}</div>
                <div class="d">{move || empf.get().unwrap_or_else(|| desc.clone())}</div>
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
    if r.b("many") {
        return empf_msg(app, r);
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

/// Meldung nach „Alle empfohlenen installieren“.
fn empf_msg(app: AppState, r: Value) -> AnyView {
    let installed = texte(&r.a("installed"));
    let missing = texte(&r.a("missing"));
    let failed: Vec<String> = r.a("failed").iter().map(|x| format!("{} ({})", x.s("title"), x.s("error"))).collect();
    let restart = r.b("restart_needed") && !installed.is_empty();
    view! {
        <div class="hint">
            {(!installed.is_empty()).then(|| view! { <p>{crate::sprache::t!("empf-installiert", namen = installed.join(", "), ordner = r.s("folder"))}</p> })}
            {(!missing.is_empty()).then(|| view! { <p>{crate::sprache::t!("kat-fehlende", namen = missing.join(", "))}</p> })}
            {(!failed.is_empty()).then(|| view! { <p>{crate::sprache::t!("kat-err-install", fehler = failed.join(", "))}</p> })}
            {restart.then(|| view! {
                <p>{crate::sprache::t!("kat-neustart")}" "
                    <button on:click=move |_| { dialog::close(""); crate::pages::overview::ask_stop(app, true); }>{crate::sprache::t!("kat-jetzt-neustarten")}</button></p>
            })}
        </div>
    }
    .into_any()
}

/// Alle empfohlenen Mods, die noch fehlen, in einem Rutsch installieren.
fn install_empf(app: AppState, c: Catalog) {
    c.empf_busy.set(true);
    leptos::task::spawn_local(async move {
        let r = task("catalog_install_recommended", json!({})).await;
        if let Ok(list) = tauri::call::<Vec<String>>("catalog_installed").await {
            c.installed.set(list);
        }
        c.empf_busy.set(false);
        match r {
            Ok(r) => {
                let namen = texte(&r.a("installed"));
                if !namen.is_empty() {
                    app.add_event(crate::sprache::t!("empf-ev", namen = namen.join(", ")));
                }
                c.msg.set(Some(r));
            }
            Err(e) => c.msg.set(Some(json!({ "error": e }))),
        }
    });
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
        empf: RwSignal::new(vec![]),
        empf_busy: RwSignal::new(false),
    };
    // Empfehlungen nur ohne Suchbegriff und nicht bei Datapacks
    let zeige_empf = move || c.query.get().trim().is_empty() && !c.datapacks.get() && !c.empf.get().is_empty();
    let alle_da = move || {
        let da = c.installed.get();
        c.empf.get().iter().all(|e| da.contains(&e.s("id")))
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
            {move || c.msg.get().map(|m| install_msg(app, m))}
            <Show when=zeige_empf>
                <div class="empf-block">
                    <h3>{crate::sprache::t!("empf-titel")}</h3>
                    <p class="dim small">{crate::sprache::t!("empf-text")}</p>
                    <div class="catalog-list">{move || c.empf.get().into_iter().map(|h| hit_row(app, c, h)).collect_view()}</div>
                    <Show when=move || !alle_da()>
                        <div class="row">
                            <button class="primary" disabled=move || c.empf_busy.get() on:click=move |_| install_empf(app, c)>
                                {move || if c.empf_busy.get() { crate::sprache::t!("kat-laedt") } else { crate::sprache::t!("empf-alle") }}</button>
                        </div>
                    </Show>
                </div>
            </Show>
            <div class="dim small">{move || c.info.get()}</div>
            <div class="catalog-list">
                {move || {
                    // Was oben schon empfohlen steht, nicht doppelt zeigen
                    let doppelt: Vec<String> = if zeige_empf() { c.empf.get().iter().map(|e| e.s("id")).collect() } else { vec![] };
                    let hits: Vec<Value> = c.hits.get().into_iter().filter(|h| !doppelt.contains(&h.s("id"))).collect();
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
        if !vanilla {
            leptos::task::spawn_local(async move {
                if let Ok(liste) = tauri::call::<Vec<Value>>("catalog_recommended").await {
                    c.empf.set(liste);
                }
            });
        }
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
