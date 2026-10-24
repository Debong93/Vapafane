//! Server-Typen: Fabric, Vanilla, Paper und NeoForge.
//!
//! Für jeden Typ: welche Minecraft-Versionen es gibt, woher die Server-Datei kommt
//! (immer mit Prüfsumme), wie sie eingebaut wird und welche Modrinth-Loader zu den
//! Mods bzw. Plugins passen.

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::archive::Progress;
use crate::net::{self, Expect};
use crate::server;
use crate::util::{self, Error, Result, cmp_versions, err};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Fabric,
    Vanilla,
    Paper,
    NeoForge,
}

impl Kind {
    pub fn parse(s: &str) -> Option<Kind> {
        match s {
            "fabric" => Some(Kind::Fabric),
            "vanilla" => Some(Kind::Vanilla),
            "paper" => Some(Kind::Paper),
            "neoforge" => Some(Kind::NeoForge),
            _ => None,
        }
    }

    /// Typ eines vorhandenen Server-Ordners (None = vom Update-Assistenten nicht unterstützt).
    pub fn of_dir(dir: &Path) -> Option<Kind> {
        Kind::parse(&server::detect_server(dir).kind)
    }

    pub fn id(self) -> &'static str {
        match self {
            Kind::Fabric => "fabric",
            Kind::Vanilla => "vanilla",
            Kind::Paper => "paper",
            Kind::NeoForge => "neoforge",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Kind::Fabric => "Fabric",
            Kind::Vanilla => "Vanilla",
            Kind::Paper => "Paper",
            Kind::NeoForge => "NeoForge",
        }
    }

    /// Ordner mit Mods bzw. Plugins (Vanilla hat keinen).
    pub fn mod_dir(self) -> Option<&'static str> {
        match self {
            Kind::Fabric | Kind::NeoForge => Some("mods"),
            Kind::Paper => Some("plugins"),
            Kind::Vanilla => None,
        }
    }

    /// Passende Loader auf Modrinth.
    pub fn loaders(self) -> &'static [&'static str] {
        match self {
            Kind::Fabric => &["fabric"],
            Kind::NeoForge => &["neoforge"],
            Kind::Paper => &["paper", "purpur", "spigot", "bukkit"],
            Kind::Vanilla => &[],
        }
    }
}

/// Mod- bzw. Plugin-Ordner eines vorhandenen Servers ("mods", wenn unklar).
pub fn mod_dir_of(dir: &Path) -> &'static str {
    Kind::of_dir(dir).and_then(Kind::mod_dir).unwrap_or("mods")
}

/// Eine Server-Datei zum Herunterladen.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Artifact {
    pub url: String,
    pub file: String,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    /// Loader-Version bzw. Build (Fabric-Loader, Paper-Build, NeoForge-Version)
    pub build: String,
    /// Für die Anzeige, z. B. „Paper Build 132“
    pub label: String,
    /// Nur eine Test-/Beta-Version verfügbar
    pub beta: bool,
}

impl Artifact {
    pub fn download(&self, dest: &Path, progress: &dyn Fn(f64)) -> Result<()> {
        net::download(
            &self.url,
            dest,
            Expect {
                sha1: self.sha1.as_deref(),
                sha256: self.sha256.as_deref(),
                sha512: None,
            },
            progress,
        )
    }
}

// ---------------------------------------------------------------- Versionslisten

/// Minecraft-Versionen, für die es diesen Server-Typ gibt (neueste zuerst).
pub fn game_versions(kind: Kind) -> Result<Vec<String>> {
    Ok(match kind {
        Kind::Fabric => net::fabric_game_versions()?,
        Kind::Vanilla => vanilla_versions()?,
        Kind::Paper => paper_versions()?,
        Kind::NeoForge => {
            let mut v: Vec<String> = neoforge_versions()?.into_iter().map(|(_, mc, _)| mc).collect();
            v.sort_by(|a, b| cmp_versions(b, a));
            v.dedup();
            v
        }
    })
}

fn mojang_manifest() -> Result<Value> {
    net::get_json("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
}

fn vanilla_versions() -> Result<Vec<String>> {
    let m = mojang_manifest()?;
    Ok(m.get("versions")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter(|v| v.get("type").and_then(|t| t.as_str()) == Some("release"))
                .filter_map(|v| v.get("id").and_then(|i| i.as_str()).map(str::to_string))
                .take(60)
                .collect()
        })
        .unwrap_or_default())
}

const PAPER_API: &str = "https://fill.papermc.io/v3/projects/paper";

