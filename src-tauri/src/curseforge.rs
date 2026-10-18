//! CurseForge als zweite Quelle (optional, nur mit eigenem API-Schlüssel).
//!
//! Modrinth hat immer Vorrang. CurseForge wird nur zusätzlich gefragt, wenn ein
//! Schlüssel hinterlegt ist. Der Schlüssel liegt in einer eigenen Datei im
//! Konfigurationsordner, die nur der Benutzer lesen darf, und taucht nie in Logs
//! oder Fehlermeldungen auf.

use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

use crate::config;
use crate::flavor::Kind;
#[cfg(not(unix))]
use crate::util;
use crate::util::{Error, Result, err};

const API: &str = "https://api.curseforge.com";
/// Minecraft auf CurseForge (nicht in der Doku, aber seit Jahren unverändert)
pub const MINECRAFT: u32 = 432;
/// Antragsformular für Drittprogramme (verlinkt auf docs.curseforge.com/rest-api)
pub const APPLY_URL: &str = "https://forms.monday.com/forms/dce5ccb7afda9a1c21dab1a1aa1d84eb?r=use1";
pub const DOCS_URL: &str = "https://docs.curseforge.com/rest-api/";
pub const CONSOLE_URL: &str = "https://console.curseforge.com/";
/// Bedingungen von CurseForge für Drittprogramme
pub const TERMS_URL: &str = "https://support.curseforge.com/en/support/solutions/articles/9000207405-curse-forge-3rd-party-api-terms-and-conditions";

// ---------------------------------------------------------------- Schlüssel

fn key_file() -> PathBuf {
    config::state_dir().join("curseforge-schluessel")
}

pub fn key() -> Option<String> {
    fs::read_to_string(key_file())
        .ok()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
}

/// Nur die ersten und letzten Zeichen zeigen.
pub fn masked() -> Option<String> {
    key().map(|k| {
        let n = k.chars().count();
        if n <= 8 {
            "••••".into()
        } else {
            let start: String = k.chars().take(4).collect();
            let end: String = k.chars().skip(n - 4).collect();
            format!("{start}••••{end}")
        }
    })
}

pub fn save_key(k: &str) -> Result<()> {
    let k = k.trim();
    if k.is_empty() || k.len() > 200 || k.chars().any(|c| c.is_control() || c.is_whitespace()) {
        return err("Das sieht nicht wie ein API-Schlüssel aus (bitte ohne Leerzeichen einfügen).");
    }
    fs::create_dir_all(config::state_dir())?;
    let path = key_file();
    // Erst mit Rechten 600 anlegen, dann schreiben
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let tmp = path.with_extension("tmp");
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&tmp)?;
        f.write_all(k.as_bytes())?;
        f.sync_all()?;
        fs::rename(&tmp, &path)?;
    }
    #[cfg(not(unix))]
    util::write_atomic(&path, k.as_bytes())?;
    Ok(())
}

pub fn delete_key() -> Result<()> {
    let p = key_file();
    if p.exists() {
        fs::remove_file(p)?;
    }
    Ok(())
}

// ---------------------------------------------------------------- Anfragen

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .user_agent(crate::net::USER_AGENT)
        .timeout_connect(Some(Duration::from_secs(15)))
        .timeout_global(Some(Duration::from_secs(60)))
        .http_status_as_error(false)
        .build()
        .into()
});

/// Fehlermeldung ohne Schlüssel (der steht nur im Header, aber sicher ist sicher).
fn clean(msg: String, k: &str) -> Error {
    Error(if k.is_empty() {
        msg
    } else {
        msg.replace(k, "••••")
    })
}

/// Im Testmodus kann ein nachgebauter CurseForge-Server benutzt werden.
fn api_base() -> String {
    match (config::test_home(), std::env::var("VAPAFANE_CF_API")) {
        (Some(_), Ok(u)) if u.starts_with("http://127.0.0.1:") => u,
        _ => API.to_string(),
    }
}

fn request(k: &str, method: &str, path: &str, query: &[(&str, String)], body: Option<&Value>) -> Result<Value> {
    let url = format!("{}{path}", api_base());
    // Bei „zu vielen Anfragen“ wartet send_polite und versucht es erneut
    let mut resp = crate::net::send_polite(&url, || {
        if method == "POST" {
            let mut r = AGENT
                .post(&url)
                .header("x-api-key", k)
                .header("Accept", "application/json");
            for (q, v) in query {
                r = r.query(*q, v);
            }
            r.send_json(body.cloned().unwrap_or(Value::Null))
        } else {
            let mut r = AGENT
                .get(&url)
                .header("x-api-key", k)
                .header("Accept", "application/json");
            for (q, v) in query {
                r = r.query(*q, v);
            }
            r.call()
        }
    })
    .map_err(|e| clean(format!("CurseForge: {e}"), k))?;
    match resp.status().as_u16() {
        200 => resp
            .body_mut()
            .with_config()
            .limit(32 * 1024 * 1024)
            .read_json()
            .map_err(|e| clean(format!("Ungültige Antwort von CurseForge: {e}"), k)),
        401 | 403 => err("CurseForge lehnt den API-Schlüssel ab (ungültig oder noch nicht freigeschaltet)."),
        404 => err("Bei CurseForge nicht gefunden."),
        429 => err("CurseForge meldet zu viele Anfragen – bitte kurz warten."),
        code => err(format!("CurseForge antwortet mit Fehler {code}.")),
    }
}

