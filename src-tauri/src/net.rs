//! Internet: JSON abrufen, Dateien mit Prüfsumme laden, Fabric- und Mojang-Infos.

use serde_json::Value;
use sha1::Digest;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::LazyLock;
use std::time::Duration;

use crate::util::{Error, Result, err};

/// Kennung bei allen Anfragen – mit Kontaktadresse, wie PaperMC und Modrinth es wünschen.
pub const USER_AGENT: &str = concat!(
    "Debong93/Vapafane/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/Debong93/Vapafane)"
);

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .user_agent(USER_AGENT)
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_global(Some(Duration::from_secs(900)))
        .http_status_as_error(false)
        .build()
        .into()
});

fn net_err(e: ureq::Error) -> Error {
    match e {
        ureq::Error::StatusCode(code) => Error(format!("Server antwortet mit Fehler {code}")),
        other => Error(format!("Keine Verbindung: {other}")),
    }
}

// ---------------------------------------------------------------- Rücksicht bei vielen Anfragen

type Response = ureq::http::Response<ureq::Body>;

/// Höchstens so viele Wiederholungen nach „429 Too Many Requests“.
const RETRIES_429: u32 = 3;

/// Bis wann wir einen Dienst schonen (Host → Zeitpunkt).
static PAUSE: std::sync::Mutex<Vec<(String, std::time::Instant)>> = std::sync::Mutex::new(Vec::new());

fn host_of(url: &str) -> String {
    url.split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or("")
        .to_string()
}

fn header_u64(resp: &Response, name: &str) -> Option<u64> {
    resp.headers().get(name)?.to_str().ok()?.trim().parse().ok()
}

/// Vor der Anfrage warten, falls der Dienst gebeten hat zu bremsen.
fn wait_if_paused(host: &str) {
    let until = crate::util::lock(&PAUSE)
        .iter()
        .find(|(h, _)| h == host)
        .map(|(_, t)| *t);
    if let Some(t) = until {
        let left = t.saturating_duration_since(std::time::Instant::now());
        if !left.is_zero() {
            std::thread::sleep(left.min(Duration::from_secs(60)));
        }
    }
}

fn pause(host: &str, secs: u64) {
    let until = std::time::Instant::now() + Duration::from_secs(secs.clamp(1, 60));
    let mut p = crate::util::lock(&PAUSE);
    p.retain(|(h, _)| h != host);
    p.push((host.to_string(), until));
}

/// Modrinth meldet in X-Ratelimit-Remaining/-Reset, wie viel Luft noch ist:
/// wird es knapp, warten wir bis zum Zurücksetzen (statt ins Limit zu laufen).
fn note_limits(host: &str, resp: &Response) {
    if let (Some(left), Some(reset)) = (
        header_u64(resp, "x-ratelimit-remaining"),
        header_u64(resp, "x-ratelimit-reset"),
    ) && left <= 10
    {
        pause(host, reset);
    }
}

/// Anfrage schicken; bei 429 dem Retry-After folgen (sonst 2, 4, 8 s warten) und es
/// wenige Male erneut versuchen. Andere Statuscodes gehen an den Aufrufer.
pub fn send_polite(
    url: &str,
    mut send: impl FnMut() -> std::result::Result<Response, ureq::Error>,
) -> Result<Response> {
    let host = host_of(url);
    for attempt in 0..=RETRIES_429 {
        wait_if_paused(&host);
        let resp = send().map_err(net_err)?;
        note_limits(&host, &resp);
        if resp.status().as_u16() != 429 {
            return Ok(resp);
        }
        if attempt == RETRIES_429 {
            break;
        }
        let wait = header_u64(&resp, "retry-after").unwrap_or(2u64 << attempt).clamp(1, 60);
        pause(&host, wait);
    }
    err(format!(
        "{host} meldet zu viele Anfragen. Bitte ein paar Minuten warten und es dann nochmal versuchen."
    ))
}

/// Fehlercodes (ab 400) in eine Meldung verwandeln.
fn ok_status(resp: Response) -> Result<Response> {
    let code = resp.status().as_u16();
    if code >= 400 {
        return err(format!("Server antwortet mit Fehler {code}"));
    }
    Ok(resp)
}

pub fn get_json(url: &str) -> Result<Value> {
    let mut resp = ok_status(send_polite(url, || AGENT.get(url).call())?)?;
    resp.body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_json()
        .map_err(net_err)
}

