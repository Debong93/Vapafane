//! Einstellungen (Server-Bereich): server.properties, RAM, Automatik, CurseForge, Aussehen, Ordner, Über.

use leptos::prelude::*;
use serde_json::{Map, Value, json};

use crate::dialog::{confirm, error, show_error};
use crate::pages::{About, LanguageChoice, SkinChoice};
use crate::state::{AppState, Page, use_app};
use crate::tauri;
use crate::util::{J, open_folder, open_link};

/// Felder aus server.properties: (Schlüssel, Beschriftung, Art)
const PROPS: [(&str, &str, &str); 9] = [
    ("motd", "Servername (MOTD)", "text"),
    ("max-players", "Max. Spieler", "num:1:200"),
    (
        "difficulty",
        "Schwierigkeit",
        "sel:peaceful=Friedlich,easy=Einfach,normal=Normal,hard=Schwer",
    ),
    (
        "gamemode",
        "Spielmodus für neue Spieler",
        "sel:survival=Überleben,creative=Kreativ,adventure=Abenteuer,spectator=Zuschauer",
    ),
    ("view-distance", "Sichtweite (Chunks)", "num:3:32"),
    (
        "simulation-distance",
        "Simulationsweite (Chunks)",
        "num:3:32",
    ),
    ("spawn-protection", "Spawn-Schutz (Blöcke)", "num:0:64"),
    ("server-port", "Server-Port", "num:1024:65535"),
    ("white-list", "Nur Whitelist darf rein", "bool"),
];

fn load(s: RwSignal<Value>, props: RwSignal<Map<String, Value>>, ram: RwSignal<String>) {
    leptos::task::spawn_local(async move {
        if let Ok(v) = tauri::call::<Value>("get_settings").await {
            props.set(v.o("props").as_object().cloned().unwrap_or_default());
            ram.set(
                v.f("ram_gb")
                    .map(|g| g.to_string())
                    .unwrap_or_else(|| "4".into()),
            );
            s.set(v);
        }
    });
}

async fn ask_linger(
    s: RwSignal<Value>,
    props: RwSignal<Map<String, Value>>,
    ram: RwSignal<String>,
) {
    if !confirm("Start ohne Anmeldung erlauben?",
        "Dafür muss systemd deine Benutzer-Dienste schon beim Hochfahren starten dürfen („loginctl enable-linger“). Danach läuft der Server auch, wenn sich niemand anmeldet.", "Erlauben").await {
        return;
    }
    if let Err(e) = tauri::call::<()>("enable_linger").await {
        error("Nicht erlaubt", e).await;
    }
    load(s, props, ram);
}

