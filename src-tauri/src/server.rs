//! Alles rund um einen Server-Ordner: server.properties, .mcpanel.env,
//! Erkennung von Typ/Version und der Startbefehl.

use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::config::ENV_NAME;
use crate::java;
use crate::util::{self, Result};

// ---------------------------------------------------------------- server.properties

pub fn read_properties(dir: &Path) -> BTreeMap<String, String> {
    let mut props = BTreeMap::new();
    if let Ok(text) = fs::read_to_string(dir.join("server.properties")) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                props.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }
    props
}

/// Einzelne Werte ändern, Rest der Datei (Kommentare, Reihenfolge) bleibt.
pub fn write_properties(dir: &Path, updates: &BTreeMap<String, String>) -> Result<()> {
    let path = dir.join("server.properties");
    let text = fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let mut seen = std::collections::HashSet::new();
    for line in lines.iter_mut() {
        if line.starts_with('#') {
            continue;
        }
        if let Some((k, _)) = line.split_once('=') {
            let k = k.to_string();
            if let Some(v) = updates.get(&k) {
                *line = format!("{k}={}", clean_value(v));
                seen.insert(k);
            }
        }
    }
    for (k, v) in updates {
        if !seen.contains(k) {
            lines.push(format!("{k}={}", clean_value(v)));
        }
    }
    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

/// Zeilenumbrüche würden die Datei kaputt machen.
fn clean_value(v: &str) -> String {
    v.replace(['\r', '\n'], " ")
}

// ---------------------------------------------------------------- .mcpanel.env

/// Liest `.mcpanel.env`. Die Datei wird nur gelesen, nie ausgeführt.
pub fn read_env(dir: &Path) -> BTreeMap<String, String> {
    let mut env: BTreeMap<String, String> = [
        ("MC_JAVA", java::default_java()),
        ("MC_XMX", "4G".into()),
        ("MC_XMS", "1G".into()),
        ("MC_JAVA_OPTS", "-XX:+UseG1GC -XX:MaxGCPauseMillis=200".into()),
        ("MC_JAR", String::new()),
        ("MC_VERSION", String::new()),
        ("MC_TYPE", String::new()),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
    static KEY: LazyLock<Option<Regex>> = LazyLock::new(|| crate::util::regex(r"^MC_[A-Z_]+$"));
    if let Ok(text) = fs::read_to_string(dir.join(ENV_NAME)) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let k = k.trim();
                if KEY.as_ref().is_some_and(|r| r.is_match(k)) {
                    let v = v.trim();
                    let v = v.strip_prefix('"').unwrap_or(v);
                    let v = v.strip_suffix('"').unwrap_or(v);
                    env.insert(k.to_string(), v.to_string());
                }
            }
        }
    }
    env
}

pub fn write_env(dir: &Path, updates: &[(&str, String)]) -> Result<()> {
    let mut env = read_env(dir);
    for (k, v) in updates {
        env.insert(k.to_string(), clean_value(v).replace('"', ""));
    }
    let mut lines = vec![
        "# Einstellungen des Bedienfensters für diesen Server".to_string(),
        "# (wird beim Start gelesen, nicht ausgeführt)".to_string(),
    ];
    for (k, v) in &env {
        if v.contains(' ') {
            lines.push(format!("{k}=\"{v}\""));
        } else {
            lines.push(format!("{k}={v}"));
        }
    }
    fs::write(dir.join(ENV_NAME), lines.join("\n") + "\n")?;
    Ok(())
}

/// RAM-Limit (z. B. "8G") in GB.
pub fn xmx_gb(env: &BTreeMap<String, String>) -> Option<u64> {
    let v = env.get("MC_XMX")?.trim().to_uppercase();
    if let Some(g) = v.strip_suffix('G') {
        return g.parse().ok();
    }
    if let Some(m) = v.strip_suffix('M') {
        return m.parse::<u64>().ok().map(|m| (m / 1024).max(1));
    }
    None
}

