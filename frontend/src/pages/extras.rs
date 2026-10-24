//! Anleitung „Shader & Extras“: eigene Shader, Ressourcenpakete und Mods zu einem eingerichteten
//! Server hinzufügen – für die Modrinth-App mit Skizzen, für den Minecraft-Launcher über den Ordner.

use leptos::prelude::*;
use serde_json::json;
use std::rc::Rc;

use crate::dialog::{self, Btn, show_error};
use crate::tauri;
use crate::util::open_link;

/// Nummer in einer Skizze (passt zur Schritt-Nummer im Text).
fn marker(x: i32, y: i32, n: &'static str) -> impl IntoView {
    view! {
        <g class="mark">
            <circle cx=x cy=y r="11"/>
            <text x=x y=y + 4 text-anchor="middle">{n}</text>
        </g>
    }
}

/// Skizze: Instanz in der Modrinth-App (Bibliothek, Reiter „Content“, „Inhalte durchsuchen“).
fn sketch_instance(name: String) -> impl IntoView {
    view! {
        <svg class="sketch" viewBox="0 0 520 190" role="img" aria-label="Skizze der Instanz in der Modrinth-App">
            <rect class="frame" x="1" y="1" width="518" height="188" rx="10"/>
            // Seitenleiste mit Instanzen
            <rect class="side" x="1" y="1" width="46" height="188" rx="10"/>
            <rect class="icon" x="14" y="22" width="20" height="20" rx="5"/>
            <rect class="icon" x="14" y="54" width="20" height="20" rx="5"/>
            <rect class="icon hot" x="12" y="100" width="24" height="24" rx="7"/>
            {marker(54, 98, "1")}
            // Kopf der Instanz
            <rect class="icon" x="66" y="16" width="34" height="34" rx="7"/>
            <text class="title" x="110" y="31">{name}</text>
            <text class="sub" x="110" y="46">"Minecraft · Fabric"</text>
            <rect class="btn green" x="420" y="18" width="80" height="28" rx="8"/>
            <text class="btn-t dark" x="460" y="37" text-anchor="middle">"Spielen"</text>
            // Reiter
            <rect class="pill hot" x="66" y="64" width="74" height="24" rx="7"/>
            <text class="btn-t dark" x="103" y="80" text-anchor="middle">"Content"</text>
            <text class="tab-t" x="160" y="80">"Files"</text>
            <text class="tab-t" x="206" y="80">"Worlds"</text>
            <text class="tab-t" x="262" y="80">"Logs"</text>
            {marker(150, 66, "2")}
            // leere Liste mit Knöpfen
            <text class="sub" x="290" y="122" text-anchor="middle">"Keine zusätzlichen Inhalte installiert"</text>
            <rect class="btn" x="160" y="140" width="122" height="30" rx="8"/>
            <text class="btn-t" x="221" y="160" text-anchor="middle">"Dateien hochladen"</text>
            <rect class="btn green" x="292" y="140" width="136" height="30" rx="8"/>
            <text class="btn-t dark" x="360" y="160" text-anchor="middle">"Inhalte durchsuchen"</text>
            {marker(440, 140, "3")}
        </svg>
    }
}

/// Skizze: Suche in der Modrinth-App (Art wählen, „Installieren“).
fn sketch_browse() -> impl IntoView {
    view! {
        <svg class="sketch" viewBox="0 0 520 130" role="img" aria-label="Skizze der Suche in der Modrinth-App">
            <rect class="frame" x="1" y="1" width="518" height="128" rx="10"/>
            <text class="tab-t" x="20" y="30">"Mods"</text>
            <text class="tab-t" x="74" y="30">"Resource Packs"</text>
            <text class="tab-t" x="186" y="30">"Data Packs"</text>
            <rect class="pill hot" x="270" y="14" width="74" height="24" rx="7"/>
            <text class="btn-t dark" x="307" y="30" text-anchor="middle">"Shaders"</text>
            {marker(354, 16, "4")}
            <rect class="row" x="14" y="52" width="492" height="62" rx="8"/>
            <rect class="icon" x="26" y="62" width="40" height="40" rx="7"/>
            <text class="title" x="78" y="80">"Ein Shader, z. B. Complementary"</text>
            <text class="sub" x="78" y="96">"Beschreibung · Downloads"</text>
            <rect class="btn green" x="396" y="68" width="98" height="30" rx="8"/>
            <text class="btn-t dark" x="445" y="88" text-anchor="middle">"Installieren"</text>
            {marker(498, 66, "5")}
        </svg>
    }
}

