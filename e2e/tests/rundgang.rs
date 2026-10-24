//! Rundgang durch alle Funktionen von Vapafane im Testmodus – bedient das echte Programm.
//! Läuft nur auf Wunsch (braucht Xvfb, WebKitWebDriver, tauri-driver, Java und Internet):
//!
//!     cargo test --test rundgang -- --ignored --nocapture
//!
//! Programm: VAPAFANE_APP (Standard: ../src-tauri/target/debug/vapafane).
//! Alle Daten landen in einem eigenen, leeren Ordner – echte Server bleiben unberührt.

use std::path::PathBuf;
use vapafane_e2e::{Env, Res, fail};

const PORT: &str = "25599";

fn app() -> PathBuf {
    std::env::var_os("VAPAFANE_APP")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../src-tauri/target/debug/vapafane")))
}

fn home() -> PathBuf {
    let base = std::env::var_os("VAPAFANE_E2E_HOME").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    base.join(format!("vapafane-e2e-{}", std::process::id()))
}

macro_rules! schritt {
    ($env:expr, $name:expr, $body:expr) => {{
        println!("▶ {}", $name);
        let r: Res<()> = $body.await;
        if let Err(err) = r {
            $env.shot(&format!("fehler-{}", $name.replace(' ', "_"))).await;
            println!("✗ {}: {err}\n--- Seite ---\n{}\n--- Fehler der Oberfläche ---\n{:?}", $name, $env.page_text().await, $env.errors().await);
            return Err(err);
        }
        println!("✓ {}", $name);
    }};
}

/// Seite in der Seitenleiste öffnen.
async fn page(e: &Env, label: &str) -> Res<()> {
    let xp = format!("//aside//button[.//span[normalize-space()={}]]", vapafane_e2e::lit(label));
    e.wait(&xp, 10).await?.click().await?;
    Ok(())
}

