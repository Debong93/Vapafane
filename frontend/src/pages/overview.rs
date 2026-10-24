//! Übersicht: Zustand, Starten/Stoppen, Karten mit Kurven, letzte Ereignisse.

use leptos::prelude::*;
use serde_json::{Value, json};
use std::time::Duration;
use vapafane_shared::Status;

use crate::dialog::{self, Btn, ask, confirm, error};
use crate::state::{AppState, Mode, Page, push_spark, reduced_motion, use_app};
use crate::tauri;
use crate::util::{J, animate, by_id, computed, css_var, fmt_bytes, fmt_duration, num};

/// Text und Farbe je Zustand.
pub fn label_of(state: &str) -> (&'static str, &'static str) {
    match state {
        "running" => ("Läuft", "--green"),
        "starting" => ("Startet …", "--amber"),
        "stopping" => ("Stoppt …", "--amber"),
        "restarting" => ("Neustart nach Absturz …", "--amber"),
        "stopped" => ("Gestoppt", "--gray"),
        "failed" => ("Abgestürzt", "--red"),
        "external" => ("Läuft außerhalb von Vapafane", "--amber"),
        "other" => ("Ein anderer Server läuft", "--amber"),
        "none" => ("Kein Server ausgewählt", "--gray"),
        _ => ("…", "--gray"),
    }
}

/// Wird bei jedem neuen Status aufgerufen (vor dem Speichern): Übergänge melden,
/// Animationen auslösen, Kurven füttern, bei Serverwechsel alles neu laden.
pub fn on_status(app: AppState, st: &Status) {
    let old = app.status.get_untracked();
    let (last, state) = (old.state.as_str(), st.state.as_str());
    let first = last.is_empty();

    // Anderer Server aktiv → alles zurücksetzen und neu laden
    let old_path = old.server.as_ref().map(|s| s.path.clone());
    let new_path = st.server.as_ref().map(|s| s.path.clone());
    if old_path != new_path {
        for s in [app.spark_tps, app.spark_cpu, app.spark_ram] {
            s.set(vec![]);
        }
        app.live.set(Value::Null);
        crate::pages::console::reset_log();
        app.refresh_details();
        app.backups_tick.update(|n| *n += 1);
        if !first && app.mode.get_untracked() == Mode::Server {
            app.reach_checked.set(false);
            crate::pages::reach::reach_init(app);
        }
    }

    if !first && state != last {
        if matches!(state, "failed" | "restarting") && !matches!(last, "failed" | "restarting") {
            let again = state == "restarting";
            app.add_event(format!(
                "Server ist abgestürzt{}",
                if again {
                    " – wird automatisch neu gestartet"
                } else {
                    ""
                }
            ));
            crate::util::notify(
                "Minecraft-Server abgestürzt",
                if again {
                    "Er wird automatisch neu gestartet."
                } else {
                    ""
                },
            );
        } else if state == "running" {
            app.add_event("Server ist bereit");
            app.refresh_live();
            if app.reach.get_untracked().is_some_and(|r| r.b("server_off")) {
                crate::pages::reach::run_reach(app);
            }
        } else if state == "stopped" && last != "none" {
            app.add_event("Server gestoppt");
        }
        // Kleine Animation: Block setzen bzw. abbauen
        if state == "running" && matches!(last, "starting" | "restarting" | "stopped" | "failed") {
            block_place();
        } else if matches!(state, "stopped" | "none")
            && matches!(last, "running" | "stopping" | "external")
            && let Some(dot) = by_id("status-dot")
        {
            block_break(computed(&dot, "background-color"));
        }
    }

    // Kurven: CPU und RAM
    if st.pid.is_some() {
        if let Some(c) = st.cpu {
            push_spark(app.spark_cpu, c);
        }
        if let Some(r) = st.ram {
            push_spark(app.spark_ram, r as f64 / 1024f64.powi(3));
        }
    }

    // Fenstertitel
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let (label, _) = label_of(state);
        doc.set_title(&match &st.server {
            Some(s) => format!("{} – {label} · Vapafane", s.name),
            None => "Vapafane".into(),
        });
    }
}