#[component]
pub fn Settings() -> impl IntoView {
    let app = use_app();
    let s = RwSignal::new(Value::Null);
    let props = RwSignal::new(Map::new());
    let ram = RwSignal::new(String::new());
    let saved = RwSignal::new(String::new());
    Effect::new(move |_| {
        app.server_path.track();
        if app.page.get() == Page::Settings {
            load(s, props, ram);
        }
    });
    let has_props = move || s.get().get("props").is_some_and(|p| p.is_object());

    let prop_field = move |(key, label, kind): (&'static str, &'static str, &'static str)| {
        let val = move || {
            props
                .get()
                .get(key)
                .map(|v| v.as_str().unwrap_or("").to_string())
                .unwrap_or_default()
        };
        let set = move |v: String| {
            props.update(|p| {
                p.insert(key.into(), Value::String(v));
            })
        };
        let field = if kind == "bool" {
            view! { <label class="switch"><input type="checkbox" disabled=move || !has_props() prop:checked=move || val() == "true"
                on:change=move |e| set(event_target_checked(&e).to_string())/><span></span></label> }.into_any()
        } else if let Some(opts) = kind.strip_prefix("sel:") {
            view! { <select disabled=move || !has_props() prop:value=val on:change=move |e| set(event_target_value(&e))>
                {opts.split(',').map(|o| { let (v, l) = o.split_once('=').unwrap_or((o, o)); view! { <option value=v>{l}</option> } }).collect_view()}
            </select> }.into_any()
        } else if let Some(r) = kind.strip_prefix("num:") {
            let (min, max) = r.split_once(':').unwrap_or(("0", "100"));
            view! { <input type="number" class="num" min=min max=max disabled=move || !has_props() prop:value=val on:input=move |e| set(event_target_value(&e))/> }.into_any()
        } else {
            view! { <input disabled=move || !has_props() prop:value=val on:input=move |e| set(event_target_value(&e))/> }.into_any()
        };
        let help = (key == "white-list").then(|| view! {
            <span></span><span class="dim small">"Empfohlen für die Sicherheit: Nur Spieler auf der Whitelist (Seite „Spieler“) kommen rein. Ohne Whitelist kann jeder beitreten, der deine Adresse kennt."</span>
        });
        view! { <label>{label}</label>{field}{help} }
    };

    let save = move |_| {
        let p: Map<String, Value> = props
            .get_untracked()
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Value::String(v.as_str().unwrap_or("").trim().to_string()),
                )
            })
            .collect();
        let gb = ram.get_untracked().parse::<u32>().unwrap_or(4);
        leptos::task::spawn_local(async move {
            match tauri::invoke::<()>("save_settings", json!({ "props": p, "ramGb": gb })).await {
                Ok(()) => saved.set(
                    if matches!(
                        app.status.get_untracked().state.as_str(),
                        "running" | "starting"
                    ) {
                        "Gespeichert. Wird beim nächsten Neustart aktiv.".into()
                    } else {
                        "Gespeichert.".into()
                    },
                ),
                Err(e) => error("Speichern fehlgeschlagen", e).await,
            }
            app.refresh_details();
        });
    };
    let total_gb = move || (s.get().f("total_memory").unwrap_or(0.0) / 1024f64.powi(3)).round();

    // Schalter: setzt im Backend, lädt danach neu
    let switch = move |key: &'static str, cmd: &'static str| {
        let busy = RwSignal::new(false);
        view! {
            <label class="switch"><input type="checkbox" prop:checked=move || s.get().b(key) disabled=move || busy.get() || (key == "autostart" && !auto_ok(s))
                on:change=move |e| {
                    let on = event_target_checked(&e);
                    busy.set(true);
                    leptos::task::spawn_local(async move {
                        match tauri::invoke::<()>(cmd, json!({ "on": on })).await {
                            Ok(()) => if cmd == "set_autostart" && on && s.get_untracked().b("systemd") && !s.get_untracked().b("linger") { ask_linger(s, props, ram).await },
                            Err(er) => error("Einstellung", er).await,
                        }
                        busy.set(false);
                        load(s, props, ram);
                    });
                }/><span></span></label>
        }
    };
    let autostart_sub = move || {
        let v = s.get();
        if !auto_ok(s) {
            if v.b("test_mode") {
                "Im Testmodus abgeschaltet."
            } else {
                "Auf diesem System nicht verfügbar (kein systemd)."
            }
        } else if v.b("windows") {
            "Startet beim PC-Start, auch ohne Anmeldung (Windows fragt beim Einschalten nach Administratorrechten)."
        } else if v.b("autostart") && !v.b("linger") {
            "Achtung: startet erst, wenn du dich anmeldest. Klick hier, um den Start ohne Anmeldung zu erlauben."
        } else {
            "Startet auch ohne Anmeldung und nach einem Absturz von selbst neu."
        }
    };
    let linger_click = move |_| {
        let v = s.get_untracked();
        if v.b("autostart") && !v.b("linger") && v.b("systemd") {
            leptos::task::spawn_local(async move { ask_linger(s, props, ram).await });
        }
    };
    let set_time = move |e: leptos::ev::Event| {
        let t = event_target_value(&e);
        leptos::task::spawn_local(async move {
            if let Err(er) = tauri::invoke::<String>("set_daily_time", json!({ "time": t })).await {
                error("Uhrzeit", er).await;
            }
            load(s, props, ram);
        });
    };

    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Settings>
            <LanguageChoice/>
            <h3>"Spiel (dieser Server)"</h3>
            <div class="form">{PROPS.into_iter().map(prop_field).collect_view()}</div>
            <h3>"Leistung (dieser Server)"</h3>
            <div class="form">
                <label>"Max. Arbeitsspeicher für den Server (GB)"</label>
                <input type="number" min="1" max=move || total_gb().max(2.0).to_string() class="num" prop:value=ram on:input=move |e| ram.set(event_target_value(&e))/>
                <span></span><span class="dim small">{move || format!("Der PC hat {} GB. Lass etwa 3–4 GB für das System übrig.", total_gb())}</span>
            </div>
            <div class="row"><button class="primary" on:click=save>"Einstellungen speichern"</button><span class="dim">{saved}</span></div>

            <h3>"Automatik"</h3>
            <div class="switches">
                <div class="sw-row"><div><div>"Aktiven Server beim PC-Start automatisch starten"</div>
                    <div class="dim small" style:cursor=move || if s.get().b("autostart") && !s.get().b("linger") && s.get().b("systemd") { "pointer" } else { "" }
                        on:click=linger_click>{autostart_sub}</div></div>{switch("autostart", "set_autostart")}</div>
                <div class="sw-row"><div><div>"Dieses Fenster beim Anmelden öffnen"</div></div>{switch("window", "set_window_autostart")}</div>
                <div class="sw-row"><div><div>"Tägliches Backup um "
                    <input type="time" class="time" aria-label="Uhrzeit für das tägliche Backup" prop:value=move || { let t = s.get().s("daily_time"); if t.is_empty() { "05:00".into() } else { t } } on:change=set_time/>
                    " Uhr (ganzer Server als ZIP)"</div><div class="dim small">"Macht der Hintergrund-Dienst. Holt es nach, falls der PC zu der Zeit aus war."</div></div>{switch("daily", "set_daily_backup")}</div>
                <div class="sw-row"><div><div>"Erreichbarkeit von außen prüfen"</div><div class="dim small">"Beim Wechsel in den Server-Bereich. Fragt dafür einen Prüfdienst im Internet."</div></div>{switch("reach_check", "set_reach_check")}</div>
                <div class="sw-row"><div><div>"Benachrichtigungen"</div><div class="dim small">"Wenn jemand beitritt oder geht und wenn der Server abstürzt (solange das Fenster offen ist). Ohne Ton."</div></div>{switch("notify", "set_notify")}</div>
            </div>
            {move || old_panel(app, s, props, ram)}
            <CurseForge/>
            <SkinChoice/>
            <h3>"Ordner"</h3>
            <div class="row">
                <button on:click=|_| open_folder("server")>"Server-Ordner"</button>
                <button on:click=|_| open_folder("mods")>"Mods"</button>
                <button on:click=|_| open_folder("backups")>"Backups"</button>
                <button on:click=|_| open_folder("logs")>"Logs"</button>
            </div>
            <About/>
        </section>
    }
}

