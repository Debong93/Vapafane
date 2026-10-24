//! Prüft die Übersetzungen gegen den Quelltext:
//! Jeder Schlüssel aus `t!("…")` bzw. `text("…", None)` steht in `locales/de.ftl`,
//! und kein Schlüssel in de.ftl ist ungenutzt (vergessene Reste).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
}

/// Alle Rust-Dateien unter src/ der Komponenten (ohne target/).
fn quelldateien() -> Vec<PathBuf> {
    let mut out = vec![];
    for teil in ["src-tauri/src", "frontend/src", "shared/src"] {
        sammle(&repo().join(teil), &mut out);
    }
    out
}

fn sammle(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.filter_map(|e| e.ok()) {
        let p = e.path();
        if p.is_dir() {
            sammle(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Quelltext ohne Kommentarzeilen, Zeilenumbrüche zu Leerzeichen (für mehrzeilige Aufrufe).
fn flach(text: &str) -> String {
    text.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Schlüssel aus `t!("…")` und `text("…", …)` – auch über Zeilenumbrüche hinweg.
fn aufgerufene_schluessel() -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    for datei in quelldateien() {
        let Ok(text) = fs::read_to_string(&datei) else { continue };
        let text = flach(&text);
        for pat in ["t!(", "::text("] {
            let mut rest = text.as_str();
            while let Some(i) = rest.find(pat) {
                rest = &rest[i + pat.len()..];
                let nach = rest.trim_start();
                if let Some(mitte) = nach.strip_prefix('"')
                    && let Some(end) = mitte.find('"')
                {
                    let key = &mitte[..end];
                    if !key.is_empty() && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                        keys.insert(key.to_string());
                    }
                }
            }
        }
    }
    // Schlüssel, die zur Laufzeit zusammengesetzt werden (format!("set-wert-{v}"))
    for v in ["peaceful", "easy", "normal", "hard", "survival", "creative", "adventure", "spectator"] {
        keys.insert(format!("set-wert-{v}"));
    }
    keys
}

/// Für „ungenutzt“: Ein Schlüssel gilt als benutzt, wenn er irgendwo als "…"-Literal
/// im Quelltext steht (deckt auch Tabellen wie PROPS/KINDS/PARTS ab).
fn literale() -> String {
    quelldateien()
        .iter()
        .filter_map(|d| fs::read_to_string(d).ok())
        .map(|t| flach(&t))
        .collect::<Vec<_>>()
        .join(" ")
}

fn de_schluessel() -> BTreeSet<String> {
    let quelle = fs::read_to_string(repo().join("locales/de.ftl")).unwrap();
    quelle
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .filter_map(|l| {
            let (k, _) = l.split_once('=')?;
            let k = k.trim();
            (!k.is_empty() && !l.starts_with(' ')).then(|| k.to_string())
        })
        .collect()
}

/// Schlüssel, die nur über zusammengesetzte Aufrufe laufen (format!("set-wert-{v}")).
const AUSNAHMEN: &[&str] = &[
    "set-wert-peaceful",
    "set-wert-easy",
    "set-wert-normal",
    "set-wert-hard",
    "set-wert-survival",
    "set-wert-creative",
    "set-wert-adventure",
    "set-wert-spectator",
];

#[test]
fn alle_benutzten_schluessel_sind_uebersetzt() {
    let de = de_schluessel();
    let fehlt: Vec<String> = aufgerufene_schluessel()
        .into_iter()
        .filter(|k| !de.contains(k))
        .collect();
    assert!(fehlt.is_empty(), "In de.ftl fehlen: {fehlt:?}");
}

#[test]
fn keine_ungenutzten_schluessel() {
    let quelltext = literale();
    let ungenutzt: Vec<String> = de_schluessel()
        .into_iter()
        .filter(|k| !quelltext.contains(&format!("\"{k}\"")) && !AUSNAHMEN.contains(&k.as_str()))
        .collect();
    assert!(ungenutzt.is_empty(), "Ungenutzt in de.ftl: {ungenutzt:?}");
}
