//! Testumgebung: unsichtbarer Bildschirm (Xvfb), tauri-driver und ein Vapafane im Testmodus
//! mit eigenem, leerem Zuhause – die echten Daten des Nutzers bleiben unberührt.

#![deny(unsafe_code)]

use fantoccini::elements::Element;
use fantoccini::{Client, ClientBuilder, Locator};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub type Res<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn fail<T>(msg: impl Into<String>) -> Res<T> {
    Err(msg.into().into())
}

/// Gestartete Hilfsprogramme – werden beim Beenden (auch bei Fehlern) wieder beendet.
struct Procs(Vec<Child>);

impl Drop for Procs {
    fn drop(&mut self) {
        for p in self.0.iter_mut().rev() {
            let _ = p.kill();
            let _ = p.wait();
        }
        // WebKitWebDriver und das Testprogramm laufen als Kinder von tauri-driver weiter
        let _ = Command::new("pkill").args(["-x", "WebKitWebDriver"]).status();
    }
}

/// Prozesse dieses Testlaufs beenden: Minecraft-Server (laufen im Testordner) und den
/// Hintergrund-Dienst (hat VAPAFANE_HOME = Testordner). Echte Server liegen woanders.
fn kill_under(home: &Path) {
    let Ok(rd) = std::fs::read_dir("/proc") else {
        return;
    };
    let marker = format!("VAPAFANE_HOME={}", home.display());
    for e in rd.filter_map(|e| e.ok()) {
        let pid = e.file_name().to_string_lossy().into_owned();
        if !pid.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let cwd = std::fs::read_link(e.path().join("cwd")).ok();
        let env = std::fs::read(e.path().join("environ")).unwrap_or_default();
        let ours = cwd.is_some_and(|c| c.starts_with(home))
            || env.split(|b| *b == 0).any(|v| v == marker.as_bytes());
        if ours {
            let _ = Command::new("kill").arg(&pid).status();
        }
    }
}

/// Reste eines abgebrochenen Laufs beenden (nur Testprogramme, nie das installierte Vapafane).
fn cleanup(display: &str) {
    for args in [vec!["-x", "tauri-driver"], vec!["-x", "WebKitWebDriver"]] {
        let _ = Command::new("pkill").args(args).status();
    }
    let _ = Command::new("pkill").args(["-f", &format!("Xvfb {display} ")]).status();
    std::thread::sleep(Duration::from_millis(300));
}

impl Drop for Env {
    fn drop(&mut self) {
        kill_under(&self.home);
    }
}

/// Startet Xvfb + tauri-driver und räumt beim Beenden wieder auf.
pub struct Env {
    _procs: Procs,
    pub home: PathBuf,
    pub client: Client,
    pub shots: PathBuf,
}