/// Namen aus whitelist.json / ops.json.
pub fn read_json_names(path: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str::<Vec<serde_json::Value>>(&t).ok())
        .unwrap_or_default()
        .iter()
        .filter_map(|e| e.get("name").and_then(|n| n.as_str()).map(str::to_string))
        .collect();
    names.sort_by_key(|n| n.to_lowercase());
    names
}

pub fn eula_accepted(dir: &Path) -> bool {
    fs::read_to_string(dir.join("eula.txt"))
        .map(|t| t.lines().any(|l| l.trim().eq_ignore_ascii_case("eula=true")))
        .unwrap_or(false)
}

pub fn accept_eula(dir: &Path) -> Result<()> {
    let now = chrono::Local::now().format("%d.%m.%Y %H:%M");
    fs::write(
        dir.join("eula.txt"),
        format!("# Akzeptiert über Vapafane am {now}\neula=true\n"),
    )?;
    Ok(())
}

pub fn count_mods(dir: &Path) -> usize {
    fs::read_dir(dir.join("mods"))
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().ends_with(".jar"))
                .count()
        })
        .unwrap_or(0)
}

// ---------------------------------------------------------------- Erkennung

#[derive(Clone, Debug, Default, Serialize)]
pub struct ServerInfo {
    /// fabric, vanilla, paper, quilt, forge, neoforge oder "unbekannt"
    #[serde(rename = "type")]
    pub kind: String,
    pub jar: String,
    pub version: String,
    pub loader: String,
}

fn read_zip_entry(jar: &Path, name: &str) -> Option<Vec<u8>> {
    let f = fs::File::open(jar).ok()?;
    let mut z = zip::ZipArchive::new(f).ok()?;
    let mut e = z.by_name(name).ok()?;
    let mut buf = Vec::new();
    e.read_to_end(&mut buf).ok()?;
    Some(buf)
}

pub fn jar_manifest(jar: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let Some(data) = read_zip_entry(jar, "META-INF/MANIFEST.MF") else {
        return out;
    };
    let text = String::from_utf8_lossy(&data).replace("\r\n", "\n").replace("\n ", "");
    for line in text.lines() {
        if let Some((k, v)) = line.split_once(": ") {
            out.insert(k.to_string(), v.to_string());
        }
    }
    out
}

pub fn detect_server(dir: &Path) -> ServerInfo {
    let mut info = ServerInfo {
        kind: "unbekannt".into(),
        ..Default::default()
    };
    let Ok(rd) = fs::read_dir(dir) else {
        return info;
    };
    let files: Vec<String> = rd
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let mut jars: Vec<&String> = files.iter().filter(|f| f.ends_with(".jar")).collect();

    if files.iter().any(|f| f == "fabric-server-launch.jar") {
        info.kind = "fabric".into();
        info.jar = "fabric-server-launch.jar".into();
    } else if forge_args_file(dir).is_some() {
        let neo = dir.join("libraries").join("net").join("neoforged").is_dir();
        info.kind = if neo { "neoforge" } else { "forge" }.into();
    } else {
        jars.sort_by_key(|j| std::cmp::Reverse(fs::metadata(dir.join(j)).map(|m| m.len()).unwrap_or(0)));
        for jar in &jars {
            let main = jar_manifest(&dir.join(jar))
                .get("Main-Class")
                .cloned()
                .unwrap_or_default();
            let kind = if main.contains("fabricmc") {
                "fabric"
            } else if main.contains("paperclip") || main.contains("papermc") {
                "paper"
            } else if main.contains("quilt") {
                "quilt"
            } else if main.starts_with("net.minecraft") || main.contains("bundler") {
                "vanilla"
            } else {
                continue;
            };
            info.kind = kind.into();
            info.jar = (*jar).clone();
            break;
        }
    }

    // Version: zuerst aus dem Log, dann aus .mcpanel.env, dann aus den Jars
    static FABRIC_RE: LazyLock<Option<Regex>> =
        LazyLock::new(|| crate::util::regex(r"Loading Minecraft (\S+) with Fabric Loader (\S+)"));
    static VANILLA_RE: LazyLock<Option<Regex>> =
        LazyLock::new(|| crate::util::regex(r"Starting minecraft server version (\S+)"));
    if let Ok(mut f) = fs::File::open(dir.join("logs").join("latest.log")) {
        let mut buf = vec![0u8; 6000];
        let n = f.read(&mut buf).unwrap_or(0);
        let head = String::from_utf8_lossy(&buf[..n]);
        if let Some(c) = FABRIC_RE.as_ref().and_then(|r| r.captures(&head)) {
            info.version = c[1].to_string();
            info.loader = c[2].to_string();
        } else if let Some(c) = VANILLA_RE.as_ref().and_then(|r| r.captures(&head)) {
            info.version = c[1].to_string();
        }
    }
    let env = read_env(dir);
    if let Some(v) = env.get("MC_VERSION").filter(|v| !v.is_empty()) {
        info.version = v.clone();
    }
    if info.loader.is_empty() && info.kind != "vanilla" {
        info.loader = env.get("MC_LOADER_VERSION").cloned().unwrap_or_default();
    }
    if info.version.is_empty() && info.kind == "fabric" && !info.jar.is_empty() {
        static INTER: LazyLock<Option<Regex>> = LazyLock::new(|| crate::util::regex(r"intermediary/([^/\s]+)/"));
        let cp = jar_manifest(&dir.join(&info.jar))
            .get("Class-Path")
            .cloned()
            .unwrap_or_default();
        if let Some(c) = INTER.as_ref().and_then(|r| r.captures(&cp)) {
            info.version = c[1].to_string();
        }
    }
    if info.version.is_empty() {
        for jar in &jars {
            if let Some(data) = read_zip_entry(&dir.join(jar), "version.json")
                && let Ok(v) = serde_json::from_slice::<serde_json::Value>(&data)
                && let Some(id) = v.get("id").and_then(|i| i.as_str())
            {
                info.version = id.to_string();
                break;
            }
        }
    }
    info
}

