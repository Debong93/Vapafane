//! Client-Teil: eine Einladung einlösen und Minecraft für den Server einrichten.
//!
//! Vapafane richtet ein eigenes Spielverzeichnis ein (Mods, servers.dat), installiert
//! den Loader in den offiziellen Launcher und legt dort ein Profil „Vapafane – <Server>“
//! an. Gespielt wird im offiziellen Launcher – Vapafane startet Minecraft nicht selbst
//! und fasst kein Microsoft-Konto an.

use base64::Engine;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::archive::Progress;
use crate::config;
use crate::flavor::{self, Kind};
use crate::invite::{Invite, InviteMod};
use crate::nbt;
use crate::net;
use crate::transfer::{self, Take};
use crate::updater;
use crate::util::{self, Error, Result, err};

const META: &str = "vapafane-client.json";
const MR: &str = "https://api.modrinth.com/v2";
pub const LAUNCHER_DOWNLOAD: &str = "https://www.minecraft.net/download";

// ---------------------------------------------------------------- Status-Ping

/// host:port zerlegen (ohne Port: 25565).
pub fn split_address(addr: &str) -> Result<(String, u16)> {
    let a = addr.trim();
    if a.is_empty() {
        return err("Keine Serveradresse angegeben.");
    }
    if let Some(rest) = a.strip_prefix('[') {
        // [IPv6]:Port
        let (host, port) = rest.split_once(']').ok_or_else(|| Error("Ungültige Adresse".into()))?;
        let port = port
            .strip_prefix(':')
            .map(|p| p.parse())
            .transpose()
            .map_err(|_| Error("Ungültiger Port".into()))?;
        return Ok((host.to_string(), port.unwrap_or(25565)));
    }
    match a.rsplit_once(':') {
        Some((h, p)) if !h.contains(':') => Ok((
            h.to_string(),
            p.parse().map_err(|_| Error("Ungültiger Port in der Adresse".into()))?,
        )),
        _ => Ok((a.to_string(), 25565)),
    }
}

fn write_varint(out: &mut Vec<u8>, v: i32) {
    let mut v = v as u32;
    loop {
        if v & !0x7f == 0 {
            out.push(v as u8);
            return;
        }
        out.push((v as u8 & 0x7f) | 0x80);
        v >>= 7;
    }
}

fn read_varint(r: &mut impl Read) -> Result<i32> {
    let mut out: u32 = 0;
    for i in 0..5 {
        let mut b = [0u8];
        r.read_exact(&mut b)?;
        out |= ((b[0] & 0x7f) as u32) << (7 * i);
        if b[0] & 0x80 == 0 {
            return Ok(out as i32);
        }
    }
    err("Ungültige Antwort vom Server")
}

/// „Server List Ping“ über den normalen Spiel-Port – wie die Serverliste im Spiel.
pub fn ping(address: &str) -> Result<Value> {
    let (host, port) = split_address(address)?;
    let sock_addr = (host.as_str(), port)
        .to_socket_addrs()
        .map_err(|_| Error(format!("Adresse „{host}“ nicht gefunden.")))?
        .next()
        .ok_or_else(|| Error(format!("Adresse „{host}“ nicht gefunden.")))?;
    let mut s = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(5))
        .map_err(|_| Error("Server nicht erreichbar (aus, falsche Adresse oder Port nicht freigegeben).".into()))?;
    s.set_read_timeout(Some(Duration::from_secs(5)))?;
    s.set_write_timeout(Some(Duration::from_secs(5)))?;
    let mut hs = vec![];
    write_varint(&mut hs, 0x00); // Handshake
    write_varint(&mut hs, -1); // Protokoll unbekannt – der Server nennt seins
    write_varint(&mut hs, host.len() as i32);
    hs.extend_from_slice(host.as_bytes());
    hs.extend_from_slice(&port.to_be_bytes());
    write_varint(&mut hs, 1); // nächster Schritt: Status
    let mut pkt = vec![];
    write_varint(&mut pkt, hs.len() as i32);
    pkt.extend_from_slice(&hs);
    pkt.extend_from_slice(&[1, 0]); // Status-Anfrage
    s.write_all(&pkt)?;
    let len = read_varint(&mut s)?;
    if !(1..=1_000_000).contains(&len) {
        return err("Ungültige Antwort vom Server");
    }
    let mut body = vec![0u8; len as usize];
    s.read_exact(&mut body)?;
    let mut r = &body[..];
    let _id = read_varint(&mut r)?;
    let slen = read_varint(&mut r)? as usize;
    let text = String::from_utf8_lossy(&r[..slen.min(r.len())]).into_owned();
    let v: Value = serde_json::from_str(&text).map_err(|_| Error("Ungültige Antwort vom Server".into()))?;
    let motd = match v.get("description") {
        Some(Value::String(s)) => s.clone(),
        Some(d) => d.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string(),
        None => String::new(),
    };
    Ok(json!({
        "online": true,
        "version": v.pointer("/version/name"),
        "protocol": v.pointer("/version/protocol"),
        "players": v.pointer("/players/online"),
        "max": v.pointer("/players/max"),
        "motd": crate::rcon::strip_formatting(&motd),
    }))
}

/// Passt die Version des Servers zur Einladung? (Paper meldet z. B. „Paper 1.21.4“)
pub fn version_matches(ping: &Value, mc: &str) -> bool {
    ping.get("version")
        .and_then(|v| v.as_str())
        .map(|v| v == mc || v.split([' ', '-']).any(|p| p == mc))
        .unwrap_or(false)
}

// ---------------------------------------------------------------- Launcher finden

/// Ordner des offiziellen Launchers (Linux auch Flatpak/Snap; Windows %APPDATA%\.minecraft).
pub fn launcher_dirs() -> Vec<PathBuf> {
    let h = config::home();
    let mut c = vec![];
    if config::test_home().is_none()
        && cfg!(windows)
        && let Some(a) = std::env::var_os("APPDATA")
    {
        c.push(PathBuf::from(a).join(".minecraft"));
    }
    c.push(h.join(".minecraft"));
    c.push(h.join(".var/app/com.mojang.Minecraft/.minecraft"));
    c.push(h.join("snap/mc-installer/current/.minecraft"));
    let mut seen = HashSet::new();
    c.into_iter()
        .filter(|d| d.is_dir())
        .filter(|d| seen.insert(util::canonical(d)))
        .collect()
}