fn auto_ok(s: RwSignal<Value>) -> bool {
    let v = s.get();
    v.b("systemd") || v.b("windows")
}

fn old_panel(
    app: AppState,
    s: RwSignal<Value>,
    props: RwSignal<Map<String, Value>>,
    ram: RwSignal<String>,
) -> Option<AnyView> {
    let o = s.get().o("old_panel");
    if o.is_null() {
        return None;
    }
    let off = move |_| {
        leptos::task::spawn_local(async move {
            if !confirm("Altes Panel abschalten?",
                "Ich stoppe den Dienst „minecraft.service“ (der Server speichert dabei sauber und beendet sich) und schalte ihn und sein tägliches Backup beim PC-Start aus.\n\nDas alte Panel selbst und seine Dateien bleiben erhalten – man kann es jederzeit wieder einschalten. Danach startest du den Server hier mit „Starten“.",
                "Abschalten").await {
                return;
            }
            match tauri::call::<()>("disable_old_panel").await {
                Ok(()) => app.add_event("Altes Panel abgeschaltet"),
                Err(e) => error("Abschalten fehlgeschlagen", e).await,
            }
            load(s, props, ram);
            app.refresh_status();
        });
    };
    let a = |b: bool, y: &'static str, n: &'static str| if b { y } else { n };
    Some(view! {
        <div class="hint">
            <b>"Altes Bedienfenster noch eingerichtet."</b>
            {format!(" Dienst „minecraft.service“: {}, beim Start {}; tägliches Backup-Zeitprogramm {}. Solange es läuft, kann Vapafane den Server nicht selbst starten (gleicher Port), und Backups würden doppelt gemacht. ",
                a(o.b("active"), "läuft", "aus"), a(o.b("enabled"), "an", "aus"), a(o.b("timer"), "an", "aus"))}
            <div class="row"><button on:click=off>"Altes Panel abschalten …"</button></div>
        </div>
    }.into_any())
}