/// Forge/NeoForge: Datei mit den Startargumenten (statt run.sh/run.bat auszuführen).
pub fn forge_args_file(dir: &Path) -> Option<PathBuf> {
    let want = if cfg!(windows) { "win_args.txt" } else { "unix_args.txt" };
    for base in [["net", "neoforged", "neoforge"], ["net", "minecraftforge", "forge"]] {
        let mut p = dir.join("libraries");
        for b in base {
            p.push(b);
        }
        // Zuerst die Version, die laut .mcpanel.env installiert ist
        let installed = read_env(dir).get("MC_LOADER_VERSION").cloned().unwrap_or_default();
        if !installed.is_empty() && p.join(&installed).join(want).is_file() {
            return Some(p.join(&installed).join(want));
        }
        if let Ok(rd) = fs::read_dir(&p) {
            let mut versions: Vec<PathBuf> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            let name = |v: &PathBuf| {
                v.file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default()
            };
            versions.sort_by(|a, b| util::cmp_versions(&name(a), &name(b)));
            if let Some(f) = versions.iter().rev().map(|v| v.join(want)).find(|f| f.is_file()) {
                return Some(f);
            }
        }
    }
    None
}

/// Macht einen Server-Ordner bereit: RCON an (mit Zufallspasswort), Startdatei merken.
pub fn prepare_server(dir: &Path, accept: bool) -> Result<(ServerInfo, Vec<String>)> {
    let mut notes = Vec::new();
    let props = read_properties(dir);
    let mut updates = BTreeMap::new();
    if props.get("enable-rcon").map(String::as_str) != Some("true") {
        updates.insert("enable-rcon".into(), "true".into());
        notes.push("RCON eingeschaltet (braucht Vapafane für Befehle).".into());
    }
    if props.get("rcon.password").map(|p| p.is_empty()).unwrap_or(true) {
        updates.insert("rcon.password".into(), util::random_hex(12));
    }
    if props.get("rcon.port").map(|p| p.is_empty()).unwrap_or(true) {
        updates.insert("rcon.port".into(), "25575".into());
    }
    if !updates.is_empty() {
        write_properties(dir, &updates)?;
    }
    if accept {
        accept_eula(dir)?;
    }
    let info = detect_server(dir);
    let mut env = vec![("MC_JAR", info.jar.clone()), ("MC_TYPE", info.kind.clone())];
    if !info.version.is_empty() {
        env.push(("MC_VERSION", info.version.clone()));
    }
    write_env(dir, &env)?;
    if info.jar.is_empty() && info.kind != "forge" && info.kind != "neoforge" {
        notes.push("Keine Server-Jar gefunden. Bitte die Jar in den Ordner legen.".into());
    }
    Ok((info, notes))
}