#[tokio::test]
#[ignore = "braucht Xvfb, WebKitWebDriver, tauri-driver, Java und Internet"]
async fn rundgang() -> Res<()> {
    let env = Env::start(&app(), &home(), ":99").await?;
    let e = &env;
    println!("Daten in {}", e.home.display());
    e.watch_errors().await?;
    let mut code = String::new();

    schritt!(e, "Willkommen", async {
        e.wait("//h2[contains(.,'Willkommen bei Vapafane')]", 20).await?;
        e.click_button("OK").await
    });
    schritt!(e, "Start im Client-Bereich", async {
        e.wait("//h2[normalize-space()='Einladung einlösen']", 10).await?;
        e.until("document.body.classList.contains('client-mode')", 5).await
    });
    schritt!(e, "Falscher Einladungs-Code", async {
        e.type_into("//textarea[@id='cl-code']", "hallo").await?;
        e.click_button("Einladung prüfen").await?;
        e.wait("//p[contains(@class,'error') and contains(.,'VPF1-')]", 10).await?;
        Ok(())
    });
    schritt!(e, "Server-Bereich ohne Server", async {
        e.click_button("Server").await?;
        e.until("!document.body.classList.contains('client-mode')", 5).await?;
        e.until("document.getElementById('btn-add-server').classList.contains('glow')", 10).await?;
        page(e, "Erreichbarkeit").await?;
        e.wait("//div[normalize-space()='Kein Server ausgewählt']", 10).await?;
        page(e, "Übersicht").await
    });
    schritt!(e, "Neuen Server erstellen", async {
        e.wait("//button[@id='btn-add-server']", 5).await?.click().await?;
        e.click_button("Neuen Server erstellen (Fabric, Paper, NeoForge, Vanilla) …").await?;
        e.wait("//h2[normalize-space()='Neuen Server erstellen']", 10).await?;
        e.type_into("//div[@id='modal-body']/input", "E2E").await?;
        e.select("(//div[@id='modal-body']//select)[1]", "vanilla").await?;
        e.until("(() => { const s = document.querySelectorAll('#modal-body select')[1]; return s && s.value && s.value !== ''; })()", 60).await?;
        e.until("document.querySelector('#modal-body code') && document.querySelector('#modal-body code').textContent.length > 0", 15).await?;
        e.wait("//div[@id='modal-body']//input[@type='checkbox']", 5).await?.click().await?;
        e.click_button("Weiter").await?;
        e.until("Array.from(document.querySelectorAll('#server-select option')).some(o => o.textContent === 'E2E')", 600).await?;
        e.shot("02-server-angelegt").await;
        Ok(())
    });
    schritt!(e, "Einstellungen: Port ändern und speichern", async {
        e.wait("//button[@id='btn-gear']", 5).await?.click().await?;
        e.type_into("//label[normalize-space()='Server-Port']/following-sibling::input[1]", PORT).await?;
        e.click_button("Einstellungen speichern").await?;
        e.wait("//span[normalize-space()='Gespeichert.']", 10).await?;
        e.shot("03-einstellungen").await;
        page(e, "Übersicht").await?;
        e.sees(&format!("Port {PORT}"), 70).await?;
        Ok(())
    });
    schritt!(e, "Server starten", async {
        e.click_button("▶ Starten").await?;
        e.wait("//div[@class='status-big' and normalize-space()='Läuft']", 300).await?;
        e.wait("//li[contains(.,'Server ist bereit')]", 30).await?;
        e.shot("04-laeuft").await;
        Ok(())
    });
    schritt!(e, "Konsole: Befehl senden", async {
        page(e, "Konsole").await?;
        e.wait("//div[contains(@class,'console')]/div[contains(.,'Done (')]", 30).await?;
        e.type_into("//div[@class='cmd-row']/input", "list").await?;
        e.click_button("Senden").await?;
        e.wait("//div[contains(@class,'console')]/div[contains(@class,'resp') and contains(.,'players online')]", 20).await?;
        e.shot("05-konsole").await;
        Ok(())
    });
    schritt!(e, "Spieler und Whitelist", async {
        page(e, "Spieler").await?;
        e.wait("//div[@class='empty' and normalize-space()='Niemand online']", 15).await?;
        e.type_into("//input[@placeholder='Minecraft-Name']", "Notch").await?;
        e.click_button("Hinzufügen").await?;
        e.wait("//div[@class='item']/div[normalize-space()='Notch']", 30).await?;
        Ok(())
    });
    schritt!(e, "Backup erstellen", async {
        page(e, "Backups").await?;
        e.click_button("Jetzt sichern").await?;
        e.wait("//section[contains(@class,'active')]//button[normalize-space()='Wiederherstellen']", 180).await?;
        let _ = e.wait("//li[contains(.,'Backup fertig')]", 10).await;
        Ok(())
    });
    schritt!(e, "Version & Updates prüfen", async {
        page(e, "Version & Updates").await?;
        e.click_button("Versionen laden").await?;
        e.click_button("Prüfen").await?;
        e.until("(() => { const s = document.querySelector('section.active .summary'); return s && s.textContent.trim().length > 0; })()", 300).await?;
        e.shot("06-updates").await;
        Ok(())
    });
    schritt!(e, "Erreichbarkeit prüfen", async {
        page(e, "Erreichbarkeit").await?;
        e.click_button("Erneut prüfen").await?;
        e.wait("//span[starts-with(normalize-space(),'Geprüft um')]", 60).await?;
        Ok(())
    });
    schritt!(e, "Datapack-Katalog", async {
        page(e, "Mods/Plugins").await?;
        e.click_button("Datapacks hinzufügen …").await?;
        e.wait("//div[contains(@class,'catalog-list')]//div[@class='hit']", 60).await?;
        e.shot("07-katalog").await;
        e.click_button("Schließen").await
    });
    schritt!(e, "Freunde einladen", async {
        page(e, "Übersicht").await?;
        e.click_button("Freunde einladen").await?;
        e.click_button("Code erzeugen").await?;
        let t = e.wait("//textarea[@id='inv-code']", 60).await?;
        let v = t.prop("value").await?.unwrap_or_default();
        if !v.starts_with("VPF1-") {
            return fail(format!("Code sieht falsch aus: {v}"));
        }
        code = v;
        e.click_button("Schließen").await
    });
    schritt!(e, "Server stoppen", async {
        e.click_button("■ Stoppen").await?;
        e.wait("//h2[normalize-space()='Server stoppen?']", 10).await?;
        e.click_button("Stoppen").await?;
        e.wait("//div[@class='status-big' and normalize-space()='Gestoppt']", 120).await?;
        Ok(())
    });
    schritt!(e, "Backup wiederherstellen", async {
        page(e, "Backups").await?;
        e.click_button("Wiederherstellen").await?;
        e.wait("//h2[contains(.,'zurücksetzen?')]", 10).await?;
        let modal = "//div[@class='modal-buttons']/button[normalize-space()='Wiederherstellen']";
        e.wait(modal, 5).await?.click().await?;
        e.wait("//h2[normalize-space()='Wiederhergestellt']", 180).await?;
        e.click_button("OK").await
    });
    schritt!(e, "Client: Einladung prüfen und einrichten", async {
        e.click_button("Client").await?;
        e.type_into("//textarea[@id='cl-code']", &code).await?;
        e.click_button("Einladung prüfen").await?;
        e.wait("//span[@class='big' and normalize-space()='E2E']", 60).await?;
        e.wait("//label[input[@name='cl-dest'] and contains(.,'Minecraft-Launcher')]", 10).await?.click().await?;
        e.click_button("Einrichten").await?;
        e.wait("//p[contains(.,'Eingerichtet')]", 600).await?;
        e.wait("//div[@class='item']//b[normalize-space()='E2E']", 30).await?;
        e.shot("08-client").await;
        Ok(())
    });
    schritt!(e, "Client entfernen", async {
        let rm = "//div[@class='item'][.//b[normalize-space()='E2E']]//button[normalize-space()='Entfernen']";
        e.wait(rm, 10).await?.click().await?;
        e.wait("//div[@class='modal-buttons']/button[normalize-space()='Entfernen']", 10).await?.click().await?;
        e.wait("//div[@class='empty' and contains(.,'Noch kein Server eingerichtet')]", 30).await?;
        Ok(())
    });
    schritt!(e, "Themen umschalten", async {
        e.wait("//button[@id='btn-gear']", 5).await?.click().await?;
        e.wait("//section[contains(@class,'active')]//input[@value='obsidian']", 10).await?.click().await?;
        e.until("document.documentElement.dataset.skin === 'obsidian'", 5).await?;
        e.shot("09-obsidian").await;
        e.wait("//section[contains(@class,'active')]//input[@value='grass']", 10).await?.click().await?;
        e.until("document.documentElement.dataset.skin === undefined", 5).await?;
        let before = e.js("return document.documentElement.dataset.theme || '';").await?;
        e.wait("//button[@id='btn-theme']", 5).await?.click().await?;
        e.until(&format!("(document.documentElement.dataset.theme || '') !== {before}"), 5).await
    });
    schritt!(e, "Keine Fehler in der Oberfläche", async {
        let errs = e.errors().await;
        if errs.is_empty() { Ok(()) } else { fail(format!("{} Fehler: {errs:?}", errs.len())) }
    });
    Ok(())
}