/// Spieler und TPS holen (nur wenn der Server läuft).
pub async fn refresh_live(app: AppState) {
    let state = app.status.get_untracked().state;
    if !matches!(state.as_str(), "running" | "external" | "starting") {
        app.live.set(Value::Null);
        return;
    }
    match tauri::call::<Value>("get_live").await {
        Ok(l) => {
            if let Some(t) = l.f("tps") {
                push_spark(app.spark_tps, t);
            }
            app.live.set(l);
        }
        Err(_) => app.live.set(json!({"error": true})),
    }
}

// ---------------------------------------------------------------- Starten / Stoppen

/// Fragt nach der Minecraft-EULA. `true` = zugestimmt.
pub async fn ask_eula(name: &str) -> bool {
    let ok = RwSignal::new(false);
    let name = name.to_string();
    let body: dialog::BodyFn = std::rc::Rc::new(move || {
        let name = name.clone();
        view! {
            <p>"Ohne Zustimmung zur Minecraft-EULA startet kein Server."</p>
            <label class="check">
                <input type="checkbox" on:change=move |e| ok.set(event_target_checked(&e))/>
                " Ich akzeptiere die "
                <a href="#" on:click=move |e| {
                    e.prevent_default();
                    leptos::task::spawn_local(async { let _ = tauri::call::<()>("open_eula").await; });
                }>"Minecraft-EULA"</a>
                {format!(" für „{name}“.")}
            </label>
        }
        .into_any()
    });
    let r = dialog::ask_view(
        "EULA bestätigen",
        body,
        vec![
            Btn::new("Abbrechen", ""),
            Btn::primary("Weiter", "ok").when(ok.into()),
        ],
        false,
    )
    .await;
    r == "ok"
}

pub fn start_server(app: AppState) {
    leptos::task::spawn_local(async move {
        loop {
            match tauri::call::<()>("start_server").await {
                Ok(()) => {
                    app.add_event("Server wird gestartet");
                    break;
                }
                Err(e) if e == "EULA" => {
                    let name = app
                        .status
                        .get_untracked()
                        .server
                        .map(|s| s.name)
                        .unwrap_or_default();
                    if !ask_eula(&name).await {
                        return;
                    }
                    if let Err(e2) = tauri::call::<()>("accept_eula").await {
                        return error("EULA", e2).await;
                    }
                }
                Err(e) => return error("Start fehlgeschlagen", e).await,
            }
        }
        set_timeout(move || app.refresh_status(), Duration::from_millis(400));
    });
}

pub fn ask_stop(app: AppState, restart: bool) {
    leptos::task::spawn_local(async move {
        let verb = if restart { "neu starten" } else { "stoppen" };
        let secs = tauri::call::<Value>("get_config")
            .await
            .ok()
            .and_then(|c| c.f("stop_warning"))
            .unwrap_or(30.0) as u64;
        if app.status.get_untracked().state == "external" {
            if confirm("Server stoppen?", "Der Server läuft außerhalb von Vapafane. Ich schicke ihm den Befehl „stop“ – er speichert die Welt und beendet sich.", "Stoppen").await {
                do_stop(app, false, 0);
            }
            return;
        }
        let players: Vec<String> = app
            .live
            .get_untracked()
            .a("players")
            .iter()
            .filter_map(|p| p.as_str().map(String::from))
            .collect();
        let choice = if players.is_empty() {
            let mut cap = verb.to_string();
            cap.replace_range(..1, &cap[..1].to_uppercase());
            ask(
                format!("Server {verb}?"),
                "Gerade ist niemand online.",
                vec![Btn::new("Abbrechen", ""), Btn::primary(cap, "0")],
            )
            .await
        } else {
            ask(
                format!("Server {verb}?"),
                format!("{} Spieler online: {}", players.len(), players.join(", ")),
                vec![
                    Btn::new("Abbrechen", ""),
                    Btn::new("Sofort", "0"),
                    Btn::primary(format!("In {secs} Sek. mit Warnung"), "warn"),
                ],
            )
            .await
        };
        match choice.as_str() {
            "0" => do_stop(app, restart, 0),
            "warn" => do_stop(app, restart, secs),
            _ => {}
        }
    });
}

