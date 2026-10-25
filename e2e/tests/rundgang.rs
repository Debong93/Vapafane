//! Rundgang durch alle Funktionen von Vapafane im Testmodus – bedient das echte Programm.
//! Läuft nur auf Wunsch (braucht Xvfb, WebKitWebDriver, tauri-driver, Java und Internet):
//!
//!     cargo test --test rundgang -- --ignored --nocapture
//!
//! Programm: VAPAFANE_APP (Standard: ../src-tauri/target/debug/vapafane).
//! Alle Daten landen in einem eigenen, leeren Ordner – echte Server bleiben unberührt.

use std::path::{Path, PathBuf};
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

/// Server-Liste im Format von Minecraft (unkomprimiertes NBT).
fn servers_dat(entries: &[(&str, &str)]) -> Vec<u8> {
    fn s(out: &mut Vec<u8>, t: &str) {
        out.extend_from_slice(&(t.len() as u16).to_be_bytes());
        out.extend_from_slice(t.as_bytes());
    }
    let mut b = vec![10u8];
    s(&mut b, "");
    b.push(9);
    s(&mut b, "servers");
    b.push(10);
    b.extend_from_slice(&(entries.len() as i32).to_be_bytes());
    for (name, ip) in entries {
        b.push(8);
        s(&mut b, "name");
        s(&mut b, name);
        b.push(8);
        s(&mut b, "ip");
        s(&mut b, ip);
        b.push(0);
    }
    b.push(0);
    b
}

fn put(dir: &Path, rel: &str, data: &[u8]) -> Res<()> {
    let p = dir.join(rel);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(p, data)?;
    Ok(())
}

/// Eine Instanz der Modrinth-App, wie sie ein Spieler schon hat (nur im Test-HOME).
fn modrinth_instance(home: &Path, name: &str) -> PathBuf {
    home.join(".local/share/ModrinthApp/profiles").join(name)
}

fn has(hay: &[u8], needle: &str) -> bool {
    hay.windows(needle.len()).any(|w| w == needle.as_bytes())
}

/// Dateien im Modrinth-Paket.
fn pack_entries(pack: &Path) -> Res<Vec<String>> {
    let mut z = zip::ZipArchive::new(std::fs::File::open(pack)?)?;
    let mut names = vec![];
    for i in 0..z.len() {
        names.push(z.by_index(i)?.name().to_string());
    }
    names.sort();
    Ok(names)
}