/// Profil-Dateien des Launchers (normal und Microsoft-Store-Variante).
pub fn profile_files() -> Vec<PathBuf> {
    launcher_dirs()
        .into_iter()
        .flat_map(|d| {
            ["launcher_profiles.json", "launcher_profiles_microsoft_store.json"]
                .into_iter()
                .map(move |f| d.join(f))
        })
        .filter(|f| f.is_file())
        .collect()
}

fn profile_key(name: &str) -> String {
    format!("vapafane-{}", util::safe_name(name).to_lowercase())
}

fn icon_data_uri() -> String {
    let png = include_bytes!("../icons/64x64.png");
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(png)
    )
}

/// Sicherheitskopie einer Launcher-Datei (vor jeder Änderung).
fn backup(file: &Path) -> Result<()> {
    let b = PathBuf::from(format!("{}.vapafane-sicherung", file.display()));
    fs::copy(file, b)?;
    Ok(())
}

/// Nur den eigenen Eintrag in launcher_profiles.json anlegen/ändern.
fn upsert_profile(file: &Path, key: &str, name: &str, version_id: &str, game_dir: &Path) -> Result<()> {
    backup(file)?;
    let mut v: Value = serde_json::from_str(&fs::read_to_string(file)?).map_err(|_| {
        Error(format!(
            "{} ist beschädigt – bitte den Launcher einmal starten.",
            file.display()
        ))
    })?;
    if !v.get("profiles").map(|p| p.is_object()).unwrap_or(false) {
        v["profiles"] = json!({});
    }
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let created = v["profiles"][key].get("created").cloned().unwrap_or(json!(now));
    v["profiles"][key] = json!({
        "name": format!("Vapafane – {name}"),
        "type": "custom",
        "created": created,
        "lastUsed": now,
        "lastVersionId": version_id,
        "gameDir": game_dir,
        "icon": icon_data_uri(),
    });
    util::write_atomic(file, serde_json::to_string_pretty(&v)?.as_bytes())
}

fn remove_profile(file: &Path, key: &str) -> Result<()> {
    let Ok(text) = fs::read_to_string(file) else {
        return Ok(());
    };
    let mut v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    if let Some(p) = v.get_mut("profiles").and_then(|p| p.as_object_mut())
        && p.remove(key).is_some()
    {
        backup(file)?;
        util::write_atomic(file, serde_json::to_string_pretty(&v)?.as_bytes())?;
    }
    Ok(())
}

/// Java für den NeoForge-Installer: installiertes Java oder das vom Launcher mitgebrachte.
fn java_for(need: u32, launcher: &Path) -> Option<String> {
    if let Some(j) = crate::java::java_at_least(need) {
        return Some(j);
    }
    let exe = if cfg!(windows) { "java.exe" } else { "java" };
    walkdir::WalkDir::new(launcher.join("runtime"))
        .max_depth(6)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == exe && e.path().parent().map(|p| p.ends_with("bin")).unwrap_or(false))
        .map(|e| e.path().to_string_lossy().into_owned())
        .find(|j| crate::java::java_major(j) >= need)
}

// ---------------------------------------------------------------- servers.dat

/// Server-Liste für einen Spielordner: vorhandene Einträge bleiben (auch die der übernommenen
/// Quelle), der Server dieser Einladung steht sicher drin.
fn servers_dat(lists: &[Vec<u8>], name: &str, address: &str) -> Vec<u8> {
    let refs: Vec<&[u8]> = lists.iter().map(Vec::as_slice).collect();
    nbt::servers_with(&refs, name, address)
}

// ---------------------------------------------------------------- Vorschau

pub struct Resolved {
    pub title: String,
    pub version: String,
    pub file: String,
    pub size: u64,
    pub url: Option<String>,
    pub sha512: Option<String>,
    pub sha1: Option<String>,
    pub optional: bool,
    pub source: String,
    /// Für den Download von Hand (CurseForge ohne Erlaubnis/Schlüssel)
    pub page: Option<String>,
    pub key: String,
    pub deps: Vec<String>,
    pub project: Option<String>,
}

fn kind_of(inv: &Invite) -> Kind {
    Kind::parse(&inv.loader).unwrap_or(Kind::Vanilla)
}

fn cf_page(m: &InviteMod, fid: &str) -> String {
    if m.slug.is_empty() {
        format!(
            "https://www.curseforge.com/minecraft/search?search={}",
            m.title.replace(' ', "+")
        )
    } else {
        format!("https://www.curseforge.com/minecraft/mc-mods/{}/files/{fid}", m.slug)
    }
}

