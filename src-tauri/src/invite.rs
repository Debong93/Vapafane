//! Einladungen für Freunde: ein kurzer Code (`VPF1-…`) oder eine Datei (`.vapafane`).
//!
//! Inhalt: Adresse, Servername, Minecraft-Version, Loader und die Mods, die auch
//! die Spieler brauchen (Modrinth-Versionen bzw. CurseForge-Dateien, jeweils mit
//! Hash-Anfang zur Kontrolle). Keine Geheimnisse: kein RCON-Passwort, keine Pfade,
//! kein CurseForge-Schlüssel.
//!
//! Kodierung: JSON → Deflate → Base58 (nur Buchstaben und Ziffern, damit Messenger
//! nichts daran verändern und ein Doppelklick den ganzen Code markiert).

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::flavor::Kind;
use crate::mods;
use crate::net;
use crate::server;
use crate::updater;
use crate::util::{Error, Result, err};

pub const PREFIX: &str = "VPF1-";
/// So viele Hex-Zeichen des SHA1 kommen in den Code (reicht zur Kontrolle).
const HASH_LEN: usize = 12;
pub const MANIFEST: &str = "vapafane.json";

fn is_false(b: &bool) -> bool {
    !*b
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InviteMod {
    /// "mr" (Modrinth), "cf" (CurseForge) oder "file" (in der Einladungs-Datei)
    #[serde(rename = "s")]
    pub source: String,
    /// mr: Versions-ID · cf: "Projekt/Datei" · file: Dateiname
    #[serde(rename = "i")]
    pub id: String,
    /// Anfang des SHA1 der Datei
    #[serde(rename = "h")]
    pub hash: String,
    #[serde(rename = "o", default, skip_serializing_if = "is_false")]
    pub optional: bool,
    /// Name (nur bei CurseForge und Dateien – Modrinth-Namen holt der Client selbst)
    #[serde(rename = "t", default, skip_serializing_if = "String::is_empty")]
    pub title: String,
    /// CurseForge: Slug für den Link „im Browser herunterladen“
    #[serde(rename = "u", default, skip_serializing_if = "String::is_empty")]
    pub slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Invite {
    #[serde(rename = "v")]
    pub format: u32,
    #[serde(rename = "n")]
    pub name: String,
    /// host:port
    #[serde(rename = "a")]
    pub address: String,
    #[serde(rename = "m")]
    pub mc: String,
    /// fabric, neoforge, paper, vanilla
    #[serde(rename = "l")]
    pub loader: String,
    #[serde(rename = "lv", default, skip_serializing_if = "String::is_empty")]
    pub loader_version: String,
    #[serde(rename = "x", default, skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<InviteMod>,
    /// Zusätzlich die IPv6-Adresse des Servers: [IPv6]:Port (ältere Versionen ignorieren das)
    #[serde(rename = "a6", default, skip_serializing_if = "String::is_empty")]
    pub address6: String,
    /// Server ist laut letzter Prüfung nur über IPv6 erreichbar
    #[serde(rename = "o6", default, skip_serializing_if = "is_false")]
    pub only_v6: bool,
}

/// Was beim Erstellen herauskam (für die Anzeige beim Admin).
pub struct Built {
    pub invite: Invite,
    /// Server-Mods, die die Spieler nicht brauchen
    pub server_only: Vec<String>,
    /// Mods ohne Quelle (nur in der Einladungs-Datei enthalten): (Titel, Pfad)
    pub files: Vec<(String, PathBuf)>,
}

// ---------------------------------------------------------------- Kodieren

pub fn encode(inv: &Invite) -> Result<String> {
    let json = serde_json::to_vec(inv)?;
    let mut z = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
    z.write_all(&json)?;
    Ok(format!("{PREFIX}{}", bs58::encode(z.finish()?).into_string()))
}

pub fn decode(code: &str) -> Result<Invite> {
    // Messenger fügen gern Zeilenumbrüche oder Leerzeichen ein
    let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    let Some(body) = code.strip_prefix(PREFIX) else {
        if code.starts_with("VPF") {
            return err(crate::sprache::t!("inv-err-neuer"));
        }
        return err(crate::sprache::t!("inv-err-kein-code"));
    };
    let bytes = bs58::decode(body)
        .into_vec()
        .map_err(|_| Error(crate::sprache::t!("inv-err-unvollstaendig")))?;
    let mut json = Vec::new();
    flate2::read::DeflateDecoder::new(&bytes[..])
        .take(1024 * 1024)
        .read_to_end(&mut json)
        .map_err(|_| Error(crate::sprache::t!("inv-err-kaputt")))?;
    let inv: Invite = serde_json::from_slice(&json).map_err(|_| Error(crate::sprache::t!("inv-err-kaputt")))?;
    check(&inv)?;
    Ok(inv)
}

/// Inhalt auf Plausibilität prüfen (die Daten kommen von außen).
pub fn check(inv: &Invite) -> Result<()> {
    if inv.format != 1 {
        return err(crate::sprache::t!("inv-err-version"));
    }
    if Kind::parse(&inv.loader).is_none() {
        return err(crate::sprache::t!("inv-err-typ-unbekannt"));
    }
    let ok_text = |s: &str, max: usize| s.len() <= max && !s.chars().any(|c| c.is_control());
    if !ok_text(&inv.name, 80)
        || !ok_text(&inv.address, 260)
        || !ok_text(&inv.mc, 40)
        || !ok_text(&inv.loader_version, 60)
        || !(inv.address6.is_empty() || (inv.address6.starts_with('[') && ok_text(&inv.address6, 60)))
    {
        return err(crate::sprache::t!("inv-err-angaben"));
    }
    for m in &inv.mods {
        let safe_id = !m.id.is_empty()
            && m.id.len() <= 200
            && !m.id.contains("..")
            && !m.id.contains('\\')
            && !m.id.starts_with('/');
        if !matches!(m.source.as_str(), "mr" | "cf" | "file")
            || !safe_id
            || !m.hash.chars().all(|c| c.is_ascii_hexdigit())
        {
            return err(crate::sprache::t!("inv-err-mod"));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- Erstellen (Server)

/// Einladung für den Server in `dir` bauen. `address` = host:port, den Freunde benutzen.
pub fn build(dir: &Path, address: &str) -> Result<Built> {
    let info = server::detect_server(dir);
    let kind = Kind::parse(&info.kind).ok_or_else(|| Error(crate::sprache::t!("inv-err-typ")))?;
    if info.version.is_empty() {
        return err(crate::sprache::t!("err-mc-unbekannt"));
    }
    let cfg = crate::config::Config::load();
    let mut inv = Invite {
        format: 1,
        name: cfg.name_of(dir),
        address: address.trim().to_string(),
        mc: info.version.clone(),
        loader: kind.id().into(),
        loader_version: info.loader.clone(),
        mods: vec![],
        address6: crate::reach::invite_v6(dir).unwrap_or_default(),
        only_v6: crate::reach::last().is_some_and(|r| r.v6_only && r.port == crate::reach::server_port(dir)),
    };
    let mut built = Built {
        invite: inv.clone(),
        server_only: vec![],
        files: vec![],
    };
    // Paper-Plugins und Vanilla: Spieler brauchen nichts außer der Version
    if !matches!(kind, Kind::Fabric | Kind::NeoForge) {
        return Ok(built);
    }
    let sub = kind.mod_dir().unwrap_or("mods");
    let jars: Vec<(String, PathBuf, String)> = fs::read_dir(dir.join(sub))?
        .filter_map(|e| e.ok())
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .filter(|(n, p)| n.ends_with(".jar") && p.is_file())
        .filter_map(|(n, p)| net::sha1_file(&p).ok().map(|h| (n, p, h)))
        .collect();
    let hashes: Vec<String> = jars.iter().map(|(_, _, h)| h.clone()).collect();
    let known = updater::version_files(&hashes)?;
    let ids: HashSet<String> = known
        .values()
        .filter_map(|v| v.get("project_id").and_then(|p| p.as_str()).map(str::to_string))
        .collect();
    let infos = updater::project_infos(&ids)?;
    let sources = mods::sources(dir);
    for (name, path, sha1) in &jars {
        let short = sha1[..HASH_LEN].to_string();
        if let Some(v) = known.get(sha1) {
            let pid = v.get("project_id").and_then(|p| p.as_str()).unwrap_or("");
            let p = infos.get(pid);
            let client = p
                .and_then(|p| p.get("client_side"))
                .and_then(|c| c.as_str())
                .unwrap_or("required");
            let title = p
                .and_then(|p| p.get("title"))
                .and_then(|t| t.as_str())
                .unwrap_or(name)
                .to_string();
            if client == "unsupported" {
                built.server_only.push(title);
                continue;
            }
            inv.mods.push(InviteMod {
                source: "mr".into(),
                id: v.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string(),
                hash: short,
                optional: client == "optional",
                title: String::new(),
                slug: String::new(),
            });
        } else if let Some(src) = sources
            .get(&format!("{sub}/{name}"))
            .filter(|s| s.get("source").and_then(|x| x.as_str()) == Some("curseforge"))
        {
            let pid = src.get("id").and_then(|x| x.as_u64()).unwrap_or(0);
            let fid = src.get("file").and_then(|x| x.as_u64()).unwrap_or(0);
            if pid == 0 || fid == 0 {
                built.files.push((name.clone(), path.clone()));
                continue;
            }
            let (title, slug) =
                cf_title_slug(pid).unwrap_or_else(|| (name.trim_end_matches(".jar").to_string(), String::new()));
            inv.mods.push(InviteMod {
                source: "cf".into(),
                id: format!("{pid}/{fid}"),
                hash: short,
                optional: false,
                title,
                slug,
            });
        } else {
            built
                .files
                .push((name.trim_end_matches(".jar").to_string(), path.clone()));
        }
    }
    built.invite = inv;
    Ok(built)
}

/// Name und Slug einer CurseForge-Mod (nur mit eigenem Schlüssel des Admins).
fn cf_title_slug(pid: u64) -> Option<(String, String)> {
    crate::curseforge::key()?;
    let m = crate::curseforge::get_mod(pid).ok()?;
    Some((
        m.get("name")?.as_str()?.to_string(),
        m.get("slug").and_then(|s| s.as_str()).unwrap_or("").to_string(),
    ))
}

/// Einladungs-Datei: ZIP mit vapafane.json und den Mods ohne Quelle (die dürfen und
/// müssen eingepackt werden). CurseForge-Mods werden nie eingepackt.
pub fn write_file(built: &Built, dest: &Path) -> Result<Invite> {
    let mut inv = built.invite.clone();
    let tmp = dest.with_extension("part");
    {
        let mut z = zip::ZipWriter::new(fs::File::create(&tmp)?);
        let o = zip::write::SimpleFileOptions::default();
        for (title, path) in &built.files {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let sha1 = net::sha1_file(path)?;
            z.start_file(
                format!("mods/{name}"),
                o.compression_method(zip::CompressionMethod::Stored),
            )?;
            z.write_all(&fs::read(path)?)?;
            inv.mods.push(InviteMod {
                source: "file".into(),
                id: name,
                hash: sha1[..HASH_LEN].to_string(),
                optional: false,
                title: title.clone(),
                slug: String::new(),
            });
        }
        z.start_file(MANIFEST, o)?;
        z.write_all(serde_json::to_string_pretty(&inv)?.as_bytes())?;
        z.finish()?;
    }
    fs::rename(&tmp, dest)?;
    Ok(inv)
}

/// Einladung aus einer .vapafane-Datei lesen.
pub fn read_file(path: &Path) -> Result<Invite> {
    let mut z = zip::ZipArchive::new(fs::File::open(path)?).map_err(|_| Error(crate::sprache::t!("inv-err-datei")))?;
    let mut s = String::new();
    z.by_name(MANIFEST)
        .map_err(|_| Error(crate::sprache::t!("inv-err-datei-leer")))?
        .take(1024 * 1024)
        .read_to_string(&mut s)?;
    let inv: Invite = serde_json::from_str(&s).map_err(|_| Error(crate::sprache::t!("inv-err-datei-kaputt")))?;
    check(&inv)?;
    Ok(inv)
}

/// Heimnetz-Adresse des Servers (für die Vorbelegung).
pub fn lan_address(dir: &Path) -> String {
    let port = server::read_properties(dir)
        .get("server-port")
        .cloned()
        .unwrap_or_else(|| "25565".into());
    format!("{}:{port}", crate::util::local_ip())
}

/// Öffentliche Adresse (fragt einmal bei api.ipify.org nach).
pub fn public_address(dir: &Path) -> Result<String> {
    let ip = net::get_text("https://api.ipify.org")?.trim().to_string();
    if ip.parse::<std::net::IpAddr>().is_err() {
        return err(crate::sprache::t!("inv-err-oeffentlich"));
    }
    let port = server::read_properties(dir)
        .get("server-port")
        .cloned()
        .unwrap_or_else(|| "25565".into());
    Ok(if port == "25565" { ip } else { format!("{ip}:{port}") })
}

#[allow(dead_code)]
pub fn flavor_of(inv: &Invite) -> Kind {
    Kind::parse(&inv.loader).unwrap_or(Kind::Vanilla)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(n: usize) -> Invite {
        Invite {
            format: 1,
            name: "Debongs Welt".into(),
            address: "beispiel.dyndns.org".into(),
            mc: "26.3".into(),
            loader: "fabric".into(),
            loader_version: "0.19.5".into(),
            mods: (0..n)
                .map(|i| InviteMod {
                    source: "mr".into(),
                    // Modrinth-Versions-IDs: 8 zufällige Zeichen aus A–Z, a–z, 0–9
                    id: random_text(
                        i as u64 * 7 + 1,
                        8,
                        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                    ),
                    hash: random_text(i as u64 * 7 + 3, 12, b"0123456789abcdef"),
                    optional: i % 5 == 0,
                    title: String::new(),
                    slug: String::new(),
                })
                .collect(),
            address6: String::new(),
            only_v6: false,
        }
    }

    /// Einfache Pseudo-Zufallszeichen (wie echte IDs/Hashes kaum komprimierbar).
    fn random_text(seed: u64, len: usize, abc: &[u8]) -> String {
        let mut x = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (0..len)
            .map(|_| {
                x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                abc[((x >> 33) as usize) % abc.len()] as char
            })
            .collect()
    }

    #[test]
    fn roundtrip_and_messenger_safe() {
        let inv = sample(20);
        let code = encode(&inv).unwrap();
        assert!(code.starts_with(PREFIX));
        assert!(
            code[PREFIX.len()..].chars().all(|c| c.is_ascii_alphanumeric()),
            "nur Buchstaben/Ziffern"
        );
        // Zeilenumbrüche, wie sie Messenger einfügen, schaden nicht
        let broken = format!("{}\n{} ", &code[..40], &code[40..]);
        assert_eq!(decode(&broken).unwrap(), inv);
        println!("Code-Länge bei 20 Mods: {} Zeichen", code.len());
        assert!(code.len() < 1200);
    }

    #[test]
    fn ipv6_zusatz() {
        let mut inv = sample(0);
        inv.address6 = "[2a02:8071::1]:25565".into();
        inv.only_v6 = true;
        let back = decode(&encode(&inv).unwrap()).unwrap();
        assert_eq!(back, inv);
        // Ohne Zusatz bleibt der Code wie bisher (ältere Versionen lesen ihn weiter)
        let plain = serde_json::to_string(&sample(0)).unwrap();
        assert!(!plain.contains("a6") && !plain.contains("o6"));
        inv.address6 = "evil.example:1".into();
        assert!(check(&inv).is_err());
    }

    #[test]
    fn rejects_garbage() {
        assert!(decode("hallo").is_err());
        assert!(decode("VPF1-0OIl").is_err()); // keine Base58-Zeichen
        assert_eq!(decode("VPF2-abc").unwrap_err().0, crate::sprache::t!("inv-err-neuer"));
        let mut bad = sample(1);
        bad.mods[0].id = "../../evil".into();
        assert!(check(&bad).is_err());
    }
}
