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
        return "✓ passt wieder – wird eingeschaltet".into();
    }
    match st.as_str() {
        "update" => "✓ Update",
        "current" => "= aktuell",
        "missing" => "✗ fehlt",
        "unknown" => "? unbekannt",
        "new" => "+ neu (wird gebraucht)",
        "manual" => "↗ Update nur im Browser (CurseForge)",
        "reenable" => "✓ passt wieder – wird eingeschaltet",
        "duplicate" => "⇄ doppelt (ist schon als Mod da)",
        _ => return st,
    }
    .into()
}

fn java_hint(n: &str) -> String {
    let windows = web_sys::window()
        .and_then(|w| w.navigator().user_agent().ok())
        .is_some_and(|u| u.contains("Windows"));
    if windows {
        format!("Installieren z. B. von adoptium.net (Temurin {n}).")
    } else {
        format!("Installieren mit: sudo apt install openjdk-{n}-jre-headless")
    }
}

/// Zusammenfassung unter der Tabelle.
fn summary(plan: &Value, has_work: bool) -> String {
    if !has_work {
        return "Alles ist auf dem neuesten Stand.".into();
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
    let mut first = format!(
        "{} Updates, {} schon aktuell, {} ohne passende Version, {} unbekannt",
        count("update"),
        count("current"),
        count("missing"),
        count("unknown")
    );
    if new_deps > 0 {
        first += &format!(", {new_deps} werden neu gebraucht");
    }
    if count("reenable") > 0 {
        first += &format!(", {} passen wieder", count("reenable"));
    }
    if count("duplicate") > 0 {
        first += &format!(", {} doppelt (wird beiseitegelegt)", count("duplicate"));
    }
    let mut lines = vec![first];
    let server = plan.o("server");
    let change = plan.b("change_version");
    if !server.is_null() && !change {
        lines.push(format!(
            "Neuer Server-Stand für Minecraft {}: {}.",
            plan.s("target"),
            server.s("label")
        ));
    }
    if server.b("beta") {
        lines.push("Hinweis: Dafür gibt es bisher nur eine Test-/Beta-Version.".into());
    }
    if change {
        lines.push(format!(
            "Server: Minecraft {} → {} ({}).",
            plan.s("current"),
            plan.s("target"),
            if server.is_null() {
                "?".into()
            } else {
                server.s("label")
            }
        ));
        lines.push("Achtung: Eine Welt, die einmal mit der neuen Version lief, kann man nicht zurückstufen. Zurück geht nur über das Backup, das automatisch gemacht wird.".into());
    }
    let java = plan.s("java_required");
    if !java.is_empty() {
        lines.push(if plan.b("java_ok") {
            format!("Java {java} nötig – vorhanden ({}).", plan.s("java_path"))
        } else {
            format!(
                "Java {java} nötig, aber nicht installiert. {}",
                java_hint(&java)
            )
        });
    }
    lines.extend(
        plan.a("notes")
            .iter()
            .filter_map(|n| n.as_str().map(String::from)),
    );
    if plan.a("mods").iter().any(|m| m.s("status") == "manual") {
        lines.push("Für Mods mit „Update nur im Browser“ erlaubt der Autor kein automatisches Laden – neue Datei auf CurseForge herunterladen und selbst austauschen.".into());
    }
    if !plan.b("space_ok") {
        lines.push(format!(
            "Zu wenig Speicherplatz für das Backup ({} frei, Server {}).",
            fmt_bytes(plan.f("free_space")),
            fmt_bytes(plan.f("server_size"))
        ));
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
                    vec![(v.clone(), format!("{v} (nur Mods/Plugins aktualisieren)"))]
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
        if ask_first && !confirm("Zurück zum Stand vor dem letzten Update?",
            "Der Server wird gestoppt und aus dem Backup wiederhergestellt, das beim Update gemacht wurde. Der jetzige Stand bleibt als Ordner „…_vor_Wiederherstellung_<Datum>“ daneben liegen.", "Zurücksetzen").await {
            return;
        }
        match task("upd_rollback", json!({})).await {
            Ok(r) => {
                app.add_event("Stand vor dem Update wiederhergestellt");
                let aside = r.s("aside");
                info("Zurückgesetzt", format!("Der Server ist wieder auf dem Stand von {}. Du kannst ihn jetzt starten.{}", r.s("label"),
                    if aside.is_empty() { String::new() } else { format!("\n\nDer Stand nach dem Update liegt in:\n{aside}") })).await;
            }
            Err(e) => error("Zurücksetzen fehlgeschlagen", e).await,
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
            "Mods aktualisieren".into()
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
        let mut msg = format!(
            "Ablauf: Server stoppen (mit Warnung, falls jemand online ist) → komplettes Backup → {n} Dateien laden und prüfen → tauschen → Server starten und prüfen."
        );
        let missing = plan
            .a("mods")
            .iter()
            .chain(plan.a("datapacks").iter())
            .filter(|m| m.s("status") == "missing")
            .count();
        if missing > 0 && u.disable_missing.get_untracked() {
            msg += &format!(
                "\n\n{missing} Mod(s)/Datapack(s) ohne passende Version werden deaktiviert."
            );
        }
        msg += "\n\nTipp: Mit „Probelauf“ kannst du das vorher gefahrlos an einer Kopie testen.";
        if !confirm(format!("Update starten? ({what})"), msg, "Update starten").await {
            return;
        }
        app.add_event(format!("Update gestartet ({what})"));
        let r = match task("upd_run", options(u)).await {
            Ok(r) => r,
            Err(e) => {
                error("Update fehlgeschlagen", e).await;
                return refresh(u);
            }
        };
        refresh(u);
        crate::pages::console::reset_log();
        if r.b("ok") {
            app.add_event(format!("Update fertig ({})", r.s("what")));
            let c = r.o("cleaned");
            info("Update fertig", format!("Der Server läuft wieder ({}).\n\nBackup vom alten Stand:\n{}{}\nMods ohne passende Version sind ausgeschaltet und unter „Mods/Plugins“ zu sehen.",
                r.s("what"), r.s("backup"),
                if c.is_null() { String::new() } else { format!("\n\nAufgeräumt: {} alte Dateien/Ordner gelöscht ({}).", c.s("count"), fmt_bytes(c.f("freed"))) })).await;
        } else if confirm("Update hat nicht geklappt", format!("{}\n\nSchau in der Konsole nach der Fehlermeldung. Soll ich den Stand vor dem Update aus dem Backup wiederherstellen?", r.s("error")), "Wiederherstellen").await {
            rollback(app, u, false);
        }
    });
}

fn trial(app: AppState, u: Upd) {
    leptos::task::spawn_local(async move {
        if !confirm("Probelauf starten?",
            "Ich kopiere den Server in den Temp-Ordner, aktualisiere die Kopie, starte sie auf einem anderen Port und schaue, ob sie fertig hochfährt. Danach wird die Kopie gestoppt und gelöscht.\n\nDein echter Server bleibt dabei unverändert und kann weiterlaufen. Bei großen Welten dauert das Kopieren ein paar Minuten.",
            "Probelauf starten").await {
            return;
        }
        let r = match task("upd_trial", options(u)).await {
            Ok(r) => r,
            Err(e) => return error("Probelauf fehlgeschlagen", e).await,
        };
        let ok = r.b("ok");
        let mut parts = vec![if ok {
            format!(
                "✓ Die aktualisierte Kopie ist in {} Sekunden fertig gestartet (Port {}).",
                r.s("seconds"),
                r.s("port")
            )
        } else if r.b("timeout") {
            "✗ Die Kopie war nach 15 Minuten noch nicht fertig gestartet.".into()
        } else {
            format!(
                "✗ Die Kopie hat sich beim Start beendet (Exit-Code {}).",
                if r.s("exit_code").is_empty() {
                    "?".into()
                } else {
                    r.s("exit_code")
                }
            )
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
            parts.push(format!("Auffällige Meldungen:\n{}", problems.join("\n")));
        }
        let tail: Vec<String> = r
            .a("tail")
            .iter()
            .filter_map(|p| p.as_str().map(String::from))
            .collect();
        if !tail.is_empty() {
            parts.push(format!("Letzte Zeilen:\n{}", tail.join("\n")));
        }
        parts.push(if r.b("copy_deleted") {
            "Die Kopie wurde wieder gelöscht.".into()
        } else {
            format!(
                "Die Kopie konnte nicht ganz gelöscht werden: {}",
                r.s("copy")
            )
        });
        app.add_event(format!(
            "Probelauf {}",
            if ok { "erfolgreich" } else { "fehlgeschlagen" }
        ));
        ask(
            if ok {
                "Probelauf erfolgreich"
            } else {
                "Probelauf fehlgeschlagen"
            },
            parts.join("\n\n"),
            vec![Btn::primary("OK", "ok")],
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
                            format!("{cur} (nur Mods/Plugins aktualisieren)"),
                        ));
                    }
                    if let Some(first) = t.first() {
                        u.target.set(first.0.clone());
                    }
                    u.targets.set(t);
                    u.plan.set(None);
                    if vs.is_empty() {
                        u.plan.set(Some((
                            json!({"_msg": "Es gibt keine neuere stabile Minecraft-Version."}),
                            false,
                        )));
                    }
                }
                Err(e) => error("Keine Verbindung", e).await,
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
                Err(e) => error("Prüfen fehlgeschlagen", e).await,
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
                    "Den Update-Assistenten gibt es für Fabric, Paper, NeoForge und Vanilla.".into()
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
                Ok(Some(r)) => info("Modpack gespeichert", format!("{}\n\n{} Mods von Modrinth{}.\nDeine Freunde importieren die Datei in der Modrinth-App („Datei hinzufügen“). Reine Server-Mods werden dort nicht installiert.",
                    r.s("path"), r.s("files"), r.f("overrides").filter(|o| *o > 0.0).map(|o| format!(", {o} direkt eingepackt")).unwrap_or_default())).await,
                Ok(None) => {}
                Err(e) => error("Export fehlgeschlagen", e).await,
            }
        });
    };
    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Updates id="tab-updates">
            <p><b>"Jetzt: "</b>{current}</p>
            <p class="dim small">"Prüft, ob es alle Mods für die gewählte Version gibt. Vor jedem Update wird automatisch ein Backup gemacht."</p>
            <div class="row">
                <label>"Ziel:"</label>
                <select disabled=move || !supported() prop:value=move || u.target.get()
                    on:change=move |e| { u.target.set(event_target_value(&e)); u.plan.set(None); }>
                    {move || u.targets.get().into_iter().map(|(v, l)| view! { <option value=v>{l}</option> }).collect_view()}
                </select>
                <button disabled=move || !supported() on:click=load_versions>"Versionen laden"</button>
                <button class="primary" disabled=move || !supported() on:click=check>"Prüfen"</button>
            </div>
            <div class="table-wrap"><table id="upd-table">
                <thead><tr><th>"Status"</th><th>"Mod / Datapack"</th><th>"Installiert"</th><th>"Neu"</th></tr></thead>
                <tbody>{move || rows().into_iter().map(|(cls, st, t, cur, new)| view! {
                    <tr><td class=cls>{st}</td><td>{t}</td><td>{cur}</td><td>{new}</td></tr>
                }).collect_view()}</tbody>
            </table></div>
            <div class="summary">{summary_text}</div>
            <label class="check"><input type="checkbox" prop:checked=u.disable_missing on:change=move |e| u.disable_missing.set(event_target_checked(&e))/>
                " Mods/Datapacks ohne passende Version deaktivieren (sonst startet der Server wahrscheinlich nicht)"</label>
            <Show when=unknown>
                <label class="check"><input type="checkbox" prop:checked=u.disable_unknown on:change=move |e| u.disable_unknown.set(event_target_checked(&e))/>
                    " Unbekannte Mods/Datapacks (nicht auf Modrinth) deaktivieren"</label>
            </Show>
            <div class="row">
                <button disabled=move || !ready() title="Aktualisiert eine Kopie im Temp-Ordner und startet sie auf einem anderen Port" on:click=move |_| trial(app, u)>"Probelauf"</button>
                <button class="primary" disabled=move || !ready() on:click=move |_| run_update(app, u)>"Update starten"</button>
                <Show when=move || u.info.get().b("rollback")>
                    <button on:click=move |_| rollback(app, u, true)>"Zurück zum Stand vor dem letzten Update"</button>
                </Show>
                <span class="spacer"></span>
                <button disabled=move || !mrpack_ok() title=move || if mrpack_ok() { "" } else { "Nur für Fabric- und NeoForge-Server (Mods, die auch die Freunde brauchen)" }
                    on:click=export>"Mods für Freunde exportieren (.mrpack)"</button>
            </div>
        </section>
    }
}
