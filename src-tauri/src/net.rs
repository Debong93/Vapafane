//! Internet: JSON abrufen, Dateien mit Prüfsumme laden, Fabric- und Mojang-Infos.

use serde_json::Value;
use sha1::Digest;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::LazyLock;
use std::time::Duration;

use crate::util::{Error, Result, err};

const USER_AGENT: &str = concat!(
    "Debong93/Redcraftserver/",
    env!("CARGO_PKG_VERSION"),
    " (private Minecraft-Serververwaltung)"
);

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .user_agent(USER_AGENT)
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_global(Some(Duration::from_secs(900)))
        .build()
        .into()
});

fn net_err(e: ureq::Error) -> Error {
    match e {
        ureq::Error::StatusCode(code) => Error(format!("Server antwortet mit Fehler {code}")),
        other => Error(format!("Keine Verbindung: {other}")),
    }
}

pub fn get_json(url: &str) -> Result<Value> {
    let mut resp = AGENT.get(url).call().map_err(net_err)?;
    resp.body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_json()
        .map_err(net_err)
}

/// GET mit Abfrage-Parametern (werden korrekt kodiert).
pub fn get_json_query(url: &str, query: &[(&str, String)]) -> Result<Value> {
    let mut req = AGENT.get(url);
    for (k, v) in query {
        req = req.query(*k, v);
    }
    let mut resp = req.call().map_err(net_err)?;
    resp.body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_json()
        .map_err(net_err)
}

pub fn post_json(url: &str, body: &Value) -> Result<Value> {
    let mut resp = AGENT.post(url).send_json(body).map_err(net_err)?;
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
    if !url.starts_with("https://") {
        return err(format!("Unsichere Download-Adresse: {url}"));
    }
    let mut resp = AGENT.get(url).call().map_err(net_err)?;
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
        return err(format!(
            "Prüfsumme stimmt nicht: {}",
            dest.file_name().unwrap_or_default().to_string_lossy()
        ));
    }
    fs::rename(&tmp, dest)?;
    Ok(())
}

/// Text-Datei laden (z. B. eine .sha1-Datei von Maven).
pub fn get_text(url: &str) -> Result<String> {
    let mut resp = AGENT.get(url).call().map_err(net_err)?;
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
        .ok_or_else(|| Error(format!("Kein Fabric-Loader für Minecraft {game} gefunden")))?;
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