/// GET mit Abfrage-Parametern (werden korrekt kodiert).
pub fn get_json_query(url: &str, query: &[(&str, String)]) -> Result<Value> {
    let mut resp = ok_status(send_polite(url, || {
        let mut req = AGENT.get(url);
        for (k, v) in query {
            req = req.query(*k, v);
        }
        req.call()
    })?)?;
    resp.body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_json()
        .map_err(net_err)
}

pub fn post_json(url: &str, body: &Value) -> Result<Value> {
    let mut resp = ok_status(send_polite(url, || AGENT.post(url).send_json(body))?)?;
    resp.body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_json()
        .map_err(net_err)
}

/// Erwartete Prüfsummen einer Datei (Hex). Was fehlt, wird nicht geprüft.
#[derive(Clone, Debug, Default)]
pub struct Expect<'a> {
    pub sha1: Option<&'a str>,
    pub sha256: Option<&'a str>,
    pub sha512: Option<&'a str>,
}

/// Lädt eine Datei und prüft die Prüfsummen. Erst danach liegt die Datei unter `dest`.
pub fn download(url: &str, dest: &Path, expect: Expect, progress: &dyn Fn(f64)) -> Result<()> {
    let test_local = crate::config::test_home().is_some() && url.starts_with("http://127.0.0.1:");
    if !url.starts_with("https://") && !test_local {
        return err(format!("Unsichere Download-Adresse: {url}"));
    }
    let mut resp = ok_status(send_polite(url, || AGENT.get(url).call())?)?;
    let total: u64 = resp
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let tmp = dest.with_extension("part");
    if let Some(dir) = dest.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut h1 = sha1::Sha1::new();
    let mut h256 = sha2::Sha256::new();
    let mut h512 = sha2::Sha512::new();
    let res = (|| -> Result<()> {
        let mut out = fs::File::create(&tmp)?;
        let mut reader = resp.body_mut().with_config().limit(4 * 1024 * 1024 * 1024).reader();
        let mut buf = vec![0u8; 256 * 1024];
        let mut done = 0u64;
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            out.write_all(&buf[..n])?;
            h1.update(&buf[..n]);
            h256.update(&buf[..n]);
            h512.update(&buf[..n]);
            done += n as u64;
            if total > 0 {
                progress(done as f64 / total as f64);
            }
        }
        out.sync_all()?;
        Ok(())
    })();
    if let Err(e) = res {
        let _ = fs::remove_file(&tmp);
        return Err(e);
    }
    let ok = |want: Option<&str>, got: String| want.map(|w| w.eq_ignore_ascii_case(&got)).unwrap_or(true);
    if !ok(expect.sha1, hex::encode(h1.finalize()))
        || !ok(expect.sha256, hex::encode(h256.finalize()))
        || !ok(expect.sha512, hex::encode(h512.finalize()))
    {
        let _ = fs::remove_file(&tmp);
        return err(crate::sprache::t!(
            "err-pruefsumme",
            datei = dest.file_name().unwrap_or_default().to_string_lossy().into_owned()
        ));
    }
    fs::rename(&tmp, dest)?;
    Ok(())
}

/// Text-Datei laden (z. B. eine .sha1-Datei von Maven).
pub fn get_text(url: &str) -> Result<String> {
    let mut resp = ok_status(send_polite(url, || AGENT.get(url).call())?)?;
    resp.body_mut()
        .with_config()
        .limit(16 * 1024 * 1024)
        .read_to_string()
        .map_err(net_err)
}

pub fn sha1_file(path: &Path) -> Result<String> {
    hash_file::<sha1::Sha1>(path)
}

fn hash_file<D: Digest>(path: &Path) -> Result<String> {
    let mut f = fs::File::open(path)?;
    let mut h = D::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex::encode(h.finalize()))
}

// ---------------------------------------------------------------- Fabric

/// Stabile Minecraft-Versionen mit Fabric-Unterstützung, neueste zuerst.
pub fn fabric_game_versions() -> Result<Vec<String>> {
    let v = get_json("https://meta.fabricmc.net/v2/versions/game")?;
    Ok(v.as_array()
        .map(|a| {
            a.iter()
                .filter(|g| g.get("stable").and_then(|s| s.as_bool()).unwrap_or(false))
                .filter_map(|g| g.get("version").and_then(|s| s.as_str()).map(str::to_string))
                .collect()
        })
        .unwrap_or_default())
}