/// Mods der Einladung nachschlagen (Namen, Größen, Download-Adressen).
pub fn resolve(inv: &Invite, zip: Option<&Path>) -> Result<Vec<Resolved>> {
    let mut out = vec![];
    let mr: Vec<&InviteMod> = inv.mods.iter().filter(|m| m.source == "mr").collect();
    if !mr.is_empty() {
        let ids: Vec<&str> = mr.iter().map(|m| m.id.as_str()).collect();
        let versions = net::get_json_query(&format!("{MR}/versions"), &[("ids", serde_json::to_string(&ids)?)])?;
        let by_id: HashMap<String, Value> = versions
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| Some((v.get("id")?.as_str()?.to_string(), v.clone())))
            .collect();
        let pids: HashSet<String> = by_id
            .values()
            .filter_map(|v| v.get("project_id")?.as_str().map(str::to_string))
            .collect();
        let infos = updater::project_infos(&pids)?;
        for m in mr {
            let Some(v) = by_id.get(&m.id) else {
                return err(format!(
                    "Eine Mod aus der Einladung gibt es auf Modrinth nicht mehr ({}).",
                    m.id
                ));
            };
            let files = v.get("files").and_then(|f| f.as_array()).cloned().unwrap_or_default();
            let f = files
                .iter()
                .find(|f| {
                    f.pointer("/hashes/sha1")
                        .and_then(|h| h.as_str())
                        .map(|h| h.starts_with(&m.hash))
                        .unwrap_or(false)
                })
                .ok_or_else(|| Error("Eine Mod-Datei passt nicht zur Einladung (Hash stimmt nicht).".into()))?;
            let pid = v.get("project_id").and_then(|p| p.as_str()).unwrap_or("").to_string();
            let deps = v
                .get("dependencies")
                .and_then(|d| d.as_array())
                .into_iter()
                .flatten()
                .filter(|d| d.get("dependency_type").and_then(|t| t.as_str()) == Some("required"))
                .filter_map(|d| d.get("project_id").and_then(|p| p.as_str()).map(str::to_string))
                .collect();
            out.push(Resolved {
                title: infos
                    .get(&pid)
                    .and_then(|p| p.get("title"))
                    .and_then(|t| t.as_str())
                    .unwrap_or(&m.id)
                    .to_string(),
                version: v
                    .get("version_number")
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string(),
                file: f.get("filename").and_then(|n| n.as_str()).unwrap_or("").to_string(),
                size: f.get("size").and_then(|s| s.as_u64()).unwrap_or(0),
                url: f.get("url").and_then(|u| u.as_str()).map(str::to_string),
                sha512: f.pointer("/hashes/sha512").and_then(|h| h.as_str()).map(str::to_string),
                sha1: f.pointer("/hashes/sha1").and_then(|h| h.as_str()).map(str::to_string),
                optional: m.optional,
                source: "modrinth".into(),
                page: None,
                key: m.id.clone(),
                deps,
                project: Some(pid),
            });
        }
    }
    for m in inv.mods.iter().filter(|m| m.source == "cf") {
        let (pid, fid) = m.id.split_once('/').unwrap_or((&m.id, "0"));
        let mut r = Resolved {
            title: if m.title.is_empty() {
                format!("CurseForge-Mod {pid}")
            } else {
                m.title.clone()
            },
            version: String::new(),
            file: String::new(),
            size: 0,
            url: None,
            sha512: None,
            sha1: None,
            optional: m.optional,
            source: "curseforge".into(),
            page: Some(cf_page(m, fid)),
            key: m.id.clone(),
            deps: vec![],
            project: None,
        };
        // Mit eigenem Schlüssel des Spielers: direkt laden, wenn der Autor es erlaubt
        if crate::curseforge::key().is_some()
            && let (Ok(p), Ok(f)) = (pid.parse::<u64>(), fid.parse::<u64>())
            && let Ok(file) = crate::curseforge::get_file(p, f)
        {
            r.file = file.get("fileName").and_then(|n| n.as_str()).unwrap_or("").to_string();
            r.version = file
                .get("displayName")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string();
            r.size = file.get("fileLength").and_then(|n| n.as_u64()).unwrap_or(0);
            r.sha1 = crate::curseforge::file_sha1(&file);
            r.url = crate::curseforge::download_url(&file);
        }
        out.push(r);
    }
    for m in inv.mods.iter().filter(|m| m.source == "file") {
        let size = zip
            .and_then(|z| zip::ZipArchive::new(fs::File::open(z).ok()?).ok())
            .and_then(|mut a| a.by_name(&format!("mods/{}", m.id)).ok().map(|f| f.size()))
            .unwrap_or(0);
        out.push(Resolved {
            title: if m.title.is_empty() {
                m.id.clone()
            } else {
                m.title.clone()
            },
            version: String::new(),
            file: m.id.clone(),
            size,
            url: None,
            sha512: None,
            sha1: None,
            optional: m.optional,
            source: "file".into(),
            page: None,
            key: m.id.clone(),
            deps: vec![],
            project: None,
        });
    }
    Ok(out)
}

/// Adresse, unter der der Server gerade antwortet: erst die normale, dann die IPv6-Adresse
/// aus der Einladung. Liefert (Adresse, Ping-Ergebnis, Hinweis für den Spieler).
pub fn best_address(inv: &Invite) -> (String, Value, Option<String>) {
    match ping(&inv.address) {
        Ok(p) => (inv.address.clone(), p, None),
        Err(e) => {
            if !inv.address6.is_empty()
                && let Ok(p) = ping(&inv.address6)
            {
                return (inv.address6.clone(), p, None);
            }
            let hint = crate::reach::client_hint(inv.only_v6, crate::reach::has_ipv6());
            (inv.address.clone(), json!({"online": false, "error": e.0}), hint)
        }
    }
}

/// Adresse für die Serverliste im Spiel (mit IPv6 als Ausweich-Adresse).
fn game_address(inv: &Invite) -> String {
    if inv.address6.is_empty() {
        inv.address.clone()
    } else {
        best_address(inv).0
    }
}

/// Vorschau für die Anzeige.
pub fn preview(inv: &Invite, zip: Option<&Path>) -> Result<Value> {
    let mods = resolve(inv, zip)?;
    let (address, ping, hint) = best_address(inv);
    let ok = version_matches(&ping, &inv.mc);
    Ok(json!({
        "invite": inv,
        "mods": mods.iter().map(|m| json!({
            "key": m.key, "title": m.title, "version": m.version, "size": m.size, "optional": m.optional,
            "source": m.source, "manual": m.source == "curseforge" && m.url.is_none(), "page": m.page,
            "missing_in_file": m.source == "file" && zip.is_none(),
        })).collect::<Vec<_>>(),
        "total": mods.iter().map(|m| m.size).sum::<u64>(),
        "ping": ping,
        "address": address,
        "hint": hint,
        "version_ok": ok,
        "launcher": !launcher_dirs().is_empty(),
        "profiles": !profile_files().is_empty(),
        "default_dir": default_dir(&inv.name),
        "modrinth_existing": modrinth_existing(inv)
            .iter()
            .filter_map(|d| d.file_name().map(|n| n.to_string_lossy().into_owned()))
            .collect::<Vec<_>>(),
    }))
}