#[component]
fn CurseForge() -> impl IntoView {
    let app = use_app();
    let state = RwSignal::new(Value::Null);
    let msg = RwSignal::new(String::new());
    let key = RwSignal::new(String::new());
    let reload = move || {
        leptos::task::spawn_local(async move {
            let c = tauri::call::<Value>("cf_status")
                .await
                .unwrap_or(json!({"set": false}));
            msg.set(if c.b("set") {
                format!(
                    "Gespeicherter Schlüssel: {} – CurseForge wird bei der Suche mit gefragt.",
                    c.s("masked")
                )
            } else {
                "Kein Schlüssel gespeichert – gesucht wird nur auf Modrinth.".into()
            });
            state.set(c);
        });
    };
    reload();
    let test = move |_| {
        msg.set("Teste …".into());
        let k = key.get_untracked();
        leptos::task::spawn_local(async move {
            let arg = if k.is_empty() {
                Value::Null
            } else {
                Value::String(k)
            };
            msg.set(
                match tauri::invoke::<()>("cf_key_test", json!({ "key": arg })).await {
                    Ok(()) => "✓ Der Schlüssel funktioniert.".into(),
                    Err(e) => format!("✗ {e}"),
                },
            );
        });
    };
    let save = move |_| {
        let k = key.get_untracked();
        leptos::task::spawn_local(async move {
            match tauri::invoke::<()>("cf_key_save", json!({ "key": k })).await {
                Ok(()) => {
                    key.set(String::new());
                    app.add_event("CurseForge-Schlüssel gespeichert");
                }
                Err(e) => return error("Speichern", e).await,
            }
            reload();
        });
    };
    let delete = move |_| {
        leptos::task::spawn_local(async move {
            if !confirm(
                "CurseForge-Schlüssel entfernen?",
                "Danach sucht Vapafane wieder nur auf Modrinth.",
                "Entfernen",
            )
            .await
            {
                return;
            }
            if let Err(e) = tauri::call::<()>("cf_key_delete").await {
                show_error("Entfernen", e);
            }
            reload();
        });
    };
    view! {
        <h3>"CurseForge (optional) " <span class="badge exp">"Experimentell"</span></h3>
        <div class="cf-box">
            <p><b>"Brauche ich das?"</b>" Nein – die meisten Mods gibt es auf "<b>"Modrinth"</b>", das geht ohne Einrichtung. CurseForge findet "<i>"zusätzlich"</i>" Mods, die es nur dort gibt, und braucht dafür einen persönlichen API-Schlüssel (eine Art Zugangscode)."</p>
            <details>
                <summary><b>"So bekommst du einen Schlüssel (kostenlos)"</b></summary>
                <ol>
                    <li>"Auf "<b>"„Schlüssel beantragen“"</b>" klicken und das Formular ausfüllen (z. B. Zweck: „Mods für meinen eigenen Server“)."</li>
                    <li>"Auf die Antwort per E-Mail warten – CurseForge prüft von Hand, das kann ein paar Tage dauern."</li>
                    <li>"Den Schlüssel unten einfügen, "<b>"„Schlüssel testen“"</b>" und "<b>"„Speichern“"</b>"."</li>
                </ol>
                <p><b>"Der Schlüssel ist persönlich"</b>" – nicht weitergeben oder veröffentlichen. Er bleibt nur auf diesem PC. Es gelten die "
                    <a href="#" on:click=|e| { e.prevent_default(); open_link("cf-terms"); }>"CurseForge-Bedingungen für Drittprogramme"</a>
                    ". Experimentell: Ob CurseForge Schlüssel für private Tools vergibt, entscheidet CurseForge."</p>
            </details>
            <div class="row">
                <button on:click=|_| open_link("cf-apply")>"Schlüssel beantragen ↗"</button>
                <button on:click=|_| open_link("cf-console")>"CurseForge-Konsole ↗"</button>
                <button on:click=|_| open_link("cf-docs")>"Mehr zur CurseForge-API ↗"</button>
            </div>
            <div class="row">
                <input type="password" placeholder="API-Schlüssel hier einfügen" autocomplete="off" spellcheck="false" prop:value=key on:input=move |e| key.set(event_target_value(&e))/>
                <button on:click=test>"Schlüssel testen"</button>
                <button class="primary" on:click=save>"Speichern"</button>
                <button class="danger" disabled=move || !state.get().b("set") on:click=delete>"Entfernen"</button>
            </div>
            <p class="small">{msg}</p>
        </div>
    }
}

