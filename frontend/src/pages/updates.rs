//! Version & Updates: Zielversion wählen, prüfen, Probelauf, Update, Zurück, Export (.mrpack).

use leptos::prelude::*;
use serde_json::{Value, json};

use crate::dialog::{Btn, ask, confirm, error, info};
use crate::state::{AppState, Page, use_app};
use crate::tasks::task;
use crate::tauri;
use crate::util::{J, fmt_bytes};

fn status_text(m: &Value) -> String {
    let st = m.s("status");
    if m.b("disabled") && st == "update" {
        return format!("✓ {}", crate::sprache::t!("upd-st-reenable"));
    }
    match st.as_str() {
        "update" => "✓ Update".to_string(),
        "current" => format!("= {}", crate::sprache::t!("upd-st-aktuell")),
        "missing" => format!("✗ {}", crate::sprache::t!("upd-st-fehlt")),
        "unknown" => format!("? {}", crate::sprache::t!("upd-st-unbekannt")),
        "new" => format!("+ {}", crate::sprache::t!("upd-st-neu")),
        "manual" => format!("↗ {}", crate::sprache::t!("upd-st-manuell")),
        "reenable" => format!("✓ {}", crate::sprache::t!("upd-st-reenable")),
        "duplicate" => format!("⇄ {}", crate::sprache::t!("upd-st-doppelt")),
        _ => st,
    }
}

fn java_hint(n: &str) -> String {
    let windows = web_sys::window()
        .and_then(|w| w.navigator().user_agent().ok())
        .is_some_and(|u| u.contains("Windows"));
    if windows {
        crate::sprache::t!("java-hinweis-windows", version = n)
    } else {
        crate::sprache::t!("java-hinweis-linux", version = n)
    }
}

/// Zusammenfassung unter der Tabelle.
fn summary(plan: &Value, has_work: bool) -> String {
    if !has_work {
        return crate::sprache::t!("upd-alles-aktuell");
    }
    let all: Vec<Value> = plan
        .a("mods")
        .into_iter()
        .chain(plan.a("datapacks"))
        .collect();
    let count = |st: &str| all.iter().filter(|m| m.s("status") == st).count();
    let new_deps = plan
        .a("extra")
        .iter()
        .filter(|e| e.s("status") == "new")
        .count();
    let mut first = crate::sprache::t!(
        "upd-zusammenfassung",
        updates = count("update") as u32,
        aktuell = count("current") as u32,
        fehlend = count("missing") as u32,
        unbekannt = count("unknown") as u32
    );
    if new_deps > 0 {
        first += &format!(", {}", crate::sprache::t!("upd-neu-gebraucht", anzahl = new_deps as u32));
    }
    if count("reenable") > 0 {
        first += &format!(", {}", crate::sprache::t!("upd-passen-wieder", anzahl = count("reenable") as u32));
    }
    if count("duplicate") > 0 {
        first += &format!(", {}", crate::sprache::t!("upd-doppelt", anzahl = count("duplicate") as u32));
    }
    let mut lines = vec![first];
    let server = plan.o("server");
    let change = plan.b("change_version");
    if !server.is_null() && !change {
        lines.push(crate::sprache::t!("upd-server-stand", mc = plan.s("target"), label = server.s("label")));
    }
    if server.b("beta") {
        lines.push(crate::sprache::t!("upd-beta-hinweis"));
    }
    if change {
        lines.push(crate::sprache::t!("upd-wechsel", von = plan.s("current"), nach = plan.s("target"),
            label = if server.is_null() { "?".into() } else { server.s("label") }));
        lines.push(crate::sprache::t!("upd-welt-warnung"));
    }
    let java = plan.s("java_required");
    if !java.is_empty() {
        lines.push(if plan.b("java_ok") {
            crate::sprache::t!("upd-java-da", version = java.clone(), pfad = plan.s("java_path"))
        } else {
            crate::sprache::t!("upd-java-fehlt", version = java.clone(), hinweis = java_hint(&java))
        });
    }
    lines.extend(
        plan.a("notes")
            .iter()
            .filter_map(|n| n.as_str().map(String::from)),
    );
    if plan.a("mods").iter().any(|m| m.s("status") == "manual") {
        lines.push(crate::sprache::t!("upd-manuell-hinweis"));
    }
    if !plan.b("space_ok") {
        lines.push(crate::sprache::t!("upd-platz", frei = fmt_bytes(plan.f("free_space")), server = fmt_bytes(plan.f("server_size"))));
    }
    lines.join("\n")
}