/// Instanzen in der Modrinth-App, die aus einem früheren Paket dieses Servers stammen.
fn modrinth_existing(inv: &Invite) -> Vec<PathBuf> {
    transfer::modrinth_instances_for(&util::safe_name(&inv.name), &transfer::modrinth_profile_dirs())
}

// ---------------------------------------------------------------- Einrichten

pub fn clients_base() -> PathBuf {
    config::data_dir().join("clients")
}

pub fn default_dir(name: &str) -> PathBuf {
    clients_base().join(util::safe_name(name))
}

fn index_file() -> PathBuf {
    config::data_dir().join("clients.json")
}

fn read_index() -> Vec<PathBuf> {
    fs::read_to_string(index_file())
        .ok()
        .and_then(|t| serde_json::from_str::<Vec<PathBuf>>(&t).ok())
        .unwrap_or_default()
}

fn write_index(list: &[PathBuf]) -> Result<()> {
    util::write_atomic(&index_file(), serde_json::to_string_pretty(list)?.as_bytes())
}

/// Neuen Ordner gleich als Vapafane-Client kennzeichnen. Bricht das Einrichten ab (z. B. ohne
/// Internet), blockiert der halb gefüllte Ordner so keinen zweiten Versuch und lässt sich entfernen.
fn mark_incomplete(inv: &Invite, dir: &Path, target: &str) -> Result<()> {
    let meta = json!({ "invite": inv, "target": target, "mods": [], "incomplete": true });
    util::write_atomic(&dir.join(META), serde_json::to_string_pretty(&meta)?.as_bytes())?;
    let mut idx = read_index();
    if !idx.iter().any(|p| util::canonical(p) == util::canonical(dir)) {
        idx.push(dir.to_path_buf());
        write_index(&idx)?;
    }
    Ok(())
}

/// Client einrichten bzw. aktualisieren. `selected` = gewählte optionale Mods (Schlüssel).
/// Mods, die geladen werden: Einladung (ohne abgewählte optionale) plus fehlende Pflicht-Abhängigkeiten.
/// Was beim Einrichten gewählt wurde.
#[derive(Default)]
pub struct Choices<'a> {
    /// gewählte optionale Mods (Schlüssel)
    pub selected: HashSet<String>,
    /// Einstellungen aus dieser Quelle übernehmen
    pub take: Option<&'a Take>,
}

/// Neueste passende Version eines Modrinth-Projekts als zusätzliche Mod.
fn extra_mod(project: &str, title: &str, kind: Kind, mc: &str) -> Result<Option<Resolved>> {
    let Some(v) = updater::newest_for_project(project, kind.loaders(), mc)? else {
        return Ok(None);
    };
    let Some(f) = updater::primary_file(&v) else {
        return Ok(None);
    };
    let r = Resolved {
        title: title.to_string(),
        version: v.get("version_number").and_then(|n| n.as_str()).unwrap_or("").to_string(),
        file: f.filename,
        size: f.size,
        url: Some(f.url),
        sha512: f.sha512,
        sha1: f.sha1,
        optional: false,
        source: "modrinth".into(),
        page: None,
        key: project.to_string(),
        deps: vec![],
        project: Some(project.to_string()),
    };
    Ok(Some(r))
}

/// Client einrichten bzw. aktualisieren. `selected` = gewählte optionale Mods (Schlüssel).
/// Mods, die geladen werden: Einladung (ohne abgewählte optionale) plus fehlende Pflicht-Abhängigkeiten.
fn plan_mods(inv: &Invite, zip: Option<&Path>, selected: &HashSet<String>) -> Result<Vec<Resolved>> {
    let kind = kind_of(inv);
    let mods: Vec<Resolved> = resolve(inv, zip)?
        .into_iter()
        .filter(|m| !m.optional || selected.contains(&m.key))
        .collect();

    let mut extra: Vec<Resolved> = vec![];
    if matches!(kind, Kind::Fabric | Kind::NeoForge) {
        let mut seen: HashSet<String> = mods.iter().filter_map(|m| m.project.clone()).collect();
        // Pflicht-Abhängigkeiten, die nicht in der Einladung stehen
        for dep in mods.iter().flat_map(|m| m.deps.clone()) {
            if !seen.insert(dep.clone()) {
                continue;
            }
            if let Some(r) = extra_mod(&dep, &dep, kind, &inv.mc)? {
                extra.push(r);
            }
        }
    }

    let mut all = mods;
    all.extend(extra);
    Ok(all)
}