pub fn do_stop(app: AppState, restart: bool, warn: u64) {
    app.add_event(format!(
        "Server {}{}",
        if restart { "startet neu" } else { "stoppt" },
        if warn > 0 {
            format!(" in {warn} Sek.")
        } else {
            String::new()
        }
    ));
    leptos::task::spawn_local(async move {
        if let Err(e) =
            tauri::invoke::<()>("stop_server", json!({ "warn": warn, "restart": restart })).await
        {
            error("Stoppen fehlgeschlagen", e).await;
        }
        set_timeout(move || app.refresh_status(), Duration::from_millis(400));
    });
}

pub fn backup_now(app: AppState) {
    app.add_event("Backup gestartet");
    leptos::task::spawn_local(async move {
        match crate::tasks::task("backup_now", json!({})).await {
            Ok(r) => app.add_event(format!(
                "Backup fertig: {} ({})",
                crate::util::file_name(&r.s("path")),
                fmt_bytes(r.f("size"))
            )),
            Err(e) => error("Backup fehlgeschlagen", e).await,
        }
    });
}

// ---------------------------------------------------------------- Animationen

/// Pixel-Partikel um den Status-Block: erst hoch, dann fallen sie.
fn pixel_burst(colors: &[String], n: usize, spread: f64, fall: f64) {
    let (Some(dot), Some(doc)) = (
        by_id("status-dot"),
        web_sys::window().and_then(|w| w.document()),
    ) else {
        return;
    };
    let Some(boxx) = dot.parent_element() else {
        return;
    };
    let cx = dot.offset_left() as f64 + dot.offset_width() as f64 / 2.0;
    let cy = dot.offset_top() as f64 + dot.offset_height() as f64 / 2.0;
    for i in 0..n {
        let Ok(p) = doc.create_element("span") else {
            continue;
        };
        p.set_class_name("fx-px");
        let _ = p.set_attribute(
            "style",
            &format!(
                "background:{};left:{}px;top:{}px",
                colors[i % colors.len()],
                cx - 2.0,
                cy - 2.0
            ),
        );
        let _ = boxx.append_child(&p);
        let a = std::f64::consts::TAU * i as f64 / n as f64 + js_sys::Math::random() * 0.4;
        let d = spread * (0.6 + js_sys::Math::random() * 0.5);
        let (dx, up) = (a.cos() * d, a.sin() * d * 0.5 - 12.0);
        animate(
            &p,
            json!([
                {"transform": "translate(0, 0)", "opacity": 1},
                {"transform": format!("translate({}px, {}px)", dx * 0.8, up), "opacity": 1, "offset": 0.4},
                {"transform": format!("translate({}px, {}px)", dx, up + fall), "opacity": 0}
            ]),
            json!({"duration": 650.0 + js_sys::Math::random() * 250.0, "easing": "steps(9)"}),
        );
        set_timeout(move || p.remove(), Duration::from_millis(950));
    }
}

/// Start: der Block fällt an seinen Platz, Partikel stieben, kurzes Leuchten.
fn block_place() {
    if reduced_motion() {
        return;
    }
    let Some(dot) = by_id("status-dot") else {
        return;
    };
    animate(
        &dot,
        json!([
            {"transform": "translateY(-24px) scale(0.5)", "opacity": 0},
            {"transform": "translateY(2px) scale(1.12)", "opacity": 1, "offset": 0.7},
            {"transform": "none", "opacity": 1}
        ]),
        json!({"duration": 420, "easing": "steps(7)"}),
    );
    set_timeout(
        move || {
            pixel_burst(
                &[css_var("--p1"), css_var("--p2"), css_var("--p3")],
                12,
                30.0,
                20.0,
            );
            if let Some(dot) = by_id("status-dot") {
                animate(
                    &dot,
                    json!([
                        {"boxShadow": format!("0 0 0 3px var(--panel), 0 0 14px 5px {}", css_var("--p2"))},
                        {"boxShadow": "0 0 0 3px var(--panel), 0 0 0 4px var(--border)"}
                    ]),
                    json!({"duration": 900}),
                );
            }
        },
        Duration::from_millis(300),
    );
}