fn paper_versions() -> Result<Vec<String>> {
    let p = net::get_json(PAPER_API)?;
    let mut out: Vec<String> = p
        .get("versions")
        .and_then(|v| v.as_object())
        .map(|groups| {
            groups
                .values()
                .filter_map(|list| list.as_array())
                .flatten()
                .filter_map(|v| v.as_str())
                .filter(|v| !v.contains('-')) // keine Vorabversionen (rc/pre)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    out.sort_by(|a, b| cmp_versions(b, a));
    Ok(out)
}

/// Alle NeoForge-Versionen als (NeoForge-Version, Minecraft-Version, Beta?).
fn neoforge_versions() -> Result<Vec<(String, String, bool)>> {
    static RE: LazyLock<Option<Regex>> = LazyLock::new(|| crate::util::regex(r"<version>([^<]+)</version>"));
    let xml = net::get_text("https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml")?;
    Ok(RE
        .iter()
        .flat_map(|r| r.captures_iter(&xml))
        .filter_map(|c| {
            let v = c[1].to_string();
            let mc = neoforge_mc_version(&v)?;
            let beta = v.contains('-');
            Some((v, mc, beta))
        })
        .collect())
}

/// NeoForge-Version → Minecraft-Version:
/// alt `21.4.161` → 1.21.4, `21.0.5` → 1.21; neu `26.3.0.88` → 26.3, `26.1.2.121` → 26.1.2.
pub fn neoforge_mc_version(v: &str) -> Option<String> {
    let base = v.split('-').next()?;
    let p: Vec<&str> = base.split('.').collect();
    if p.iter().any(|x| x.parse::<u32>().is_err()) {
        return None;
    }
    let major: u32 = p[0].parse().ok()?;
    if major >= 26 {
        if p.len() < 4 {
            return None;
        }
        return Some(if p[2] == "0" {
            format!("{}.{}", p[0], p[1])
        } else {
            format!("{}.{}.{}", p[0], p[1], p[2])
        });
    }
    if p.len() != 3 || major < 20 {
        return None;
    }
    Some(if p[1] == "0" {
        format!("1.{}", p[0])
    } else {
        format!("1.{}.{}", p[0], p[1])
    })
}

// ---------------------------------------------------------------- Server-Dateien

/// Neueste passende Server-Datei für eine Minecraft-Version.
pub fn server_artifact(kind: Kind, game: &str) -> Result<Artifact> {
    match kind {
        Kind::Fabric => {
            let (url, loader) = net::fabric_server_jar_url(game)?;
            Ok(Artifact {
                url,
                file: "fabric-server-launch.jar".into(),
                label: format!("Fabric {loader}"),
                build: loader,
                ..Default::default()
            })
        }
        Kind::Vanilla => {
            let m = mojang_manifest()?;
            let url = m
                .get("versions")
                .and_then(|v| v.as_array())
                .and_then(|a| a.iter().find(|v| v.get("id").and_then(|i| i.as_str()) == Some(game)))
                .and_then(|v| v.get("url"))
                .and_then(|u| u.as_str())
                .ok_or_else(|| Error(format!("Minecraft {game} gibt es bei Mojang nicht.")))?;
            let v = net::get_json(url)?;
            let s = v
                .pointer("/downloads/server")
                .ok_or_else(|| Error(crate::sprache::t!("flv-err-kein-server", mc = game)))?;
            Ok(Artifact {
                url: s.get("url").and_then(|u| u.as_str()).unwrap_or_default().to_string(),
                file: "server.jar".into(),
                sha1: s.get("sha1").and_then(|h| h.as_str()).map(str::to_string),
                build: game.into(),
                label: format!("Vanilla {game}"),
                ..Default::default()
            })
        }
        Kind::Paper => {
            let builds = net::get_json(&format!("{PAPER_API}/versions/{game}/builds"))?;
            let builds = builds.as_array().cloned().unwrap_or_default();
            let id = |b: &Value| b.get("id").and_then(|i| i.as_u64()).unwrap_or(0);
            let stable = builds
                .iter()
                .filter(|b| b.get("channel").and_then(|c| c.as_str()) == Some("STABLE"))
                .max_by_key(|b| id(b));
            let (b, beta) = match stable {
                Some(b) => (b, false),
                None => (
                    builds
                        .iter()
                        .max_by_key(|b| id(b))
                        .ok_or_else(|| Error(crate::sprache::t!("flv-err-kein-paper", mc = game)))?,
                    true,
                ),
            };
            let d = b
                .pointer("/downloads/server:default")
                .ok_or_else(|| Error("Paper-Download fehlt".into()))?;
            Ok(Artifact {
                url: d.get("url").and_then(|u| u.as_str()).unwrap_or_default().to_string(),
                file: d
                    .get("name")
                    .and_then(|u| u.as_str())
                    .unwrap_or("paper.jar")
                    .to_string(),
                sha256: d
                    .pointer("/checksums/sha256")
                    .and_then(|h| h.as_str())
                    .map(str::to_string),
                build: id(b).to_string(),
                label: format!("Paper {game} Build {}{}", id(b), if beta { " (Beta)" } else { "" }),
                beta,
                ..Default::default()
            })
        }
        Kind::NeoForge => {
            let all = neoforge_versions()?;
            let mut matching: Vec<&(String, String, bool)> = all.iter().filter(|(_, mc, _)| mc == game).collect();
            if matching.is_empty() {
                return err(crate::sprache::t!("flv-err-kein-neoforge", mc = game));
            }
            let any_stable = matching.iter().any(|(_, _, b)| !b);
            if any_stable {
                matching.retain(|(_, _, b)| !b);
            }
            matching.sort_by(|a, b| cmp_versions(&a.0, &b.0));
            let Some((v, _, beta)) = matching.last() else {
                return err(crate::sprache::t!("flv-err-kein-neoforge", mc = game));
            };
            let base =
                format!("https://maven.neoforged.net/releases/net/neoforged/neoforge/{v}/neoforge-{v}-installer.jar");
            let sha1 = net::get_text(&format!("{base}.sha1"))?
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();
            Ok(Artifact {
                url: base,
                file: format!("neoforge-{v}-installer.jar"),
                sha1: Some(sha1).filter(|s| s.len() == 40),
                build: v.clone(),
                label: format!("NeoForge {v}{}", if *beta { " (Beta)" } else { "" }),
                beta: *beta,
                ..Default::default()
            })
        }
    }
}

/// Installierte Loader-Version bzw. Build laut .mcpanel.env.
pub fn installed_build(dir: &Path) -> String {
    server::read_env(dir)
        .get("MC_LOADER_VERSION")
        .cloned()
        .unwrap_or_default()
}

/// Gibt es für dieselbe Minecraft-Version einen neueren Build (Paper) bzw. ein neueres NeoForge?
pub fn newer_build(kind: Kind, dir: &Path, art: &Artifact) -> bool {
    let have = installed_build(dir);
    match kind {
        Kind::Paper | Kind::NeoForge => have.is_empty() || cmp_versions(&art.build, &have).is_gt(),
        _ => false,
    }
}

/// NeoForge-Installer ausführen: richtet den Server in `dir` ein (Bibliotheken, Startargumente).
pub fn run_neoforge_installer(installer: &Path, dir: &Path, java: &str, progress: Progress) -> Result<()> {
    progress(0.5, &crate::sprache::t!("fort-neoforge"));
    let workdir = installer.parent().unwrap_or(dir);
    let out = util::command(java)
        .arg("-jar")
        .arg(installer)
        .arg("--installServer")
        .arg(dir)
        .current_dir(workdir)
        .output()
        .map_err(|e| Error(format!("NeoForge-Installer konnte nicht starten ({java}): {e}")))?;
    // Logdatei des Installers aufräumen
    if let Ok(rd) = fs::read_dir(workdir) {
        for e in rd.filter_map(|e| e.ok()) {
            if e.file_name().to_string_lossy().ends_with("installer.jar.log") {
                let _ = fs::remove_file(e.path());
            }
        }
    }
    if !out.status.success() {
        let text = String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr);
        let tail: Vec<&str> = text.lines().rev().take(8).collect();
        return err(format!(
            "NeoForge-Installer ist fehlgeschlagen:\n{}",
            tail.into_iter().rev().collect::<Vec<_>>().join("\n")
        ));
    }
    Ok(())
}

/// Ordner für einen neuen Server: frei oder leer?
pub fn target_state(path: &Path) -> &'static str {
    match fs::read_dir(path) {
        Err(_) if !path.exists() => "free",
        Err(_) => "invalid",
        Ok(mut rd) => {
            if rd.next().is_none() {
                "empty"
            } else {
                "nonempty"
            }
        }
    }
}