pub fn install(
    inv: &Invite,
    zip: Option<&Path>,
    target: Option<&Path>,
    choices: &Choices,
    progress: Progress,
) -> Result<Value> {
    let Choices { selected, take } = choices;
    let take = *take;
    let kind = kind_of(inv);
    let game_dir = target.map(Path::to_path_buf).unwrap_or_else(|| default_dir(&inv.name));
    let meta_path = game_dir.join(META);
    let old_meta: Option<Value> = fs::read_to_string(&meta_path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok());
    if old_meta.is_none() && game_dir.exists() && fs::read_dir(&game_dir)?.next().is_some() {
        return err(format!(
            "Der Ordner {} ist nicht leer und gehört nicht zu Vapafane.",
            game_dir.display()
        ));
    }
    fs::create_dir_all(game_dir.join("mods"))?;
    if old_meta.is_none() {
        mark_incomplete(inv, &game_dir, "launcher")?;
    }
    // Ordner für eigene Shader und Ressourcenpakete gleich anlegen (Anleitung „Shader & Extras“)
    fs::create_dir_all(game_dir.join("shaderpacks"))?;
    fs::create_dir_all(game_dir.join("resourcepacks"))?;
    // Vorher ein Modrinth-Paket? Das wird nicht mehr gebraucht.
    if let Some(p) = old_meta.as_ref().and_then(|m| m.get("mrpack")).and_then(|p| p.as_str()) {
        let _ = fs::remove_file(p);
    }

    progress(0.05, "Prüfe die Mods …");
    let all = plan_mods(inv, zip, selected)?;

    // Alte Mods aus einer früheren Einladung entfernen
    if let Some(m) = &old_meta {
        for f in m
            .get("mods")
            .and_then(|m| m.as_array())
            .into_iter()
            .flatten()
            .filter_map(|f| f.as_str())
        {
            let name = Path::new(f).file_name().unwrap_or_default();
            let _ = fs::remove_file(game_dir.join("mods").join(name));
        }
    }

    let total = all.len().max(1) as f64;
    let mut installed = vec![];
    let mut manual = vec![];
    for (i, m) in all.iter().enumerate() {
        progress(0.1 + 0.6 * i as f64 / total, &format!("Lade {} …", m.title));
        let name = updater::safe_filename(&m.file).unwrap_or_default();
        if m.source == "file" {
            let Some(zp) = zip else {
                manual.push(json!({"title": m.title, "page": Value::Null, "file": m.file, "reason": "nur in der Einladungs-Datei enthalten"}));
                continue;
            };
            let mut a = zip::ZipArchive::new(fs::File::open(zp)?)?;
            let mut f = a.by_name(&format!("mods/{}", m.file))?;
            let mut data = vec![];
            f.read_to_end(&mut data)?;
            let want = inv
                .mods
                .iter()
                .find(|x| x.source == "file" && x.id == m.file)
                .map(|x| x.hash.clone())
                .unwrap_or_default();
            let got = {
                use sha1::Digest;
                hex::encode(sha1::Sha1::digest(&data))
            };
            if !got.starts_with(&want) {
                return err(format!("Datei {} aus der Einladung ist beschädigt.", m.file));
            }
            util::write_atomic(&game_dir.join("mods").join(&name), &data)?;
            installed.push(name);
            continue;
        }
        let Some(url) = &m.url else {
            manual.push(json!({"title": m.title, "page": m.page, "file": m.file, "reason": "Download nur im Browser (CurseForge)"}));
            continue;
        };
        net::download(
            url,
            &game_dir.join("mods").join(&name),
            net::Expect {
                sha1: m.sha1.as_deref(),
                sha512: m.sha512.as_deref(),
                ..Default::default()
            },
            &|_| {},
        )?;
        installed.push(name);
    }

    // Loader in den offiziellen Launcher
    progress(0.75, "Richte den Launcher ein …");
    let launchers = launcher_dirs();
    let mut notes: Vec<String> = vec![];
    let version_id = match kind {
        Kind::Vanilla | Kind::Paper => inv.mc.clone(),
        Kind::Fabric => {
            let loader = if inv.loader_version.is_empty() {
                net::fabric_server_jar_url(&inv.mc)?.1
            } else {
                inv.loader_version.clone()
            };
            let prof = net::get_json(&format!(
                "https://meta.fabricmc.net/v2/versions/loader/{}/{loader}/profile/json",
                inv.mc
            ))?;
            let id = prof.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string();
            if id.is_empty() || id.contains(['/', '\\']) || id.contains("..") {
                return err("Fabric-Profil ungültig.");
            }
            for l in &launchers {
                let d = l.join("versions").join(&id);
                fs::create_dir_all(&d)?;
                util::write_atomic(
                    &d.join(format!("{id}.json")),
                    serde_json::to_string_pretty(&prof)?.as_bytes(),
                )?;
            }
            id
        }
        Kind::NeoForge => {
            let v = inv.loader_version.clone();
            let id = format!("neoforge-{v}");
            for l in &launchers {
                if l.join("versions").join(&id).join(format!("{id}.json")).is_file() {
                    continue; // schon installiert
                }
                let profiles = l.join("launcher_profiles.json");
                if !profiles.is_file() {
                    notes.push(
                        "Bitte den Minecraft-Launcher einmal starten und wieder schließen, dann nochmal einrichten."
                            .into(),
                    );
                    continue;
                }
                progress(
                    0.8,
                    &format!("Installiere NeoForge {v} (kann ein paar Minuten dauern) …"),
                );
                let art = flavor::server_artifact(Kind::NeoForge, &inv.mc)?;
                let base = format!(
                    "https://maven.neoforged.net/releases/net/neoforged/neoforge/{v}/neoforge-{v}-installer.jar"
                );
                let sha1 = if art.build == v {
                    art.sha1.clone()
                } else {
                    net::get_text(&format!("{base}.sha1"))
                        .ok()
                        .map(|s| s.trim().to_string())
                };
                let tmp = config::data_dir().join("tmp");
                fs::create_dir_all(&tmp)?;
                let inst = tmp.join(format!("neoforge-{v}-installer.jar"));
                net::download(
                    &base,
                    &inst,
                    net::Expect {
                        sha1: sha1.as_deref(),
                        ..Default::default()
                    },
                    &|_| {},
                )?;
                let need = net::required_java(&inv.mc).ok().flatten().unwrap_or(21);
                let java = java_for(need, l).ok_or_else(|| {
                    Error(format!("Für NeoForge wird Java {need} gebraucht. Bitte den Minecraft-Launcher einmal starten (er bringt Java mit) oder Java installieren."))
                })?;
                // Der Installer legt selbst ein Profil an – das machen wir rückgängig, nur unseres bleibt
                let before: Vec<(PathBuf, Option<String>)> =
                    ["launcher_profiles.json", "launcher_profiles_microsoft_store.json"]
                        .iter()
                        .map(|f| (l.join(f), fs::read_to_string(l.join(f)).ok()))
                        .collect();
                let out = util::command(&java)
                    .arg("-jar")
                    .arg(&inst)
                    .arg("--installClient")
                    .arg(l)
                    .current_dir(&tmp)
                    .output()?;
                for (f, content) in before {
                    if let Some(c) = content {
                        fs::write(&f, c)?;
                    }
                }
                let _ = fs::remove_dir_all(&tmp);
                if !out.status.success() {
                    let text = String::from_utf8_lossy(&out.stdout).into_owned();
                    let tail: Vec<&str> = text.lines().rev().take(6).collect();
                    return err(format!(
                        "NeoForge-Installer ist fehlgeschlagen:\n{}",
                        tail.into_iter().rev().collect::<Vec<_>>().join("\n")
                    ));
                }
            }
            id
        }
    };

    let mut taken = Value::Null;
    if let Some(t) = take {
        progress(0.85, "Übernehme Einstellungen …");
        let files = transfer::gather(t)?;
        taken = transfer::copy_missing(&files, &game_dir)?.json(&t.dir, t.has("servers"));
    }

    progress(0.9, "Trage den Server ein …");
    let lists: Vec<Vec<u8>> = [fs::read(game_dir.join("servers.dat")).ok(), take.and_then(Take::servers)]
        .into_iter()
        .flatten()
        .collect();
    util::write_atomic(
        &game_dir.join("servers.dat"),
        &servers_dat(&lists, &inv.name, &game_address(inv)),
    )?;
    let key = profile_key(&inv.name);
    let files = profile_files();
    for f in &files {
        upsert_profile(f, &key, &inv.name, &version_id, &game_dir)?;
    }
    if launchers.is_empty() {
        notes.push("Der offizielle Minecraft-Launcher wurde nicht gefunden. Bitte installieren, einmal starten und dann hier „Aktualisieren“ drücken.".into());
    } else if files.is_empty() {
        notes.push(
            "Der Launcher wurde noch nie gestartet. Bitte einmal starten und wieder schließen, dann „Aktualisieren“."
                .into(),
        );
    }
    let meta = json!({
        "invite": inv,
        "version_id": version_id,
        "profile_key": key,
        "profile_files": files,
        "mods": installed,
        "manual": manual,
        "updated": chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
    });
    util::write_atomic(&meta_path, serde_json::to_string_pretty(&meta)?.as_bytes())?;
    let mut idx = read_index();
    if !idx.iter().any(|p| util::canonical(p) == util::canonical(&game_dir)) {
        idx.push(game_dir.clone());
        write_index(&idx)?;
    }
    progress(1.0, "Fertig.");
    Ok(json!({
        "dir": game_dir,
        "version_id": version_id,
        "installed": meta["mods"],
        "manual": manual,
        "notes": notes,
        "profile": !files.is_empty(),
        "taken": taken,
    }))
}