#[derive(Clone, Copy)]
struct Upd {
    info: RwSignal<Value>,
    targets: RwSignal<Vec<(String, String)>>,
    target: RwSignal<String>,
    plan: RwSignal<Option<(Value, bool)>>,
    disable_missing: RwSignal<bool>,
    disable_unknown: RwSignal<bool>,
}

fn refresh(u: Upd) {
    leptos::task::spawn_local(async move {
        match tauri::call::<Value>("upd_info").await {
            Ok(i) => {
                let v = i.s("info/version");
                u.targets.set(if v.is_empty() {
                    vec![]
                } else {
                    vec![(v.clone(), crate::sprache::t!("upd-nur-mods", version = v.clone()))]
                });
                u.target.set(v);
                u.plan.set(None);
                u.info.set(i);
            }
            Err(e) => u.info.set(json!({ "error": e })),
        }
    });
}

fn options(u: Upd) -> Value {
    json!({ "disableMissing": u.disable_missing.get_untracked(), "disableUnknown": u.disable_unknown.get_untracked() })
}

fn rollback(app: AppState, u: Upd, ask_first: bool) {
    leptos::task::spawn_local(async move {
        if ask_first && !confirm(crate::sprache::t!("upd-rollback-frage"),
            crate::sprache::t!("upd-rollback-text"), crate::sprache::t!("upd-zuruecksetzen")).await {
            return;
        }
        match task("upd_rollback", json!({})).await {
            Ok(r) => {
                app.add_event(crate::sprache::t!("upd-ev-rollback"));
                let aside = r.s("aside");
                info(crate::sprache::t!("upd-rollback-fertig-titel"), format!("{}{}", crate::sprache::t!("upd-rollback-fertig", label = r.s("label")),
                    if aside.is_empty() { String::new() } else { format!("\n\n{}\n{aside}", crate::sprache::t!("upd-rollback-daneben")) })).await;
            }
            Err(e) => error(crate::sprache::t!("upd-err-rollback"), e).await,
        }
        crate::pages::console::reset_log();
        refresh(u);
    });
}

fn run_update(app: AppState, u: Upd) {
    let Some((plan, _)) = u.plan.get_untracked() else {
        return;
    };
    leptos::task::spawn_local(async move {
        let what = if plan.b("change_version") {
            format!("Minecraft {} → {}", plan.s("current"), plan.s("target"))
        } else {
            crate::sprache::t!("upd-was-mods")
        };
        let n = plan
            .a("mods")
            .iter()
            .filter(|m| m.s("status") == "update")
            .count()
            + plan
                .a("extra")
                .iter()
                .filter(|e| e.s("status") == "new")
                .count()
            + plan
                .a("datapacks")
                .iter()
                .filter(|d| d.s("status") == "update")
                .count();
        let mut msg = crate::sprache::t!("upd-ablauf", anzahl = n as u32);
        let missing = plan
            .a("mods")
            .iter()
            .chain(plan.a("datapacks").iter())
            .filter(|m| m.s("status") == "missing")
            .count();
        if missing > 0 && u.disable_missing.get_untracked() {
            msg += &format!("\n\n{}", crate::sprache::t!("upd-werden-deaktiviert", anzahl = missing as u32));
        }
        msg += &format!("\n\n{}", crate::sprache::t!("upd-probelauf-tipp"));
        if !confirm(crate::sprache::t!("upd-start-frage", was = what.clone()), msg, crate::sprache::t!("upd-starten")).await {
            return;
        }
        app.add_event(crate::sprache::t!("upd-ev-gestartet", was = what.clone()));
        let r = match task("upd_run", options(u)).await {
            Ok(r) => r,
            Err(e) => {
                error(crate::sprache::t!("upd-err-update"), e).await;
                return refresh(u);
            }
        };
        refresh(u);
        crate::pages::console::reset_log();
        if r.b("ok") {
            app.add_event(crate::sprache::t!("upd-ev-fertig", was = r.s("what")));
            let c = r.o("cleaned");
            info(crate::sprache::t!("upd-fertig-titel"), format!("{}\n\n{}\n{}{}\n{}",
                crate::sprache::t!("upd-fertig-1", was = r.s("what")),
                crate::sprache::t!("upd-fertig-backup"), r.s("backup"),
                if c.is_null() { String::new() } else { format!("\n\n{}", crate::sprache::t!("upd-fertig-aufgeraeumt", anzahl = c.s("count"), groesse = fmt_bytes(c.f("freed")))) },
                crate::sprache::t!("upd-fertig-2"))).await;
        } else if confirm(crate::sprache::t!("upd-fehlgeschlagen-titel"), format!("{}\n\n{}", r.s("error"), crate::sprache::t!("upd-fehlgeschlagen-frage")), crate::sprache::t!("bak-wiederherstellen")).await {
            rollback(app, u, false);
        }
    });
}