/// Ein Element genau über den Status-Block legen.
fn ghost(bg: &str) -> Option<web_sys::Element> {
    let dot = by_id("status-dot")?;
    let doc = web_sys::window()?.document()?;
    let g = doc.create_element("span").ok()?;
    g.set_class_name("fx-ghost");
    let shadow = if bg == "transparent" {
        String::new()
    } else {
        format!(";box-shadow:{}", computed(&dot, "box-shadow"))
    };
    let _ = g.set_attribute(
        "style",
        &format!(
            "left:{}px;top:{}px;width:{}px;height:{}px;background:{bg}{shadow}",
            dot.offset_left(),
            dot.offset_top(),
            dot.offset_width(),
            dot.offset_height()
        ),
    );
    dot.parent_element()?.append_child(&g).ok()?;
    Some(g)
}

/// Stopp: Risse in drei Stufen mit leichtem Wackeln, dann zerfällt der Block in Pixel.
fn block_break(color: String) {
    if reduced_motion() {
        return;
    }
    let (Some(g), Some(crack)) = (ghost(&color), ghost("transparent")) else {
        return;
    };
    for (i, v) in ["--svg-crack1", "--svg-crack2", "--svg-crack3"]
        .into_iter()
        .enumerate()
    {
        let c = crack.clone();
        set_timeout(
            move || {
                let style = c.get_attribute("style").unwrap_or_default();
                let _ =
                    c.set_attribute("style", &format!("{style};background-image:{}", css_var(v)));
            },
            Duration::from_millis(i as u64 * 170),
        );
    }
    for el in [&g, &crack] {
        animate(
            el,
            json!([{"transform": "none"}, {"transform": "translate(1px, -1px)"}, {"transform": "translate(-1px, 1px)"}, {"transform": "none"}]),
            json!({"duration": 170, "iterations": 3, "easing": "steps(4)"}),
        );
    }
    set_timeout(
        move || {
            crack.remove();
            g.remove();
            pixel_burst(
                &[color.clone(), css_var("--p3"), "#555".into()],
                14,
                26.0,
                26.0,
            );
            if let Some(dot) = by_id("status-dot") {
                animate(
                    &dot,
                    json!([{"transform": "scale(0)"}, {"transform": "scale(1)"}]),
                    json!({"duration": 300, "delay": 250, "easing": "steps(4)", "fill": "backwards"}),
                );
            }
        },
        Duration::from_millis(520),
    );
}

// ---------------------------------------------------------------- Ansicht

/// Kleine Kurve (letzte 150 Werte).
#[component]
pub fn Spark(
    values: RwSignal<Vec<f64>>,
    color: &'static str,
    #[prop(into)] max: Signal<Option<f64>>,
) -> impl IntoView {
    let points = move || {
        let v = values.get();
        if v.len() < 2 {
            return None;
        }
        let top = max
            .get()
            .unwrap_or_else(|| v.iter().cloned().fold(1e-9, f64::max));
        let step = 150.0 / 149.0;
        let x0 = 150.0 - (v.len() - 1) as f64 * step;
        let pts: Vec<String> = v
            .iter()
            .enumerate()
            .map(|(i, y)| {
                format!(
                    "{:.1},{:.1}",
                    x0 + i as f64 * step,
                    32.0 - (y.min(top) / top) * 30.0
                )
            })
            .collect();
        Some((format!("{x0},34 {} 150,34", pts.join(" ")), pts.join(" ")))
    };
    view! {
        <svg class="spark" viewBox="0 0 150 34" preserveAspectRatio="none">
            {move || points().map(|(poly, line)| view! {
                <polygon points=poly style=format!("fill:var({color})")></polygon>
                <polyline points=line style=format!("stroke:var({color})")></polyline>
            })}
        </svg>
    }
}