/// Steckt der Server in einem einzelnen Unterordner (typisch bei ZIPs), diesen finden.
pub fn find_server_root(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();
    for _ in 0..3 {
        let Ok(rd) = fs::read_dir(&path) else {
            return path;
        };
        let entries: Vec<_> = rd
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_name().to_string_lossy().starts_with("__MACOSX"))
            .collect();
        let has_server = entries.iter().any(|e| {
            let n = e.file_name().to_string_lossy().into_owned();
            n == "server.properties" || n.ends_with(".jar")
        });
        if has_server {
            return path;
        }
        if entries.len() == 1 && entries[0].path().is_dir() {
            path = entries[0].path();
        } else {
            return path;
        }
    }
    path
}

// ---------------------------------------------------------------- Startbefehl

/// Programm und Argumente, mit denen der Server gestartet wird.
pub fn launch_command(dir: &Path) -> Result<(String, Vec<String>)> {
    let env = read_env(dir);
    let java = env.get("MC_JAVA").cloned().unwrap_or_else(java::default_java);
    let java = java::resolve_java(&java);
    let mut args = vec![
        format!("-Xms{}", env.get("MC_XMS").map(String::as_str).unwrap_or("1G")),
        format!("-Xmx{}", env.get("MC_XMX").map(String::as_str).unwrap_or("4G")),
    ];
    if let Some(opts) = env.get("MC_JAVA_OPTS") {
        args.extend(opts.split_whitespace().map(str::to_string));
    }
    let jar = env.get("MC_JAR").cloned().unwrap_or_default();
    if !jar.is_empty() && dir.join(&jar).is_file() {
        args.push("-jar".into());
        args.push(jar);
    } else if let Some(argfile) = forge_args_file(dir) {
        if dir.join("user_jvm_args.txt").is_file() {
            args.push("@user_jvm_args.txt".into());
        }
        let rel = argfile.strip_prefix(dir).unwrap_or(&argfile);
        args.push(format!("@{}", rel.to_string_lossy()));
    } else {
        let info = detect_server(dir);
        if info.jar.is_empty() {
            return util::err(format!("Keine Server-Jar gefunden (MC_JAR={jar})"));
        }
        args.push("-jar".into());
        args.push(info.jar);
    }
    args.push("nogui".into());
    Ok((java, args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_enables_rcon_and_keeps_comments() {
        let dir = std::env::temp_dir().join(format!("vapafane-prep-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("server.properties"),
            "#Kommentar\nmotd=Hallo\nenable-rcon=false\n",
        )
        .unwrap();
        let (_, notes) = prepare_server(&dir, true).unwrap();
        let text = fs::read_to_string(dir.join("server.properties")).unwrap();
        assert!(text.starts_with("#Kommentar\nmotd=Hallo\nenable-rcon=true\n"));
        let props = read_properties(&dir);
        assert_eq!(props["rcon.password"].len(), 24);
        assert_eq!(props["rcon.port"], "25575");
        assert!(eula_accepted(&dir));
        assert!(notes.iter().any(|n| n.contains("RCON")));
        // .mcpanel.env wird gelesen, aber Shell-Kram nicht ausgeführt
        fs::write(
            dir.join(ENV_NAME),
            "MC_XMX=6G\nMC_JAVA_OPTS=\"-XX:+UseG1GC $(rm -rf /)\"\nPATH=/evil\n",
        )
        .unwrap();
        let env = read_env(&dir);
        assert_eq!(xmx_gb(&env), Some(6));
        assert!(!env.contains_key("PATH"));
        fs::remove_dir_all(&dir).unwrap();
    }
}