fn trial(app: AppState, u: Upd) {
    leptos::task::spawn_local(async move {
        if !confirm(crate::sprache::t!("upd-trial-frage"),
            crate::sprache::t!("upd-trial-text"),
            crate::sprache::t!("upd-trial-starten")).await {
            return;
        }
        let r = match task("upd_trial", options(u)).await {
            Ok(r) => r,
            Err(e) => return error(crate::sprache::t!("upd-trial-err"), e).await,
        };
        let ok = r.b("ok");
        let mut parts = vec![if ok {
            format!("✓ {}", crate::sprache::t!("upd-trial-ok", sekunden = r.s("seconds"), port = r.s("port")))
        } else if r.b("timeout") {
            format!("✗ {}", crate::sprache::t!("upd-trial-timeout"))
        } else {
            format!("✗ {}", crate::sprache::t!("upd-trial-exit", code = if r.s("exit_code").is_empty() { "?".into() } else { r.s("exit_code") }))
        }];
        let strip = |l: &str| {
            l.split_once("]: ")
                .map(|(_, b)| b.to_string())
                .unwrap_or_else(|| l.to_string())
        };
        let info_lines: Vec<String> = [r.s("version_line"), r.s("mods_line")]
            .into_iter()
            .filter(|l| !l.is_empty())
            .map(|l| strip(&l))
            .collect();
        if !info_lines.is_empty() {
            parts.push(info_lines.join("\n"));
        }
        let problems: Vec<String> = r
            .a("problems")
            .iter()
            .filter_map(|p| p.as_str())
            .map(|l| {
                format!(
                    "• {}",
                    if l.starts_with('[') {
                        l.split_once("] ").map(|(_, b)| b).unwrap_or(l)
                    } else {
                        l
                    }
                )
            })
            .collect();
        if !problems.is_empty() {
            parts.push(format!("{}\n{}", crate::sprache::t!("upd-trial-meldungen"), problems.join("\n")));
        }
        let tail: Vec<String> = r
            .a("tail")
            .iter()
            .filter_map(|p| p.as_str().map(String::from))
            .collect();
        if !tail.is_empty() {
            parts.push(format!("{}\n{}", crate::sprache::t!("upd-trial-zeilen"), tail.join("\n")));
        }
        parts.push(if r.b("copy_deleted") {
            crate::sprache::t!("upd-trial-geloescht")
        } else {
            crate::sprache::t!("upd-trial-nicht-geloescht", ordner = r.s("copy"))
        });
        app.add_event(if ok {
            crate::sprache::t!("upd-ev-trial-ok")
        } else {
            crate::sprache::t!("upd-ev-trial-fehler")
        });
        ask(
            if ok {
                crate::sprache::t!("upd-trial-ok-titel")
            } else {
                crate::sprache::t!("upd-trial-err")
            },
            parts.join("\n\n"),
            vec![Btn::primary(crate::sprache::t!("gem-ok"), "ok")],
        )
        .await;
    });
}

