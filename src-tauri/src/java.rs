//! Java finden und die Version erkennen (Linux und Windows).

use regex::Regex;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::util;

pub fn default_java() -> String {
    if cfg!(windows) {
        "java".into()
    } else {
        "/usr/bin/java".into()
    }
}

/// Unter Windows steht oft nur "java" in der Einstellung – dann das beste
/// installierte Java nehmen, falls java nicht im PATH liegt.
pub fn resolve_java(java: &str) -> String {
    if java_major(java) > 0 {
        return java.to_string();
    }
    installed_javas()
        .into_iter()
        .next_back()
        .map(|(_, p)| p)
        .unwrap_or_else(|| java.to_string())
}

/// Hauptversion (z. B. 21) oder 0, wenn Java nicht startet.
pub fn java_major(java: &str) -> u32 {
    let Ok(out) = util::command(java).arg("-version").output() else {
        return 0;
    };
    let text = String::from_utf8_lossy(&out.stderr).into_owned() + &String::from_utf8_lossy(&out.stdout);
    parse_major(&text)
}

fn parse_major(text: &str) -> u32 {
    static RE: LazyLock<Option<Regex>> = LazyLock::new(|| crate::util::regex(r#"version "(\d+)(?:\.(\d+))?"#));
    let Some(c) = RE.as_ref().and_then(|r| r.captures(text)) else {
        return 0;
    };
    let major: u32 = c[1].parse().unwrap_or(0);
    if major == 1 {
        c.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0)
    } else {
        major
    }
}

fn exe_name() -> &'static str {
    if cfg!(windows) { "java.exe" } else { "java" }
}

fn candidates() -> Vec<PathBuf> {
    let mut out = vec![];
    if cfg!(unix) {
        out.push(PathBuf::from("/usr/bin/java"));
    }
    if let Some(home) = std::env::var_os("JAVA_HOME") {
        out.push(PathBuf::from(home).join("bin").join(exe_name()));
    }
    let mut roots: Vec<PathBuf> = vec![];
    if cfg!(windows) {
        for var in ["ProgramFiles", "ProgramFiles(x86)", "LOCALAPPDATA"] {
            if let Some(pf) = std::env::var_os(var) {
                let pf = PathBuf::from(pf);
                for vendor in [
                    "Java",
                    "Eclipse Adoptium",
                    "Microsoft",
                    "Zulu",
                    "BellSoft",
                    "Amazon Corretto",
                    "Programs\\Eclipse Adoptium",
                ] {
                    roots.push(pf.join(vendor));
                }
            }
        }
    } else {
        roots.push(PathBuf::from("/usr/lib/jvm"));
        roots.push(PathBuf::from("/opt"));
    }
    if let Some(h) = dirs::home_dir() {
        roots.push(h.join(".jdks"));
    }
    for root in roots {
        if let Ok(rd) = std::fs::read_dir(&root) {
            let mut dirs: Vec<PathBuf> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            dirs.sort();
            for d in dirs {
                out.push(d.join("bin").join(exe_name()));
            }
        }
    }
    out
}

/// Installierte Java-Versionen: Hauptversion → Pfad (aufsteigend sortiert).
pub fn installed_javas() -> BTreeMap<u32, String> {
    let mut found: BTreeMap<u32, String> = BTreeMap::new();
    let mut seen = std::collections::HashSet::new();
    for c in candidates() {
        if !c.is_file() {
            continue;
        }
        let real = util::canonical(&c);
        if !seen.insert(real) {
            continue;
        }
        let path = c.to_string_lossy().into_owned();
        let major = java_major(&path);
        if major > 0 {
            found.entry(major).or_insert(path);
        }
    }
    if found.is_empty() && java_major("java") > 0 {
        found.insert(java_major("java"), "java".into());
    }
    found
}

/// Pfad zu einem Java mit mindestens dieser Version.
pub fn java_at_least(need: u32) -> Option<String> {
    installed_javas().into_iter().find(|(m, _)| *m >= need).map(|(_, p)| p)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_versions() {
        assert_eq!(super::parse_major(r#"openjdk version "25.0.4.1" 2026-08-18"#), 25);
        assert_eq!(super::parse_major(r#"java version "1.8.0_402""#), 8);
        assert_eq!(super::parse_major(r#"openjdk version "21" 2023-09-19"#), 21);
    }
}