#[component]
fn Card(
    title: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] sub: Signal<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] color: Option<Signal<String>>,
) -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-title">{title}</div>
            <div class="card-value" style:color=move || color.map(|c| c.get()).unwrap_or_default()>{value}</div>
            <div class="card-sub">{sub}</div>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn Overview() -> impl IntoView {
    let app = use_app();
    let st = app.status;
    let state = move || st.get().state;
    let free = move || {
        let s = st.get();
        s.busy.is_none() && s.job.is_none()
    };
    let label = move || label_of(&state()).0;
    let color = move || label_of(&state()).1;
    let sub = move || {
        let s = st.get();
        let mut parts = vec![];
        if let Some(u) = s.uptime.filter(|u| *u > 0)
            && matches!(
                s.state.as_str(),
                "running" | "starting" | "stopping" | "external"
            )
        {
            parts.push(format!("seit {}", fmt_duration(u as f64)));
        }
        if let Some(r) = s.restarts.filter(|r| *r > 0) {
            parts.push(format!("{r}× automatisch neu gestartet"));
        }
        if let Some(c) = &s.countdown {
            parts.push(c.clone());
        }
        if let Some(b) = s.busy.as_ref().or(s.job.as_ref()) {
            parts.push(format!("{b} …"));
        }
        parts.join("  ·  ")
    };
    let hint = move || {
        let s = st.get();
        let last = s.last_exit.clone().unwrap_or_default();
        match s.state.as_str() {
            "external" => Some(("Der Server wurde nicht über Vapafane gestartet (z. B. altes Bedienfenster oder Terminal). Konsole und Spieler gehen trotzdem; „Stoppen“ schickt ihm den stop-Befehl.".to_string(), false)),
            "other" => Some(("Der Hintergrund-Dienst betreibt gerade einen anderen Server. Bitte den zuerst stoppen.".into(), false)),
            "failed" => Some((format!("Der Server ist abgestürzt und wurde nicht mehr neu gestartet. Schau in der Konsole nach der letzten Fehlermeldung.{}",
                if last.is_empty() { String::new() } else { format!("\n\n{last}") }), true)),
            "none" => Some(("Füge oben über „＋“ einen Server hinzu.".into(), false)),
            "restarting" if !last.is_empty() => Some((last, true)),
            _ => None,
        }
    };

    // Karten
    let live = app.live;
    let running = move || matches!(state().as_str(), "running" | "external" | "starting");
    let players = move || {
        if !running() {
            "–".to_string()
        } else {
            live.get()
                .f("online")
                .map(|o| format!("{} / {}", o, live.get().f("max").unwrap_or(0.0)))
                .unwrap_or_else(|| "–".into())
        }
    };
    let players_sub = move || {
        let l = live.get();
        if !running() {
            return if state() == "stopped" {
                "Server aus".into()
            } else {
                String::new()
            };
        }
        if l.b("error") {
            return if state() == "starting" {
                "Server startet …".into()
            } else {
                "keine Verbindung".into()
            };
        }
        let names: Vec<String> = l
            .a("players")
            .iter()
            .filter_map(|p| p.as_str().map(String::from))
            .collect();
        if l.f("online").is_some() {
            if names.is_empty() {
                "niemand".into()
            } else {
                names.join(", ")
            }
        } else {
            String::new()
        }
    };
    let tps = move || {
        if running() {
            live.get()
                .f("tps")
                .map(|t| num(t, 1))
                .unwrap_or_else(|| "–".into())
        } else {
            "–".into()
        }
    };
    let tps_sub = move || {
        let l = live.get();
        match (running(), l.f("tps")) {
            (true, Some(t)) => format!(
                "{} ms pro Tick  ·  {}",
                num(l.f("mspt").unwrap_or(0.0), 1),
                if t < 18.0 { "ruckelt" } else { "flüssig" }
            ),
            _ => String::new(),
        }
    };
    let cpu = move || {
        let s = st.get();
        match (s.pid, s.cpu) {
            (Some(_), Some(c)) => format!("{} %", c.round()),
            _ => "–".into(),
        }
    };
    let cpu_sub = move || {
        let s = st.get();
        if s.pid.is_some() && s.cpu.is_some() {
            format!("von {} Kernen", s.ncpu.unwrap_or(0))
        } else {
            String::new()
        }
    };
    let ram = move || {
        let s = st.get();
        match (s.pid, s.ram) {
            (Some(_), Some(r)) => fmt_bytes(Some(r as f64)),
            _ => "–".into(),
        }
    };
    let ram_sub = move || {
        let s = st.get();
        let lim = s.ram_limit_gb.map(|g| format!("{g}"));
        match (s.pid.is_some() && s.ram.is_some(), lim) {
            (true, Some(l)) => format!("belegt (Limit {l} GB)"),
            (true, None) => "belegt".into(),
            (false, Some(l)) => format!("Limit {l} GB"),
            (false, None) => String::new(),
        }
    };
    let d = app.details;
    let has = move || !d.get().is_null();
    let size = move || {
        if has() {
            fmt_bytes(d.get().f("size"))
        } else {
            "–".into()
        }
    };
    let size_sub = move || {
        if has() {
            format!("{} frei", fmt_bytes(d.get().f("free")))
        } else {
            String::new()
        }
    };
    let addr = move || if has() { d.get().s("ip") } else { "–".into() };
    let addr_sub = move || {
        if has() {
            format!("Port {}", d.get().s("port"))
        } else {
            String::new()
        }
    };
    let backup = move || {
        if !has() {
            return "–".into();
        }
        match d.get().f("last_backup/age") {
            Some(a) => format!("vor {}", fmt_duration(a)),
            None => "noch keins".into(),
        }
    };
    let backup_sub = move || {
        if !has() {
            return String::new();
        }
        let v = d.get();
        if v.o("last_backup").is_null() {
            "Knopf „Backup“ drücken".into()
        } else {
            format!(
                "{} · {} gespeichert",
                fmt_bytes(v.f("last_backup/size")),
                v.s("backups")
            )
        }
    };
    let version = move || {
        if has() {
            let v = d.get().s("info/version");
            if v.is_empty() { "?".into() } else { v }
        } else {
            "–".into()
        }
    };
    let version_sub = move || {
        if !has() {
            return String::new();
        }
        let v = d.get();
        let base = if v.s("info/loader").is_empty() {
            v.s("info/type")
        } else {
            format!("Fabric {}", v.s("info/loader"))
        };
        let mods = v
            .f("mods")
            .filter(|m| *m > 0.0)
            .map(|m| format!(" · {m} Mods"))
            .unwrap_or_default();
        format!("{base}{mods}")
    };
    let reach = app.reach;
    let reach_val = move || match reach.get() {
        Some(r) => match r.s("light").as_str() {
            "green" => "Erreichbar",
            "yellow" => "Nur IPv6",
            "red" => "Nicht erreichbar",
            _ => "–",
        }
        .to_string(),
        None => "–".into(),
    };
    let reach_color = move || {
        match reach.get().map(|r| r.s("light")).as_deref() {
            Some("green") => "var(--green)",
            Some("yellow") => "var(--amber)",
            Some("red") => "var(--red)",
            _ => "",
        }
        .to_string()
    };
    let reach_sub = move || match reach.get() {
        None => String::new(),
        Some(r) => match r.s("light").as_str() {
            "gray" => r.s("title"),
            _ if r.b("ds_lite") => "DS-Lite".into(),
            "green" => {
                if r.s("v6_state") == "ok" {
                    "IPv4 und IPv6".into()
                } else {
                    "über IPv4".into()
                }
            }
            "red" => "Portfreigabe prüfen".into(),
            _ => String::new(),
        },
    };
    let ram_max = Signal::derive(move || st.get().ram_limit_gb);

    view! {
        <section class="tab" class:active=move || app.page.get() == Page::Overview>
            <div class="status-row">
                <div class="status">
                    <span id="status-dot" class="dot" class:pulse=move || matches!(state().as_str(), "starting" | "stopping" | "restarting")
                        style:background=move || format!("var({})", color())></span>
                    <div>
                        <div class="status-big">{label}</div>
                        <div class="dim">{sub}</div>
                    </div>
                </div>
                <div class="buttons">
                    <button class="primary" disabled=move || !(matches!(state().as_str(), "stopped" | "failed") && free())
                        on:click=move |_| start_server(app)>"▶ Starten"</button>
                    <button class="danger" disabled=move || {
                        let s = st.get();
                        let up = s.up();
                        !((up && s.state != "stopping" && s.countdown.is_none()) || s.state == "external") || !free()
                    } on:click=move |_| ask_stop(app, false)>"■ Stoppen"</button>
                    <button disabled=move || !(state() == "running" && st.get().countdown.is_none() && free())
                        on:click=move |_| ask_stop(app, true)>"⟳ Neustart"</button>
                    <button disabled=move || st.get().server.is_none() || !free() on:click=move |_| backup_now(app)>"Backup"</button>
                    <button class="primary" on:click=move |_| crate::pages::invite::open_invite()>"Freunde einladen"</button>
                </div>
            </div>
            {move || hint().map(|(h, err)| view! { <div class="hint" class:error=err>{h}</div> })}
            <crate::pages::players::WhitelistNote page=Page::Overview/>
            <div class="cards">
                <Card title="Spieler online" value=Signal::derive(players) sub=Signal::derive(players_sub)/>
                <Card title="TPS (Ticks pro Sekunde)" value=Signal::derive(tps) sub=Signal::derive(tps_sub)>
                    <Spark values=app.spark_tps color="--green" max=Some(20.0)/>
                </Card>
                <Card title="CPU" value=Signal::derive(cpu) sub=Signal::derive(cpu_sub)>
                    <Spark values=app.spark_cpu color="--blue" max=Some(100.0)/>
                </Card>
                <Card title="Arbeitsspeicher" value=Signal::derive(ram) sub=Signal::derive(ram_sub)>
                    <Spark values=app.spark_ram color="--amber" max=ram_max/>
                </Card>
                <Card title="Server-Ordner" value=Signal::derive(size) sub=Signal::derive(size_sub)/>
                <Card title="Von außen" value=Signal::derive(reach_val) sub=Signal::derive(reach_sub) color=Signal::derive(reach_color)/>
                <Card title="Adresse (Heimnetz)" value=Signal::derive(addr) sub=Signal::derive(addr_sub)/>
                <Card title="Letztes Backup" value=Signal::derive(backup) sub=Signal::derive(backup_sub)/>
                <Card title="Version" value=Signal::derive(version) sub=Signal::derive(version_sub)/>
            </div>
            <h3>"Letzte Ereignisse"</h3>
            <ul class="events">
                {move || {
                    let ev = app.events.get();
                    if ev.is_empty() {
                        view! { <li class="dim">"Noch nichts passiert."</li> }.into_any()
                    } else {
                        ev.into_iter().map(|e| view! { <li><span class="t">{e.time}</span>{e.text}</li> }).collect_view().into_any()
                    }
                }}
            </ul>
            <p class="dim small">"Fenster schließen stoppt den Server nicht – er läuft im Hintergrund weiter."</p>
            <p class="notice small">"Vapafane ist kein offizielles Minecraft-Produkt. Es ist nicht von Mojang oder Microsoft genehmigt und steht in keiner Verbindung zu ihnen. "
                <a href="#" on:click=|e| { e.prevent_default(); crate::util::open_link("issues"); }>"Kontakt & Fehler melden (GitHub)"</a></p>
        </section>
    }
}