fn get(path: &str, query: &[(&str, String)]) -> Result<Value> {
    let k = key().ok_or_else(|| Error("Kein CurseForge-Schlüssel hinterlegt.".into()))?;
    request(&k, "GET", path, query, None)
}

fn post(path: &str, body: &Value) -> Result<Value> {
    let k = key().ok_or_else(|| Error("Kein CurseForge-Schlüssel hinterlegt.".into()))?;
    request(&k, "POST", path, &[], Some(body))
}

/// Schlüssel ausprobieren (vor dem Speichern).
pub fn test_key(k: &str) -> Result<()> {
    request(k.trim(), "GET", "/v1/games", &[("pageSize", "1".into())], None).map(|_| ())
}

// ---------------------------------------------------------------- Klassen und Loader

/// Klassen-IDs (Mods, Bukkit-Plugins, Datapacks) – zur Laufzeit von CurseForge gelesen.
static CLASSES: Mutex<Option<HashMap<String, u64>>> = Mutex::new(None);

fn class_id(slug: &str) -> Result<u64> {
    let mut g = CLASSES.lock().unwrap();
    if g.is_none() {
        let v = get(
            "/v1/categories",
            &[("gameId", MINECRAFT.to_string()), ("classesOnly", "true".into())],
        )?;
        let map = v
            .get("data")
            .and_then(|d| d.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|c| Some((c.get("slug")?.as_str()?.to_string(), c.get("id")?.as_u64()?)))
                    .collect()
            })
            .unwrap_or_default();
        *g = Some(map);
    }
    g.as_ref()
        .and_then(|m| m.get(slug).copied())
        .ok_or_else(|| Error(format!("CurseForge-Bereich „{slug}“ nicht gefunden.")))
}

/// Was auf CurseForge gesucht wird: (Klassen-Slug, Loader-Typ)
pub fn section(kind: Kind, datapacks: bool) -> (&'static str, Option<u32>) {
    if datapacks || kind == Kind::Vanilla {
        return ("data-packs", None);
    }
    match kind {
        Kind::Fabric => ("mc-mods", Some(4)),
        Kind::NeoForge => ("mc-mods", Some(6)),
        Kind::Paper => ("bukkit-plugins", None),
        Kind::Vanilla => ("data-packs", None),
    }
}

// ---------------------------------------------------------------- Suche

/// Suche auf CurseForge (Popularität absteigend). Liefert die Rohdaten der Mods.
pub fn search(kind: Kind, datapacks: bool, mc: &str, query: &str, index: u32) -> Result<(Vec<Value>, u64)> {
    let (class, loader) = section(kind, datapacks);
    let mut q = vec![
        ("gameId", MINECRAFT.to_string()),
        ("classId", class_id(class)?.to_string()),
        ("gameVersion", mc.to_string()),
        ("sortField", "2".into()), // 2 = Popularity
        ("sortOrder", "desc".into()),
        ("index", index.to_string()),
        ("pageSize", "20".into()),
    ];
    if let Some(l) = loader {
        q.push(("modLoaderType", l.to_string()));
    }
    if !query.trim().is_empty() {
        q.push(("searchFilter", query.trim().to_string()));
    }
    let v = get("/v1/mods/search", &q)?;
    let total = v
        .pointer("/pagination/totalCount")
        .and_then(|t| t.as_u64())
        .unwrap_or(0);
    Ok((
        v.get("data").and_then(|d| d.as_array()).cloned().unwrap_or_default(),
        total,
    ))
}

/// Eine bestimmte Datei einer Mod.
pub fn get_file(mod_id: u64, file_id: u64) -> Result<Value> {
    Ok(get(&format!("/v1/mods/{mod_id}/files/{file_id}"), &[])?
        .get("data")
        .cloned()
        .unwrap_or(Value::Null))
}

pub fn get_mod(id: u64) -> Result<Value> {
    Ok(get(&format!("/v1/mods/{id}"), &[])?
        .get("data")
        .cloned()
        .unwrap_or(Value::Null))
}

/// SHA1 einer CurseForge-Datei (Algo 1 = Sha1).
pub fn file_sha1(file: &Value) -> Option<String> {
    file.get("hashes")?
        .as_array()?
        .iter()
        .find(|h| h.get("algo").and_then(|a| a.as_u64()) == Some(1))
        .and_then(|h| h.get("value")?.as_str().map(str::to_string))
}