/// Anleitung öffnen. `target`: "modrinth" oder "launcher"; `dir`: Ordner des Clients.
pub fn show_guide(target: String, name: String, dir: String, shaders: bool) {
    let modrinth = target == "modrinth";
    leptos::task::spawn_local(async move {
        let inst = format!("{name} (Vapafane)");
        let body: dialog::BodyFn = Rc::new(move || {
            let shader_note = if shaders {
                view! { <p class="hint">"✓ Shader-Unterstützung (Iris) ist eingerichtet – Vapafane hält sie bei Server-Updates passend."</p> }.into_any()
            } else {
                view! { <p class="hint">"Für Shader braucht Minecraft die Mod "<b>"Iris"</b>". Am einfachsten: die Einladung nochmal einlösen und "
                    <b>"„Shader-Unterstützung mit einrichten“"</b>" anhaken – dann bringt Vapafane Iris passend zur Server-Version mit, auch bei jedem Update."</p> }.into_any()
            };
            if modrinth {
                let i1 = inst.clone();
                view! {
                    <p>"In deiner Instanz "<b>{format!("„{inst}“")}</b>" kannst du jederzeit eigene Shader, Ressourcenpakete und Mods hinzufügen. "
                        "Sie bleiben bei Server-Updates erhalten – die Modrinth-App tauscht dabei nur die Mods des Servers aus."</p>
                    {shader_note}
                    {sketch_instance(i1)}
                    <ol class="steps">
                        <li>"Modrinth-App öffnen und links die Instanz "<b>{format!("„{inst}“")}</b>" anklicken."</li>
                        <li>"Oben den Reiter "<b>"„Content“"</b>" (Inhalte) wählen."</li>
                        <li>"Auf den grünen Knopf "<b>"„Inhalte durchsuchen“"</b>" klicken."</li>
                    </ol>
                    {sketch_browse()}
                    <ol class="steps" start="4">
                        <li>"Oben die Art wählen: "<b>"„Shaders“"</b>" für Shader, "<b>"„Resource Packs“"</b>" für Ressourcenpakete oder "<b>"„Mods“"</b>"."</li>
                        <li>"Beim gewünschten Eintrag auf "<b>"„Installieren“"</b>" klicken – fertig."</li>
                    </ol>
                    <p class="dim small">"Im Spiel: Optionen → Grafikeinstellungen → Shader-Pakete → Shader auswählen. Eigene Mods musst du nach einem Minecraft-Update in der App selbst aktualisieren; Shader und Ressourcenpakete meist nicht."</p>
                }.into_any()
            } else {
                view! {
                    <p>"Der Spielordner dieses Servers gehört nur ihm: Was du dort hinzufügst, bleibt bei Server-Updates erhalten."</p>
                    {shader_note}
                    <ol class="steps">
                        <li>"Shader herunterladen, z. B. auf "<a href="#" on:click=|e| { e.prevent_default(); open_link("https://modrinth.com/shaders"); }>"modrinth.com/shaders"</a>
                            " – die "<b>".zip"</b>"-Datei "<b>"nicht"</b>" entpacken."</li>
                        <li>"Unten auf "<b>"„Ordner öffnen“"</b>" klicken und die Datei in den Ordner "<b>"„shaderpacks“"</b>" legen. Ressourcenpakete gehören in "<b>"„resourcepacks“"</b>"."</li>
                        <li>"Im Spiel: Optionen → Grafikeinstellungen → Shader-Pakete → Shader auswählen."</li>
                    </ol>
                    <p class="dim small">"Eigene Mods kannst du in den Ordner „mods“ legen – nach einem Minecraft-Update musst du sie aber selbst passend halten."</p>
                }.into_any()
            }
        });
        let buttons = vec![
            Btn::new("Schließen", ""),
            if modrinth {
                Btn::primary("Modrinth-App öffnen", "open")
            } else {
                Btn::primary("Ordner öffnen", "folder")
            },
        ];
        match dialog::ask_view("Shader & Extras hinzufügen", body, buttons, true).await.as_str() {
            "open" => {
                if let Err(e) = tauri::invoke::<()>("client_play", json!({ "target": "modrinth" })).await {
                    show_error("Modrinth-App", e);
                }
            }
            "folder" => {
                if let Err(e) = tauri::invoke::<()>("client_open_folder", json!({ "dir": dir })).await {
                    show_error("Ordner", e);
                }
            }
            _ => {}
        }
    });
}