// ---------------------------------------------------------------- Liste, Entfernen, Spielen

pub fn list() -> Vec<Value> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(clients_base())
        .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
        .unwrap_or_default();
    dirs.extend(read_index());
    let mut seen = HashSet::new();
    dirs.into_iter()
        .filter(|d| seen.insert(util::canonical(d)))
        .filter_map(|d| {
            let m: Value = serde_json::from_str(&fs::read_to_string(d.join(META)).ok()?).ok()?;
            Some(json!({
                "dir": d,
                "name": m.pointer("/invite/n"),
                "address": m.pointer("/invite/a"),
                "mc": m.pointer("/invite/m"),
                "loader": m.pointer("/invite/l"),
                "mods": m.get("mods").and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0),
                "manual": m.get("manual"),
                "updated": m.get("updated"),
                "target": m.get("target").and_then(|t| t.as_str()).unwrap_or("launcher"),
                "incomplete": m.get("incomplete").and_then(|x| x.as_bool()).unwrap_or(false),
            }))
        })
        .collect()
}

pub fn remove(dir: &Path) -> Result<()> {
    let meta: Value = serde_json::from_str(
        &fs::read_to_string(dir.join(META)).map_err(|_| Error("Das ist kein Vapafane-Client.".into()))?,
    )?;
    // Bei Modrinth-Clients gibt es kein Launcher-Profil – die Instanz löscht man in der App selbst
    if let Some(key) = meta.get("profile_key").and_then(|k| k.as_str()) {
        for f in profile_files() {
            remove_profile(&f, key)?;
        }
    }
    fs::remove_dir_all(dir)?;
    let idx: Vec<PathBuf> = read_index()
        .into_iter()
        .filter(|p| util::canonical(p) != util::canonical(dir))
        .collect();
    write_index(&idx)
}

/// Gespeicherte Einladung eines Clients (für den Versions-Check beim Öffnen).
pub fn invite_of(dir: &Path) -> Result<Invite> {
    let meta: Value = serde_json::from_str(&fs::read_to_string(dir.join(META))?)?;
    Ok(serde_json::from_value(
        meta.get("invite").cloned().unwrap_or(Value::Null),
    )?)
}

/// Offiziellen Launcher öffnen (Vapafane startet Minecraft nicht selbst).
pub fn open_launcher() -> Result<()> {
    #[cfg(windows)]
    {
        let pf = std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".into());
        let classic = PathBuf::from(pf).join(r"Minecraft Launcher\MinecraftLauncher.exe");
        if classic.is_file() {
            std::process::Command::new(classic).spawn()?;
            return Ok(());
        }
        // Launcher aus dem Microsoft Store
        if std::process::Command::new("explorer.exe")
            .arg(r"shell:AppsFolder\Microsoft.4297127D64EC6_8wekyb3d8bbwe!Minecraft")
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
    }
    #[cfg(not(windows))]
    {
        let try_run = |cmd: &str, args: &[&str]| {
            std::process::Command::new(cmd)
                .args(args)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .is_ok()
        };
        if try_run("minecraft-launcher", &[]) {
            return Ok(());
        }
        let flatpak = std::process::Command::new("flatpak")
            .args(["info", "com.mojang.Minecraft"])
            .output();
        if flatpak.map(|o| o.status.success()).unwrap_or(false) && try_run("flatpak", &["run", "com.mojang.Minecraft"])
        {
            return Ok(());
        }
        if try_run("mc-installer", &[]) {
            return Ok(());
        }
    }
    err(format!(
        "Der offizielle Minecraft-Launcher wurde nicht gefunden. Download: {LAUNCHER_DOWNLOAD}"
    ))
}

// ---------------------------------------------------------------- Ziel: Modrinth-App