/// Neuen Server anlegen. `target` muss frei oder leer sein.
pub fn create_server(kind: Kind, name: &str, game: &str, target: &Path, progress: Progress) -> Result<PathBuf> {
    if !matches!(target_state(target), "free" | "empty") {
        return err(format!("Der Ordner {} ist nicht leer.", target.display()));
    }
    let created_dir = !target.exists();
    fs::create_dir_all(target)?;
    let res = (|| -> Result<()> {
        progress(0.05, &crate::sprache::t!("fort-suche-server", was = kind.label(), mc = game));
        let art = server_artifact(kind, game)?;

        // Java: was braucht diese Minecraft-Version?
        progress(0.1, &crate::sprache::t!("fort-pruefe-java"));
        let mut java = crate::java::resolve_java(&crate::java::default_java());
        if let Ok(Some(need)) = net::required_java(game)
            && crate::java::java_major(&java) < need
        {
            java = crate::java::java_at_least(need).ok_or_else(|| {
                Error(crate::sprache::t!(
                    "err-java-noetig",
                    mc = game,
                    version = need,
                    hinweis = crate::manage::java_hint(need)
                ))
            })?;
        }

        progress(0.15, &format!("Lade {} …", art.label));
        let dest = target.join(&art.file);
        art.download(&dest, &|f| progress(0.15 + f * 0.35, &format!("Lade {} …", art.label)))?;
        let mut env = vec![
            ("MC_VERSION", game.to_string()),
            ("MC_TYPE", kind.id().to_string()),
            ("MC_JAVA", java.clone()),
            ("MC_LOADER_VERSION", art.build.clone()),
        ];
        if kind == Kind::NeoForge {
            run_neoforge_installer(&dest, target, &java, progress)?;
            let _ = fs::remove_file(&dest);
            env.push(("MC_JAR", String::new()));
        } else {
            env.push(("MC_JAR", art.file.clone()));
        }
        if let Some(d) = kind.mod_dir() {
            fs::create_dir_all(target.join(d))?;
        }
        // Freie Ports – auch wenn schon ein anderer Server (mit oder ohne Vapafane) läuft
        let taken: Vec<u16> = crate::config::Config::load()
            .servers
            .iter()
            .flat_map(|s| {
                let p = server::read_properties(Path::new(&s.path));
                ["server-port", "rcon.port"]
                    .iter()
                    .filter_map(|k| p.get(*k).and_then(|v| v.parse::<u16>().ok()))
                    .collect::<Vec<_>>()
            })
            .collect();
        let game_port = crate::procs::free_port(25565, &taken).to_string();
        let rcon_port = crate::procs::free_port(25575, &taken).to_string();
        let props: std::collections::BTreeMap<String, String> = [
            ("motd", name),
            ("enable-rcon", "true"),
            ("rcon.port", rcon_port.as_str()),
            ("rcon.password", &util::random_hex(12)),
            ("white-list", "true"),
            ("server-port", game_port.as_str()),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
        server::write_properties(target, &props)?;
        server::write_env(target, &env.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>())?;
        server::prepare_server(target, true)?;
        // prepare_server erkennt Typ und Jar selbst – unsere Angaben haben Vorrang
        server::write_env(target, &env.iter().map(|(k, v)| (*k, v.clone())).collect::<Vec<_>>())?;
        Ok(())
    })();
    if let Err(e) = res {
        if created_dir {
            let _ = fs::remove_dir_all(target);
        } else if let Ok(rd) = fs::read_dir(target) {
            // Ordner war leer: unseren Inhalt wieder entfernen
            for e in rd.filter_map(|e| e.ok()) {
                let p = e.path();
                let _ = if p.is_dir() {
                    fs::remove_dir_all(&p)
                } else {
                    fs::remove_file(&p)
                };
            }
        }
        return Err(e);
    }
    Ok(target.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neoforge_versions_map_to_minecraft() {
        assert_eq!(neoforge_mc_version("21.4.161").as_deref(), Some("1.21.4"));
        assert_eq!(neoforge_mc_version("21.0.5-beta").as_deref(), Some("1.21"));
        assert_eq!(neoforge_mc_version("21.11.38-beta").as_deref(), Some("1.21.11"));
        assert_eq!(neoforge_mc_version("26.3.0.88").as_deref(), Some("26.3"));
        assert_eq!(neoforge_mc_version("26.1.2.121").as_deref(), Some("26.1.2"));
        assert_eq!(neoforge_mc_version("47.1.3"), None);
    }

    /// Braucht Internet: `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn sources_answer() {
        for k in [Kind::Vanilla, Kind::Paper, Kind::NeoForge] {
            let v = game_versions(k).unwrap();
            let a = server_artifact(k, &v[0]).unwrap();
            assert!(a.url.starts_with("https://"), "{k:?}");
            assert!(a.sha1.is_some() || a.sha256.is_some(), "{k:?} ohne Prüfsumme");
        }
    }
}