/// Neueste passende Datei einer Mod (Release bevorzugt).
pub fn best_file(mod_id: u64, kind: Kind, datapacks: bool, mc: &str) -> Result<Option<Value>> {
    let (_, loader) = section(kind, datapacks);
    let mut q = vec![("gameVersion", mc.to_string()), ("pageSize", "50".into())];
    if let Some(l) = loader {
        q.push(("modLoaderType", l.to_string()));
    }
    let v = get(&format!("/v1/mods/{mod_id}/files"), &q)?;
    let mut files: Vec<Value> = v
        .get("data")
        .and_then(|d| d.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|f| f.get("isAvailable").and_then(|a| a.as_bool()).unwrap_or(true))
        .filter(|f| !f.get("isServerPack").and_then(|a| a.as_bool()).unwrap_or(false))
        .collect();
    files.sort_by(|a, b| {
        let d = |f: &Value| f.get("fileDate").and_then(|d| d.as_str()).unwrap_or("").to_string();
        d(b).cmp(&d(a))
    });
    let release = files
        .iter()
        .find(|f| f.get("releaseType").and_then(|r| r.as_u64()) == Some(1));
    Ok(release.or(files.first()).cloned())
}

/// Download-Adresse; None, wenn der Autor Downloads über Drittprogramme nicht erlaubt.
pub fn download_url(file: &Value) -> Option<String> {
    file.get("downloadUrl")
        .and_then(|u| u.as_str())
        .filter(|u| u.starts_with("https://") || (config::test_home().is_some() && u.starts_with("http://127.0.0.1:")))
        .map(str::to_string)
}

/// Seite der Datei im Browser (für den Download von Hand).
pub fn file_page(m: &Value, file: &Value) -> String {
    let base = m
        .pointer("/links/websiteUrl")
        .and_then(|u| u.as_str())
        .unwrap_or("https://www.curseforge.com/minecraft")
        .trim_end_matches('/')
        .to_string();
    match file.get("id").and_then(|i| i.as_u64()) {
        Some(id) => format!("{base}/files/{id}"),
        None => base,
    }
}

/// Pflicht-Abhängigkeiten (RelationType 3 = RequiredDependency).
pub fn required_deps(file: &Value) -> Vec<u64> {
    file.get("dependencies")
        .and_then(|d| d.as_array())
        .map(|a| {
            a.iter()
                .filter(|d| d.get("relationType").and_then(|r| r.as_u64()) == Some(3))
                .filter_map(|d| d.get("modId").and_then(|m| m.as_u64()))
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------- Fingerprints

/// CurseForge-Fingerprint: MurmurHash2 (32 Bit, Seed 1) über die Datei ohne
/// Leerraum-Bytes (Tab, LF, CR, Leerzeichen).
pub fn fingerprint(path: &Path) -> Result<u32> {
    let data: Vec<u8> = fs::read(path)?
        .into_iter()
        .filter(|b| !matches!(b, 9 | 10 | 13 | 32))
        .collect();
    Ok(murmur2(&data, 1))
}

fn murmur2(data: &[u8], seed: u32) -> u32 {
    const M: u32 = 0x5bd1_e995;
    let mut h = seed ^ (data.len() as u32);
    let (chunks, rest) = data.as_chunks::<4>();
    for c in chunks {
        let mut k = u32::from_le_bytes(*c);
        k = k.wrapping_mul(M);
        k ^= k >> 24;
        k = k.wrapping_mul(M);
        h = h.wrapping_mul(M);
        h ^= k;
    }
    if rest.len() >= 3 {
        h ^= (rest[2] as u32) << 16;
    }
    if rest.len() >= 2 {
        h ^= (rest[1] as u32) << 8;
    }
    if !rest.is_empty() {
        h ^= rest[0] as u32;
        h = h.wrapping_mul(M);
    }
    h ^= h >> 13;
    h = h.wrapping_mul(M);
    h ^= h >> 15;
    h
}

/// Fingerprints → passende CurseForge-Dateien (exactMatches).
pub fn match_fingerprints(prints: &[u32]) -> Result<HashMap<u32, Value>> {
    if prints.is_empty() {
        return Ok(HashMap::new());
    }
    let v = post(
        &format!("/v1/fingerprints/{MINECRAFT}"),
        &json!({"fingerprints": prints}),
    )?;
    let mut out = HashMap::new();
    for m in v
        .pointer("/data/exactMatches")
        .and_then(|e| e.as_array())
        .cloned()
        .unwrap_or_default()
    {
        if let Some(fp) = m.pointer("/file/fileFingerprint").and_then(|f| f.as_u64()) {
            out.insert(fp as u32, m);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Leere Eingabe von Hand nachgerechnet: h = 1; h *= M; h ^= h >> 15.
    #[test]
    fn murmur2_empty_input() {
        assert_eq!(murmur2(b"", 0), 0);
        assert_eq!(murmur2(b"", 1), 0x5bd1_5e36);
    }

    #[test]
    fn whitespace_does_not_change_fingerprint() {
        let dir = std::env::temp_dir().join(format!("vapafane-fp-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a"), b"ab c\nd\te").unwrap();
        fs::write(dir.join("b"), b"abcde").unwrap();
        assert_eq!(
            fingerprint(&dir.join("a")).unwrap(),
            fingerprint(&dir.join("b")).unwrap()
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn key_is_masked() {
        assert_eq!(clean("Fehler mit abc123".into(), "abc123").0, "Fehler mit ••••");
    }
}