/// Ist die Modrinth-App installiert? Gibt den Startbefehl zurück (Programm + Argumente vor der Datei).
pub fn modrinth_app() -> Option<(String, Vec<String>)> {
    #[cfg(windows)]
    {
        // Windows: über die Dateiverknüpfung öffnen (siehe open_mrpack)
        let local = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
        let pf = std::env::var_os("ProgramFiles").map(PathBuf::from);
        for base in [local.clone(), local.map(|l| l.join("Programs")), pf]
            .into_iter()
            .flatten()
        {
            let exe = base.join("Modrinth App").join("Modrinth App.exe");
            if exe.is_file() {
                return Some((exe.to_string_lossy().into_owned(), vec![]));
            }
        }
        None
    }
    #[cfg(not(windows))]
    {
        for bin in ["ModrinthApp", "modrinth-app"] {
            if let Some(p) = std::env::var_os("PATH")
                .and_then(|p| std::env::split_paths(&p).map(|d| d.join(bin)).find(|f| f.is_file()))
            {
                return Some((p.to_string_lossy().into_owned(), vec![]));
            }
        }
        let flatpak = std::process::Command::new("flatpak")
            .args(["info", "com.modrinth.ModrinthApp"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
        if flatpak.map(|s| s.success()).unwrap_or(false) {
            return Some(("flatpak".into(), vec!["run".into(), "com.modrinth.ModrinthApp".into()]));
        }
        None
    }
}

/// Paket an die Modrinth-App übergeben: Sie zeigt dann ihren Import-Dialog.
/// Vapafane schreibt dabei nichts in die Dateien der App.
pub fn open_mrpack(path: &Path) -> Result<()> {
    // Im Testmodus nie die echte App öffnen – nur zeigen, was aufgerufen würde
    if config::test_home().is_some() {
        let how = modrinth_app()
            .map(|(c, a)| format!("{c} {}", a.join(" ")))
            .unwrap_or_else(|| "(keine App)".into());
        eprintln!("Testmodus – würde öffnen: {how} {}", path.display());
        return Ok(());
    }
    #[cfg(windows)]
    {
        // Dateiverknüpfung (.mrpack → Modrinth App)
        let st = util::command("cmd").args(["/C", "start", ""]).arg(path).status()?;
        if st.success() {
            return Ok(());
        }
    }
    #[cfg(not(windows))]
    if let Some((cmd, mut args)) = modrinth_app() {
        args.push(path.to_string_lossy().into_owned());
        std::process::Command::new(cmd)
            .args(args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;
        return Ok(());
    }
    err("Die Modrinth-App wurde nicht gefunden.")
}

/// Modrinth-App ohne Paket öffnen (für „Spielen“).
pub fn open_modrinth_app() -> Result<()> {
    if let Some((cmd, args)) = modrinth_app() {
        std::process::Command::new(cmd)
            .args(args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;
        return Ok(());
    }
    err("Die Modrinth-App wurde nicht gefunden. Download: https://modrinth.com/app")
}

/// Aus der Einladung ein Modrinth-Paket (.mrpack) bauen und – wenn gewünscht – öffnen.
///
/// `update`: Gibt es in der Modrinth-App schon eine Instanz aus einem früheren Paket dieses
/// Servers, wird das Paket für deren „Modpack neu installieren“ gebaut (Welten, Optionen usw.
/// bleiben dort, die Server-Liste der Instanz wird übernommen) und nicht als neue Instanz
/// geöffnet. Sonst entsteht beim Import eine neue Instanz – mit den Einstellungen aus `take`.
pub fn install_mrpack(
    inv: &Invite,
    zip: Option<&Path>,
    choices: &Choices,
    update: bool,
    open: bool,
    progress: Progress,
) -> Result<Value> {
    let Choices { selected, take } = choices;
    let take = *take;
    let kind = kind_of(inv);
    let instance = if update { modrinth_existing(inv).into_iter().next() } else { None };
    let dir = default_dir(&inv.name);
    let meta_path = dir.join(META);
    if !meta_path.exists() && dir.exists() && fs::read_dir(&dir)?.next().is_some() {
        return err(format!(
            "Der Ordner {} ist nicht leer und gehört nicht zu Vapafane.",
            dir.display()
        ));
    }
    fs::create_dir_all(&dir)?;
    if !meta_path.exists() {
        mark_incomplete(inv, &dir, "modrinth")?;
    }
    // War dieser Client vorher für den Minecraft-Launcher eingerichtet? Dessen Profil und Mods entfernen
    // (Spielstände, Einstellungen usw. im Ordner bleiben).
    if let Some(old) = fs::read_to_string(&meta_path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        && let Some(key) = old.get("profile_key").and_then(|k| k.as_str())
    {
        for f in profile_files() {
            remove_profile(&f, key)?;
        }
        for f in old
            .get("mods")
            .and_then(|m| m.as_array())
            .into_iter()
            .flatten()
            .filter_map(|f| f.as_str())
        {
            let _ = fs::remove_file(dir.join("mods").join(Path::new(f).file_name().unwrap_or_default()));
        }
    }
    progress(0.05, "Prüfe die Mods …");
    let all = plan_mods(inv, zip, selected)?;

    let mut files = vec![];
    let mut overrides: Vec<(String, Vec<u8>)> = vec![];
    let mut manual = vec![];
    let total = all.len().max(1) as f64;
    for (i, m) in all.iter().enumerate() {
        progress(0.1 + 0.7 * i as f64 / total, &format!("Bereite {} vor …", m.title));
        let name = updater::safe_filename(&m.file).unwrap_or_default();
        match m.source.as_str() {
            // Modrinth: nur Verweis mit Prüfsummen – die App lädt selbst
            "modrinth" => files.push(json!({
                "path": format!("mods/{name}"),
                "hashes": {"sha1": m.sha1, "sha512": m.sha512},
                "env": {"client": "required", "server": "unsupported"},
                "downloads": [m.url],
                "fileSize": m.size,
            })),
            // Datei aus der Einladungs-Datei: direkt einpacken
            "file" => {
                let Some(zp) = zip else {
                    manual.push(json!({"title": m.title, "page": Value::Null, "file": m.file, "reason": "nur in der Einladungs-Datei enthalten"}));
                    continue;
                };
                let mut a = zip::ZipArchive::new(fs::File::open(zp)?)?;
                let mut data = vec![];
                a.by_name(&format!("mods/{}", m.file))?.read_to_end(&mut data)?;
                overrides.push((format!("overrides/mods/{name}"), data));
            }
            // CurseForge: nur wenn der Autor Downloads über andere Programme erlaubt
            _ => match &m.url {
                Some(url) => {
                    let tmp = dir.join(format!(".{name}.part-cf"));
                    net::download(url, &tmp, net::Expect { sha1: m.sha1.as_deref(), ..Default::default() }, &|_| {})?;
                    overrides.push((format!("overrides/mods/{name}"), fs::read(&tmp)?));
                    let _ = fs::remove_file(&tmp);
                }
                None => manual.push(json!({"title": m.title, "page": m.page, "file": m.file, "reason": "Download nur im Browser (CurseForge)"})),
            },
        }
    }
    let mut deps = serde_json::Map::new();
    deps.insert("minecraft".into(), json!(inv.mc));
    match kind {
        Kind::Fabric => {
            let lv = if inv.loader_version.is_empty() {
                net::fabric_server_jar_url(&inv.mc)?.1
            } else {
                inv.loader_version.clone()
            };
            deps.insert("fabric-loader".into(), json!(lv));
        }
        Kind::NeoForge => {
            deps.insert("neoforge".into(), json!(inv.loader_version));
        }
        _ => {}
    }
    let index = json!({
        "formatVersion": 1,
        "game": "minecraft",
        "versionId": chrono::Local::now().format("%Y.%m.%d-%H%M").to_string(),
        "name": format!("{} (Vapafane)", inv.name),
        "summary": format!("Eingerichtet von Vapafane für den Server {} ({})", inv.name, inv.address),
        "files": files,
        "dependencies": deps,
    });
    // Einstellungen übernehmen – nur für eine neue Instanz (eine vorhandene behält ihre eigenen)
    let take = if instance.is_some() { None } else { take };
    let taken_files = match take {
        Some(t) => {
            progress(0.82, "Sammle die Einstellungen …");
            transfer::gather(t)?
        }
        None => Default::default(),
    };
    // Server-Liste: die der vorhandenen Instanz bzw. der Quelle, dazu dieser Server
    let lists: Vec<Vec<u8>> = match &instance {
        Some(i) => fs::read(i.join("servers.dat")).ok().into_iter().collect(),
        None => take.and_then(Take::servers).into_iter().collect(),
    };
    let servers = servers_dat(&lists, &inv.name, &game_address(inv));
    let taken: Option<Value>;

    progress(0.85, "Packe das Modrinth-Paket …");
    let pack = dir.join(format!("{}.mrpack", util::safe_name(&inv.name)));
    let tmp = pack.with_extension("part");
    {
        let mut z = zip::ZipWriter::new(fs::File::create(&tmp)?);
        let o = zip::write::SimpleFileOptions::default();
        z.start_file("modrinth.index.json", o)?;
        z.write_all(serde_json::to_string_pretty(&index)?.as_bytes())?;
        // Übernommene Einstellungen (ohne Mods und Server-Liste – die kommen hier vom Server)
        let copied = transfer::add_overrides(
            &mut z,
            &taken_files
                .into_iter()
                .filter(|(rel, _)| !rel.starts_with("mods/"))
                .collect(),
        )?;
        taken = take.map(|t| copied.json(&t.dir, t.has("servers")));
        z.start_file("overrides/servers.dat", o)?;
        z.write_all(&servers)?;
        for (path, data) in &overrides {
            z.start_file(path.as_str(), o.compression_method(zip::CompressionMethod::Stored))?;
            z.write_all(data)?;
        }
        z.finish()?;
    }
    fs::rename(&tmp, &pack)?;

    let opened = instance.is_none() && open && open_mrpack(&pack).is_ok();
    let meta = json!({
        "invite": inv,
        "target": "modrinth",
        "mrpack": pack,
        "mods": all.iter().filter(|m| m.source == "modrinth" || overrides.iter().any(|(p, _)| p.ends_with(&m.file))).map(|m| m.file.clone()).collect::<Vec<_>>(),
        "manual": manual,
        "updated": chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
    });
    util::write_atomic(&meta_path, serde_json::to_string_pretty(&meta)?.as_bytes())?;
    progress(1.0, "Fertig.");
    Ok(json!({
        "dir": dir,
        "target": "modrinth",
        "mrpack": pack,
        "opened": opened,
        // So heißt die Instanz in der App (Name aus dem Paket); der Ordner nur zur Sicherheit
        "update": instance.as_ref().map(|i| json!({
            "name": format!("{} (Vapafane)", inv.name),
            "folder": i.file_name().map(|n| n.to_string_lossy().into_owned()),
            "dir": i,
        })),
        "taken": taken,
        "app": modrinth_app().is_some(),
        "installed": meta["mods"],
        "manual": manual,
        "notes": [],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses() {
        assert_eq!(split_address("example.org").unwrap(), ("example.org".into(), 25565));
        assert_eq!(split_address("1.2.3.4:25570").unwrap(), ("1.2.3.4".into(), 25570));
        assert_eq!(split_address("[::1]:25566").unwrap(), ("::1".into(), 25566));
        assert!(split_address("host:abc").is_err());
    }

    #[test]
    fn varints() {
        for v in [0, 1, 127, 128, 255, 25565, -1, i32::MAX] {
            let mut b = vec![];
            write_varint(&mut b, v);
            assert_eq!(read_varint(&mut &b[..]).unwrap(), v);
        }
    }

    #[test]
    fn servers_dat_layout() {
        let b = servers_dat(&[], "Welt", "a.b:1");
        assert_eq!(&b[..3], &[10, 0, 0]);
        assert!(b.windows(4).any(|w| w == b"Welt"));
        assert_eq!(*b.last().unwrap(), 0);
        // eigene Einträge bleiben beim erneuten Einrichten
        let mine = nbt::servers_with(&[], "Mein Server", "mein.de");
        let again = servers_dat(&[mine], "Welt", "a.b:1");
        assert_eq!(nbt::addresses(&again), vec!["a.b:1", "mein.de"]);
    }

    #[test]
    fn version_check() {
        assert!(version_matches(&json!({"version": "26.3"}), "26.3"));
        assert!(version_matches(&json!({"version": "Paper 1.21.4"}), "1.21.4"));
        assert!(!version_matches(&json!({"version": "26.2"}), "26.3"));
    }
}
