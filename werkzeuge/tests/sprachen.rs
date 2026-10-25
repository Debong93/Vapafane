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
        for pat in ["t!(", "::text(", "grund("] {
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

/// Quelltext ohne Test-Module (`#[cfg(test)]` vor `mod …`): Zeilen mit Nummer.
fn ohne_tests(text: &str) -> Vec<(usize, &str)> {
    let zeilen: Vec<&str> = text.lines().collect();
    let mut out = vec![];
    let mut i = 0;
    while i < zeilen.len() {
        let z = zeilen[i].trim();
        if z == "#[cfg(test)]" && zeilen.get(i + 1).is_some_and(|n| n.trim_start().starts_with("mod ")) {
            break; // Test-Modul steht am Dateiende
        }
        out.push((i + 1, zeilen[i]));
        i += 1;
    }
    out
}

/// String-Literale einer Zeile (ohne Raw-Strings), mit Escape-Behandlung.
fn literale_der_zeile(zeile: &str) -> Vec<String> {
    let mut out = vec![];
    let mut zeichen = zeile.chars().peekable();
    let mut vorher = ' ';
    while let Some(c) = zeichen.next() {
        if c == '"' && vorher != '\'' && vorher != '#' {
            let mut s = String::new();
            while let Some(d) = zeichen.next() {
                match d {
                    '\\' => {
                        zeichen.next();
                    }
                    '"' => break,
                    _ => s.push(d),
                }
            }
            out.push(s);
        }
        vorher = c;
    }
    out
}

/// Sieht der Text nach deutscher Oberflächen-Sprache aus? (Umlaute/Anführungszeichen
/// oder typische deutsche Wörter – verglichen wird Wort für Wort.)
fn klingt_deutsch(s: &str) -> bool {
    const ZEICHEN: &[char] = &['ä', 'ö', 'ü', 'Ä', 'Ö', 'Ü', 'ß', '„', '“'];
    const WOERTER: &[&str] = &[
        "nicht", "bitte", "Bitte", "kein", "keine", "Kein", "Keine", "ist", "der", "die", "das",
        "und", "ein", "eine", "mit", "wird", "schon", "geht", "oder", "Unbekannt", "Unbekannter",
        "Unbekanntes", "Unbekannte", "Sekunden", "Spieler", "Datei", "Dateien", "Ordner",
        "Fehler", "falsch", "Benutzung", "Speicherplatz", "gefunden", "Anfragen", "Einladung",
        "eingeben", "Warne", "Stoppe", "Lade", "Sortiere", "Starte", "Entpacke", "Kopiere", "Hole",
        "Suche", "Tausche", "Lese", "doppelt", "passende", "unklar", "Treffer", "Quelle",
        "entfernt", "gestartet", "gestoppt", "ausgeschaltet", "eingeschaltet", "aktualisiert",
        "gespeichert", "angelegt", "fehlt", "Importiere", "Erstelle", "Richte", "Installiere",
        "installieren", "speichern", "Speichern", "Frage", "den", "dem", "von", "nur", "im",
        "Aufgabe", "Befehlszeile", "Probelauf", "eingepackt", "direkt",
    ];
    s.chars().any(|c| ZEICHEN.contains(&c))
        || s.split(|c: char| !c.is_alphanumeric()).any(|w| WOERTER.contains(&w))
}

/// Ruft die Zeile `t!(` bzw. `::text(` auf? (`format!(` zählt nicht.)
fn ruft_uebersetzung(zeile: &str) -> bool {
    zeile.contains("::text(")
        || zeile.match_indices("t!(").any(|(i, _)| {
            i == 0 || !zeile[..i].chars().next_back().is_some_and(|c| c.is_alphanumeric() || c == '_')
        })
}

/// Fest eingebaute Texte, die bewusst so bleiben: Datei- und Ordnernamen, Sprach-Eigennamen.
const FESTE_TEXTE_ERLAUBT: &[&str] = &["_vor_Wiederherstellung_", "Türkçe", "schemas.microsoft.com"];

/// Nutzer-sichtbare Texte gehören in die Sprachdateien, nicht fest in den Quelltext.
#[test]
fn keine_festen_deutschen_texte() {
    let mut funde = vec![];
    for datei in quelldateien() {
        let Ok(text) = fs::read_to_string(&datei) else { continue };
        for (nr, zeile) in ohne_tests(&text) {
            let z = zeile.trim_start();
            if z.starts_with("//") || z.starts_with("#[") || ruft_uebersetzung(zeile) {
                continue;
            }
            for lit in literale_der_zeile(zeile) {
                // Sprach-Schlüssel selbst („kat-err-keine-version“) sind kein Text
                let ist_schluessel =
                    lit.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_');
                if !ist_schluessel && klingt_deutsch(&lit) && !FESTE_TEXTE_ERLAUBT.iter().any(|e| lit.contains(e)) {
                    let name = datei.strip_prefix(repo()).unwrap_or(&datei).display().to_string();
                    funde.push(format!("{name}:{nr}: {lit}"));
                }
            }
        }
    }
    assert!(funde.is_empty(), "Feste deutsche Texte ({}):\n{}", funde.len(), funde.join("\n"));
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