/// Download-Adresse der Fabric-Server-Jar und Loader-Version für eine Minecraft-Version.
pub fn fabric_server_jar_url(game: &str) -> Result<(String, String)> {
    let loaders = get_json(&format!("https://meta.fabricmc.net/v2/versions/loader/{game}"))?;
    let loaders = loaders.as_array().cloned().unwrap_or_default();
    let version_of = |l: &Value| {
        l.pointer("/loader/version")
            .and_then(|v| v.as_str())
            .map(str::to_string)
    };
    let loader = loaders
        .iter()
        .find(|l| l.pointer("/loader/stable").and_then(|s| s.as_bool()).unwrap_or(false))
        .and_then(version_of)
        .or_else(|| loaders.first().and_then(version_of))
        .ok_or_else(|| Error(crate::sprache::t!("err-kein-fabric", mc = game)))?;
    let installers = get_json("https://meta.fabricmc.net/v2/versions/installer")?;
    let installers = installers.as_array().cloned().unwrap_or_default();
    let installer = installers
        .iter()
        .find(|i| i.get("stable").and_then(|s| s.as_bool()).unwrap_or(false))
        .or(installers.first())
        .and_then(|i| i.get("version").and_then(|v| v.as_str()))
        .ok_or_else(|| Error("Kein Fabric-Installer gefunden".into()))?
        .to_string();
    Ok((
        format!("https://meta.fabricmc.net/v2/versions/loader/{game}/{loader}/{installer}/server/jar"),
        loader,
    ))
}

/// Java-Hauptversion, die eine Minecraft-Version braucht (aus Mojangs Versionsliste).
pub fn required_java(game: &str) -> Result<Option<u32>> {
    let manifest = get_json("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")?;
    let entry = manifest
        .get("versions")
        .and_then(|v| v.as_array())
        .and_then(|a| a.iter().find(|v| v.get("id").and_then(|i| i.as_str()) == Some(game)));
    let Some(url) = entry.and_then(|e| e.get("url")).and_then(|u| u.as_str()) else {
        return Ok(None);
    };
    let v = get_json(url)?;
    Ok(v.pointer("/javaVersion/majorVersion")
        .and_then(|m| m.as_u64())
        .map(|m| m as u32))
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader, Write};

    /// Mini-Server: antwortet der Reihe nach mit den gegebenen Antworten.
    fn serve(answers: Vec<&'static str>) -> (String, std::thread::JoinHandle<usize>) {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/test", l.local_addr().unwrap());
        let h = std::thread::spawn(move || {
            let mut n = 0;
            for a in answers {
                let (mut s, _) = l.accept().unwrap();
                let mut r = BufReader::new(s.try_clone().unwrap());
                let mut line = String::new();
                while r.read_line(&mut line).unwrap() > 2 {
                    line.clear();
                }
                s.write_all(a.as_bytes()).unwrap();
                n += 1;
            }
            n
        });
        (url, h)
    }

    #[test]
    fn retries_after_429_with_retry_after() {
        let (url, h) = serve(vec![
            "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 1\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}",
        ]);
        let start = std::time::Instant::now();
        let v = super::get_json(&url).unwrap();
        assert_eq!(v["ok"], true);
        assert!(
            start.elapsed() >= std::time::Duration::from_millis(900),
            "hat nicht gewartet"
        );
        assert_eq!(h.join().unwrap(), 2);
    }

    #[test]
    fn gives_up_after_few_429() {
        let a = "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 1\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let (url, h) = serve(vec![a, a, a, a]);
        let e = super::get_json(&url).unwrap_err();
        assert!(e.0.contains("zu viele Anfragen"), "{}", e.0);
        assert_eq!(h.join().unwrap(), 4);
    }

    #[test]
    fn slows_down_before_modrinth_limit() {
        let (url, h) = serve(vec![
            "HTTP/1.1 200 OK\r\nX-Ratelimit-Remaining: 3\r\nX-Ratelimit-Reset: 1\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
        ]);
        super::get_json(&url).unwrap();
        let start = std::time::Instant::now();
        super::get_json(&url).unwrap();
        assert!(
            start.elapsed() >= std::time::Duration::from_millis(900),
            "hat nicht gebremst"
        );
        assert_eq!(h.join().unwrap(), 2);
    }

    #[test]
    fn user_agent_has_contact() {
        assert!(super::USER_AGENT.contains("+https://github.com/Debong93/Vapafane"));
    }

    /// Braucht Internet: `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn fabric_meta_works() {
        let v = super::fabric_game_versions().unwrap();
        assert!(!v.is_empty());
        let (url, loader) = super::fabric_server_jar_url(&v[0]).unwrap();
        assert!(url.starts_with("https://meta.fabricmc.net/") && !loader.is_empty());
        assert!(super::required_java(&v[0]).unwrap().unwrap_or(0) >= 17);
    }
}