fn wait_port(port: u16, timeout: Duration) -> bool {
    let end = Instant::now() + timeout;
    while Instant::now() < end {
        if std::net::TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    false
}

impl Env {
    /// `app`: Pfad zur Vapafane-Programmdatei; `home`: leerer Ordner für alle Daten.
    pub async fn start(app: &Path, home: &Path, display: &str) -> Res<Env> {
        std::fs::create_dir_all(home)?;
        let shots = home.join("bilder");
        std::fs::create_dir_all(&shots)?;
        cleanup(display);
        let mut procs = Procs(vec![]);
        procs.0.push(
            Command::new("Xvfb")
                .args([display, "-screen", "0", "1280x900x24", "-nolisten", "tcp"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
        );
        std::thread::sleep(Duration::from_millis(800));
        let log = std::fs::File::create(home.join("driver.log"))?;
        procs.0.push(
            Command::new("tauri-driver")
                .env("DISPLAY", display)
                .env("HOME", home)
                .env("XDG_CONFIG_HOME", home.join(".config"))
                .env("XDG_DATA_HOME", home.join(".local/share"))
                .env("XDG_CACHE_HOME", home.join(".cache"))
                .env("VAPAFANE_HOME", home)
                .env_remove("DBUS_SESSION_BUS_ADDRESS")
                .stdout(log.try_clone()?)
                .stderr(log)
                .spawn()?,
        );
        if !wait_port(4444, Duration::from_secs(15)) {
            return fail("tauri-driver antwortet nicht auf Port 4444");
        }
        let mut caps = serde_json::Map::new();
        caps.insert("tauri:options".into(), serde_json::json!({ "application": app }));
        caps.insert("browserName".into(), serde_json::json!("wry"));
        let client = ClientBuilder::rustls()?.capabilities(caps).connect("http://127.0.0.1:4444").await?;
        Ok(Env { _procs: procs, home: home.to_path_buf(), client, shots })
    }

    /// Bildschirmfoto (für Fehlersuche und Bericht).
    pub async fn shot(&self, name: &str) {
        if let Ok(png) = self.client.screenshot().await {
            let _ = std::fs::write(self.shots.join(format!("{name}.png")), png);
        }
    }

    /// Warten, bis ein Element da ist (XPath).
    pub async fn wait(&self, xpath: &str, secs: u64) -> Res<Element> {
        let end = Instant::now() + Duration::from_secs(secs);
        loop {
            // Seiten bleiben im Hintergrund bestehen – nur ein sichtbarer Treffer zählt
            if let Ok(list) = self.client.find_all(Locator::XPath(xpath)).await {
                for e in list {
                    if e.is_displayed().await.unwrap_or(false) {
                        return Ok(e);
                    }
                }
            }
            if Instant::now() > end {
                return fail(format!("Nicht gefunden nach {secs} s: {xpath}"));
            }
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    }

    /// Warten, bis eine Bedingung im Fenster (JavaScript-Ausdruck) wahr ist.
    pub async fn until(&self, js: &str, secs: u64) -> Res<()> {
        let end = Instant::now() + Duration::from_secs(secs);
        loop {
            if let Ok(v) = self.client.execute(&format!("return !!({js});"), vec![]).await
                && v.as_bool() == Some(true)
            {
                return Ok(());
            }
            if Instant::now() > end {
                return fail(format!("Bedingung nach {secs} s nicht erfüllt: {js}"));
            }
            tokio::time::sleep(Duration::from_millis(400)).await;
        }
    }

    /// Warten, bis die gerade sichtbare Seite (bzw. ein offener Dialog) diesen Text enthält.
    pub async fn sees(&self, text: &str, secs: u64) -> Res<()> {
        let t = serde_json::to_string(text)?;
        self.until(&format!("Array.from(document.querySelectorAll('section.tab.active, .modal, .topbar, .taskbar')).some(s => s.innerText.includes({t}))"), secs).await
    }

    pub async fn text(&self, xpath: &str) -> Res<String> {
        Ok(self.wait(xpath, 10).await?.text().await?)
    }

    /// Knopf mit genau diesem Text anklicken (nur sichtbare, aktive Knöpfe).
    pub async fn click_button(&self, label: &str) -> Res<()> {
        let xp = format!("//button[normalize-space()={}][not(@disabled)]", lit(label));
        let end = Instant::now() + Duration::from_secs(20);
        loop {
            if let Ok(list) = self.client.find_all(Locator::XPath(&xp)).await {
                for b in list {
                    if b.is_displayed().await.unwrap_or(false) {
                        b.click().await?;
                        return Ok(());
                    }
                }
            }
            if Instant::now() > end {
                return fail(format!("Knopf „{label}“ nicht gefunden oder gesperrt"));
            }
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    }

    /// Text der gesamten Seite.
    pub async fn page_text(&self) -> String {
        self.client
            .execute("return document.body.innerText;", vec![])
            .await
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_default()
    }

    pub async fn js(&self, code: &str) -> Res<serde_json::Value> {
        Ok(self.client.execute(code, vec![]).await?)
    }
}

impl Env {
    /// Eingabefeld (per XPath) leeren und Text eintippen.
    pub async fn type_into(&self, xpath: &str, text: &str) -> Res<()> {
        let el = self.wait(xpath, 15).await?;
        el.clear().await?;
        el.send_keys(text).await?;
        Ok(())
    }

    /// Auswahlliste (per XPath) auf einen Wert stellen.
    pub async fn select(&self, xpath: &str, value: &str) -> Res<()> {
        let el = self.wait(xpath, 15).await?;
        el.select_by_value(value).await?;
        Ok(())
    }

    /// Fehler der Oberfläche mitschreiben (Rust-Abstürze landen in console.error).
    pub async fn watch_errors(&self) -> Res<()> {
        self.js("window.__errs = window.__errs || []; if (!window.__w) { window.__w = 1; const o = console.error; console.error = (...a) => { window.__errs.push(String(a[0]).slice(0, 400)); o(...a); }; window.addEventListener('error', (e) => window.__errs.push('ERR ' + e.message)); } return 1;").await?;
        Ok(())
    }

    pub async fn errors(&self) -> Vec<String> {
        self.js("return window.__errs || [];")
            .await
            .ok()
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()
    }
}

/// XPath-Zeichenkette sicher in Anführungszeichen.
pub fn lit(s: &str) -> String {
    if !s.contains('\'') {
        format!("'{s}'")
    } else {
        format!("concat('{}')", s.replace('\'', "',\"'\",'"))
    }
}