#[component]
pub fn Updates() -> impl IntoView {
    let app = use_app();
    let u = Upd {
        info: RwSignal::new(Value::Null),
        targets: RwSignal::new(vec![]),
        target: RwSignal::new(String::new()),
        plan: RwSignal::new(None),
        disable_missing: RwSignal::new(true),
        disable_unknown: RwSignal::new(false),
    };
    Effect::new(move |_| {
        app.server_path.track();
        if app.page.get() == Page::Updates && u.plan.get_untracked().is_none() {
            refresh(u);
        }
    });
    let kind = move || u.info.get().s("info/type");
    let supported = move || matches!(kind().as_str(), "fabric" | "paper" | "neoforge" | "vanilla");
    let current = move || {
        let i = u.info.get();
        if !i.s("error").is_empty() {
            return i.s("error");
        }
        let t = i.s("info/type");
        let l = i.s("info/loader");
        let loader = if l.is_empty() {
            String::new()
        } else {
            match t.as_str() {
                "paper" => format!(" · Paper Build {l}"),
                "neoforge" => format!(" · NeoForge {l}"),
                "fabric" => format!(" · Fabric {l}"),
                _ => String::new(),
            }
        };
        let mods = i
            .f("mods")
            .filter(|m| *m > 0.0)
            .map(|m| {
                format!(
                    " · {m} {}",
                    if i.s("mod_dir") == "plugins" {
                        "Plugins"
                    } else {
                        "Mods"
                    }
                )
            })
            .unwrap_or_default();
        let packs = i
            .f("datapacks")
            .filter(|m| *m > 0.0)
            .map(|m| format!(" · {m} Datapacks"))
            .unwrap_or_default();
        let v = i.s("info/version");
        format!(
            "Minecraft {}{loader}{mods}{packs}  ({t})",
            if v.is_empty() { "?".into() } else { v }
        )
    };
    let load_versions = move |_| {
        leptos::task::spawn_local(async move {
            match tauri::call::<Vec<String>>("upd_versions").await {
                Ok(vs) => {
                    let cur = u.info.get_untracked().s("info/version");
                    let mut t: Vec<(String, String)> =
                        vs.iter().map(|v| (v.clone(), v.clone())).collect();
                    if !cur.is_empty() {
                        t.push((
                            cur.clone(),
                            crate::sprache::t!("upd-nur-mods", version = cur.clone()),
                        ));
                    }
                    if let Some(first) = t.first() {
                        u.target.set(first.0.clone());
                    }
                    u.targets.set(t);
                    u.plan.set(None);
                    if vs.is_empty() {
                        u.plan.set(Some((
                            json!({"_msg": crate::sprache::t!("upd-keine-neuere")}),
                            false,
                        )));
                    }
                }
                Err(e) => error(crate::sprache::t!("upd-keine-verbindung"), e).await,
            }
        });
    };
    let check = move |_| {
        let target = u.target.get_untracked();
        if target.is_empty() {
            return;
        }
        u.plan.set(None);
        leptos::task::spawn_local(async move {
            match task("upd_check", json!({ "target": target })).await {
                Ok(r) => u.plan.set(Some((r.o("plan"), r.b("has_work")))),
                Err(e) => error(crate::sprache::t!("upd-err-pruefen"), e).await,
            }
        });
    };
    let rows = move || {
        let Some((plan, _)) = u.plan.get() else {
            return vec![];
        };
        let plugins = plan.s("mod_dir") == "plugins";
        plan.a("mods")
            .into_iter()
            .chain(plan.a("extra"))
            .chain(plan.a("datapacks"))
            .map(|m| {
                let name = [m.s("title"), m.s("file"), m.s("project_id")]
                    .into_iter()
                    .find(|s| !s.is_empty())
                    .unwrap_or_else(|| "?".into());
                let title = format!(
                    "{}{name}",
                    if m.s("kind") == "datapack" {
                        "Datapack: "
                    } else if plugins {
                        "Plugin: "
                    } else {
                        ""
                    }
                );
                let cur = if !m.s("current_version").is_empty() {
                    m.s("current_version")
                } else if m.s("status") == "new" {
                    "–".into()
                } else {
                    m.s("file")
                };
                (
                    format!("st-{}", m.s("status")),
                    status_text(&m),
                    title,
                    cur,
                    m.s("new_version"),
                )
            })
            .collect::<Vec<_>>()
    };
    let summary_text = move || {
        u.plan
            .get()
            .map(|(p, w)| {
                if !p.s("_msg").is_empty() {
                    p.s("_msg")
                } else {
                    summary(&p, w)
                }
            })
            .unwrap_or_else(|| {
                if supported() || u.info.get().is_null() {
                    String::new()
                } else {
                    crate::sprache::t!("upd-nicht-unterstuetzt")
                }
            })
    };
    let unknown = move || {
        u.plan.get().is_some_and(|(p, _)| {
            p.a("mods")
                .iter()
                .chain(p.a("datapacks").iter())
                .any(|m| m.s("status") == "unknown")
        })
    };
    let ready = move || {
        u.plan
            .get()
            .is_some_and(|(p, w)| w && p.b("java_ok") && p.b("space_ok"))
    };
    let mrpack_ok = move || matches!(kind().as_str(), "fabric" | "neoforge");
    let export = move |_| {
        leptos::task::spawn_local(async move {
            match tauri::call::<Option<Value>>("export_mrpack").await {
                Ok(Some(r)) => info(crate::sprache::t!("upd-export-titel"), format!("{}\n\n{}{}\n{}",
                    r.s("path"), crate::sprache::t!("upd-export-mods", anzahl = r.s("files")),
                    r.f("overrides").filter(|o| *o > 0.0).map(|o| format!(", {}", crate::sprache::t!("upd-export-eingepackt", anzahl = o))).unwrap_or_default(),
                    crate::sprache::t!("upd-export-hinweis"))).await,
                Ok(None) => {}
                Err(e) => error(crate::sprache::t!("upd-err-export"), e).await,
            }
        });
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Updates id="tab-updates">
            <p><b>{crate::sprache::t!("upd-jetzt")}" "</b>{current}</p>
            <p class="dim small">{crate::sprache::t!("upd-erklaerung")}</p>
            <div class="row">
                <label>{crate::sprache::t!("upd-ziel")}</label>
                <select disabled=move || !supported() prop:value=move || u.target.get()
                    on:change=move |e| { u.target.set(event_target_value(&e)); u.plan.set(None); }>
                    {move || u.targets.get().into_iter().map(|(v, l)| view! { <option value=v>{l}</option> }).collect_view()}
                </select>
                <button disabled=move || !supported() on:click=load_versions>{crate::sprache::t!("upd-versionen")}</button>
                <button class="primary" disabled=move || !supported() on:click=check>{crate::sprache::t!("upd-pruefen")}</button>
            </div>
            <div class="table-wrap"><table id="upd-table">
                <thead><tr><th>"Status"</th><th>{crate::sprache::t!("upd-spalte-mod")}</th><th>{crate::sprache::t!("kat-installiert")}</th><th>{crate::sprache::t!("upd-spalte-neu")}</th></tr></thead>
                <tbody>{move || rows().into_iter().map(|(cls, st, t, cur, new)| view! {
                    <tr><td class=cls>{st}</td><td>{t}</td><td>{cur}</td><td>{new}</td></tr>
                }).collect_view()}</tbody>
            </table></div>
            <div class="summary">{summary_text}</div>
            <label class="check"><input type="checkbox" prop:checked=u.disable_missing on:change=move |e| u.disable_missing.set(event_target_checked(&e))/>
                " "{crate::sprache::t!("upd-deaktivieren-fehlende")}</label>
            <Show when=unknown>
                <label class="check"><input type="checkbox" prop:checked=u.disable_unknown on:change=move |e| u.disable_unknown.set(event_target_checked(&e))/>
                    " "{crate::sprache::t!("upd-deaktivieren-unbekannte")}</label>
            </Show>
            <div class="row">
                <button disabled=move || !ready() title=crate::sprache::t!("upd-trial-tipp") on:click=move |_| trial(app, u)>{crate::sprache::t!("upd-trial")}</button>
                <button class="primary" disabled=move || !ready() on:click=move |_| run_update(app, u)>{crate::sprache::t!("upd-starten")}</button>
                <Show when=move || u.info.get().b("rollback")>
                    <button on:click=move |_| rollback(app, u, true)>{crate::sprache::t!("upd-rollback-knopf")}</button>
                </Show>
                <span class="spacer"></span>
                <button disabled=move || !mrpack_ok() title=move || if mrpack_ok() { String::new() } else { crate::sprache::t!("upd-export-tipp") }
                    on:click=export>{crate::sprache::t!("upd-export-knopf")}</button>
            </div>
        </section>
    }
}