fn pack_file(pack: &Path, name: &str) -> Res<Vec<u8>> {
    let mut z = zip::ZipArchive::new(std::fs::File::open(pack)?)?;
    let mut out = vec![];
    std::io::Read::read_to_end(&mut z.by_name(name)?, &mut out)?;
    Ok(out)
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
    // Eine Instanz, die der Spieler in der Modrinth-App schon hat (nur im Test-HOME)
    {
        let inst = modrinth_instance(&e.home, "Meine Instanz");
        put(&inst, "options.txt", b"version:4189\nguiScale:3\nlang:de_de\n")?;
        put(&inst, "config/sodium-options.json", b"{\"e2e\":true}")?;
        put(&inst, "shaderpacks/E2E-Shader.zip", b"zip")?;
        put(&inst, "saves/Welt/level.dat", b"level")?;
        put(&inst, "mods/fremd.jar", b"jar")?;
        put(&inst, "servers.dat", &servers_dat(&[("Freund", "freund.example:25565")]))?;
    }

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
    // Vor dem Anlegen: Belegt schon ein anderer Server Port 25565? (Danach lauscht dort evtl. die
    // Erreichbarkeits-Prüfung des neuen Servers selbst.)
    let port_taken = std::net::TcpStream::connect("127.0.0.1:25565").is_ok();
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
        // Neue Server haben die Whitelist an – solange sie leer ist, warnt die Übersicht
        e.sees("aber noch leer", 30).await?;
        // Ist Port 25565 schon belegt (z. B. ein anderer Server), bekommt der neue einen freien
        if port_taken {
            e.sees("Port 2556", 30).await?;
            e.until("!document.querySelector('section.tab.active').innerText.includes('Port 25565')", 5).await?;
        }
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
        // Der Server fragt Mojang nach dem Namen – bei zu vielen Anfragen (HTTP 429) später nochmal
        let notch = "//div[@class='item']/div[normalize-space()='Notch']";
        let mut found = false;
        for _ in 0..4 {
            e.type_into("//input[@placeholder='Minecraft-Name']", "Notch").await?;
            e.click_button("Hinzufügen").await?;
            if e.wait(notch, 20).await.is_ok() {
                found = true;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        }
        if !found {
            return fail("Notch ließ sich nicht auf die Whitelist setzen (Mojang antwortet nicht)");
        }
        page(e, "Übersicht").await?;
        e.until("!document.querySelector('section.tab.active').innerText.includes('aber noch leer')", 30).await?;
        // Bild für die README: laufender Server (Werte der Karten abwarten)
        e.sees("20,0", 30).await?;
        e.shot("readme-1-uebersicht").await;
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
    schritt!(e, "Datapack installieren und entfernen", async {
        page(e, "Mods/Plugins").await?;
        e.click_button("Datapacks hinzufügen …").await?;
        e.wait("//div[contains(@class,'catalog-list')]//div[@class='hit']", 60).await?;
        // Bilder der Einträge laden (vom Modrinth-Server)
        e.until("Array.from(document.querySelectorAll('.catalog-list .hit img')).slice(0, 3).every(i => i.complete && i.naturalWidth > 0)", 30).await?;
        e.shot("07-katalog").await;
        e.shot("readme-3-mods").await;
        // Der Titel führt zur Seite der Mod
        e.wait("(//div[contains(@class,'catalog-list')]//div[@class='hit'])[1]//div[@class='t']/a[contains(.,'↗')]", 10).await?;
        e.click_button("Installieren").await?;
        e.wait("//button[normalize-space()='Installiert']", 90).await?;
        e.click_button("Schließen").await?;
        // Neue Zeile mit Symbol, Kurzbeschreibung und Link zur Seite
        let zeile = "//section[@id='tab-mods']//div[contains(@class,'mod-row')]";
        e.wait(zeile, 20).await?;
        e.until("(() => { const i = document.querySelector('#tab-mods .mod-row img.mod-icon'); return i && i.complete && i.naturalWidth > 0; })()", 60).await?;
        e.wait(&format!("{zeile}//button[contains(.,'Seite')]"), 20).await?;
        e.until("(document.querySelector('#tab-mods .mod-row .mod-desc')?.innerText || '').length > 10", 20).await?;
        e.shot("07b-mods-liste").await;
        // Endgültig entfernen
        e.js("document.querySelector('#tab-mods .mod-row button[aria-label]').click()").await?;
        e.sees("entfernen?", 15).await?;
        e.shot("07c-entfernen").await;
        e.click_button("Entfernen").await?;
        e.sees("Keine Datapacks in der Welt.", 20).await
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
        e.click_button("Schließen").await?;
        // Bild für die README: Einladung einlösen, solange der Server läuft (Modrinth-App als Ziel,
        // Einstellungen aus der bisherigen Instanz übernehmen)
        e.click_button("Client").await?;
        e.type_into("//textarea[@id='cl-code']", &code).await?;
        e.click_button("Einladung prüfen").await?;
        e.wait("//span[contains(@class,'pill') and starts-with(normalize-space(),'Online')]", 60).await?;
        e.wait("//label[input[@name='cl-dest'] and contains(.,'Modrinth-App')]", 10).await?.click().await?;
        e.wait("//select[@id='cl-take']/option[contains(.,'Modrinth-App: Meine Instanz')]", 20).await?;
        let inst = modrinth_instance(&e.home, "Meine Instanz");
        e.select("//select[@id='cl-take']", &inst.to_string_lossy()).await?;
        e.wait("//div[@class='take-parts']", 10).await?;
        e.js("document.querySelector('.preview-head').scrollIntoView({block: 'start'}); return 1;").await?;
        e.shot("readme-2-client").await;
        e.click_button("Server").await?;
        e.wait("//div[@class='status-big' and normalize-space()='Läuft']", 10).await?;
        Ok(())
    });
    schritt!(e, "Sprache auf Englisch und zurück", async {
        // Umschalten in den Client-Einstellungen; die Oberfläche lädt neu
        e.click_button("Client").await?;
        e.wait("//button[@id='btn-gear']", 5).await?.click().await?;
        e.select("//select[@id='sprache-select']", "en").await?;
        e.wait("//h2[normalize-space()='Redeem invite']", 20).await?;
        e.click_button("Server").await?;
        e.wait("//div[@class='status-big' and normalize-space()='Running']", 20).await?;
        e.sees("Players online", 10).await?;
        // Bild für die englische README
        e.shot("readme-en-uebersicht").await;
        // und zurück auf Deutsch
        e.wait("//button[@id='btn-gear']", 5).await?.click().await?;
        e.select("//select[@id='sprache-select']", "de").await?;
        e.wait("//h2[normalize-space()='Einladung einlösen']", 20).await?;
        e.click_button("Server").await?;
        e.wait("//div[@class='status-big' and normalize-space()='Läuft']", 20).await?;
        Ok(())
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
    let inst = modrinth_instance(&e.home, "Meine Instanz");
    let clients = e.home.join(".local/share/vapafane/clients/E2E");
    let pack = clients.join("E2E.mrpack");
    schritt!(e, "Client: Einstellungen aus einer Modrinth-Instanz übernehmen", async {
        e.click_button("Client").await?;
        e.type_into("//textarea[@id='cl-code']", &code).await?;
        e.click_button("Einladung prüfen").await?;
        e.wait("//span[@class='big' and normalize-space()='E2E']", 60).await?;
        e.wait("//label[input[@name='cl-dest'] and contains(.,'Minecraft-Launcher')]", 10).await?.click().await?;
        e.wait("//select[@id='cl-take']/option[contains(.,'Modrinth-App: Meine Instanz')]", 20).await?;
        e.select("//select[@id='cl-take']", &inst.to_string_lossy()).await?;
        // alles außer Welten ist vorausgewählt
        e.until("(() => { const l = Array.from(document.querySelectorAll('.take-parts label')); \
            const on = (t) => l.find(x => x.innerText.includes(t)).querySelector('input').checked; \
            return l.length === 5 && on('Optionen') && on('Shader') && on('Server-Liste') && !on('Welten'); })()", 10).await?;
        e.shot("08-client-uebernehmen").await;
        e.click_button("Einrichten").await?;
        e.wait("//p[contains(.,'Eingerichtet')]", 600).await?;
        e.sees("Aus „Modrinth-App: Meine Instanz“ übernommen: 3 Dateien.", 10).await?;
        e.wait("//div[@class='item']//b[normalize-space()='E2E']", 30).await?;
        e.js("document.querySelector('.take') && document.querySelector('.take').scrollIntoView(); return 1;").await?;
        e.shot("08-client").await;
        // Im Spielordner: Optionen, Mod-Einstellungen, Shader; keine Welt, keine fremde Mod
        if std::fs::read(clients.join("options.txt"))? != b"version:4189\nguiScale:3\nlang:de_de\n" {
            return fail("options.txt wurde nicht übernommen");
        }
        if !clients.join("config/sodium-options.json").is_file() || !clients.join("shaderpacks/E2E-Shader.zip").is_file() {
            return fail("Mod-Einstellungen oder Shader fehlen");
        }
        if clients.join("saves").exists() || clients.join("mods/fremd.jar").exists() {
            return fail("Welt oder fremde Mod wurde mitkopiert");
        }
        let servers = std::fs::read(clients.join("servers.dat"))?;
        if !has(&servers, "freund.example") || !has(&servers, PORT) {
            return fail("Server-Liste: eigener Server oder der des Freundes fehlt");
        }
        // Die Quelle bleibt unverändert
        if std::fs::read(inst.join("options.txt"))? != b"version:4189\nguiScale:3\nlang:de_de\n" {
            return fail("Quelle wurde verändert");
        }
        if !clients.join("shaderpacks").is_dir() || !clients.join("resourcepacks").is_dir() {
            return fail("Ordner shaderpacks/resourcepacks fehlen");
        }
        // Anleitung „Shader & Extras“ für den Launcher
        e.click_button("Shader & Extras – so geht's").await?;
        e.wait("//h2[normalize-space()='Shader & Extras hinzufügen']", 10).await?;
        // Vanilla-Server: Ressourcenpakete ja, Shader nicht
        e.sees("resourcepacks", 5).await?;
        e.sees("aber keine Shader", 5).await?;
        e.shot("08c-extras-launcher").await;
        e.wait("//div[@class='modal-buttons']/button[normalize-space()='Schließen']", 5).await?.click().await?;
        Ok(())
    });
    schritt!(e, "Client: Modrinth-App, neue Instanz mit Einstellungen", async {
        e.click_button("Einladung prüfen").await?;
        e.wait("//span[@class='big' and normalize-space()='E2E']", 60).await?;
        e.wait("//label[input[@name='cl-dest'] and contains(.,'Modrinth-App')]", 10).await?.click().await?;
        e.wait("//select[@id='cl-take']/option[contains(.,'Modrinth-App: Meine Instanz')]", 20).await?;
        e.select("//select[@id='cl-take']", &inst.to_string_lossy()).await?;
        e.click_button("Einrichten").await?;
        e.sees("sind im Paket", 300).await?;
        let names = pack_entries(&pack)?;
        for want in ["modrinth.index.json", "overrides/options.txt", "overrides/config/sodium-options.json",
            "overrides/shaderpacks/E2E-Shader.zip", "overrides/servers.dat"] {
            if !names.iter().any(|n| n == want) {
                return fail(format!("{want} fehlt im Paket: {names:?}"));
            }
        }
        if names.iter().any(|n| n.starts_with("overrides/saves/") || n.contains("fremd.jar")) {
            return fail(format!("Welt oder fremde Mod im Paket: {names:?}"));
        }
        let servers = pack_file(&pack, "overrides/servers.dat")?;
        if !has(&servers, "freund.example") || !has(&servers, PORT) {
            return fail("Server-Liste im Paket unvollständig");
        }
        Ok(())
    });
    schritt!(e, "Client: vorhandene Modrinth-Instanz aktualisieren", async {
        // So legt die Modrinth-App die Instanz beim Import an: Ordner nach der Paket-Datei
        let imported = modrinth_instance(&e.home, "E2E");
        put(&imported, "options.txt", b"eigene Optionen")?;
        put(&imported, "servers.dat", &servers_dat(&[("Kumpel", "kumpel.example")]))?;
        e.click_button("Einladung prüfen").await?;
        e.wait("//span[@class='big' and normalize-space()='E2E']", 60).await?;
        e.wait("//label[input[@name='cl-dest'] and contains(.,'Modrinth-App')]", 10).await?.click().await?;
        e.wait("//label[input[@name='cl-mr'] and contains(.,'Vorhandene Instanz „E2E (Vapafane)“ aktualisieren')]", 20).await?;
        e.until("document.querySelector('#cl-take') === null", 5).await?;
        e.click_button("Einrichten").await?;
        e.sees("So aktualisierst du deine Instanz „E2E (Vapafane)“", 300).await?;
        e.sees("Modpack neu installieren", 5).await?;
        e.sees("Instanz-Einstellungen", 5).await?;
        e.js("document.querySelector('ol.steps').scrollIntoView(); return 1;").await?;
        e.shot("08b-modrinth-aktualisieren").await;
        let names = pack_entries(&pack)?;
        if names.iter().any(|n| n == "overrides/options.txt") {
            return fail("Beim Aktualisieren gehören keine Optionen ins Paket");
        }
        let servers = pack_file(&pack, "overrides/servers.dat")?;
        if !has(&servers, "kumpel.example") || !has(&servers, PORT) {
            return fail("Server-Liste der vorhandenen Instanz wurde nicht übernommen");
        }
        if std::fs::read(imported.join("options.txt"))? != b"eigene Optionen" {
            return fail("Die vorhandene Instanz wurde verändert");
        }
        // Anleitung „Shader & Extras“ für die Modrinth-App (aus „Meine Server“)
        e.wait("//div[@class='item'][.//b[normalize-space()='E2E']]//button[normalize-space()='Shader & Extras']", 10).await?.click().await?;
        e.wait("//h2[normalize-space()='Shader & Extras hinzufügen']", 10).await?;
        e.sees("Inhalte durchsuchen", 5).await?;
        e.until("document.querySelectorAll('.modal svg.sketch').length === 2", 5).await?;
        e.js("document.querySelector('.modal').scrollTop = 0; return 1;").await?;
        e.shot("08d-extras-modrinth").await;
        e.wait("//div[@class='modal-buttons']/button[normalize-space()='Schließen']", 5).await?.click().await?;
        Ok(())
    });
    schritt!(e, "Client: Aktualisieren lädt die gespeicherte Einladung", async {
        // Textfeld leeren, damit sichtbar wird, dass der Code aus der Einrichtung kommt
        e.js("const t = document.getElementById('cl-code'); t.value = ''; t.dispatchEvent(new Event('input')); return 1;").await?;
        e.wait("//div[@class='item'][.//b[normalize-space()='E2E']]//button[normalize-space()='Aktualisieren']", 10).await?.click().await?;
        e.sees("Gespeicherte Einladung von „E2E“ geladen", 20).await?;
        e.wait("//span[@class='big' and normalize-space()='E2E']", 60).await?;
        e.until("document.getElementById('cl-code').value.startsWith('VPF1-')", 5).await?;
        Ok(())
    });
    schritt!(e, "Client entfernen", async {
        let rm = "//div[@class='item'][.//b[normalize-space()='E2E']]//button[normalize-space()='Entfernen']";
        e.wait(rm, 10).await?.click().await?;
        e.wait("//div[@class='modal-buttons']/button[normalize-space()='Entfernen']", 10).await?.click().await?;
        e.wait("//div[@class='empty' and contains(.,'Noch kein Server eingerichtet')]", 30).await?;
        Ok(())
    });
    schritt!(e, "Einstellungen beim Bereichswechsel", async {
        e.wait("//button[@id='btn-gear']", 5).await?.click().await?;
        e.sees("Spielordner aller Clients", 10).await?;
        e.sees("Kontakt & Fehler melden (GitHub)", 5).await?;
        e.shot("10-client-einstellungen").await;
        // In den Einstellungen bleiben – in beide Richtungen
        e.click_button("Server").await?;
        e.sees("Einstellungen speichern", 10).await?;
        e.sees("Kontakt & Fehler melden (GitHub)", 5).await?;
        e.click_button("Client").await?;
        e.sees("Spielordner aller Clients", 10).await?;
        // Zahnrad schließt die Einstellungen wieder
        e.wait("//button[@id='btn-gear']", 5).await?.click().await?;
        e.wait("//h2[normalize-space()='Einladung einlösen']", 10).await?;
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
