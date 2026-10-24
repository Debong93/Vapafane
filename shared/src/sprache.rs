//! Sprachen: Übersetzungen (Fluent), Liste der Sprachen und Auflösung des Sprachwunschs.
//! Programm und Oberfläche nutzen dieselben Dateien aus `locales/`.

use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::FluentResource;
pub use fluent_bundle::{FluentArgs, FluentValue};
use unic_langid::LanguageIdentifier;

/// Alle Sprachen: (Code, Eigenname, maschinell übersetzt?). Deutsch und Englisch zuerst,
/// danach alphabetisch nach Code. Der Eigenname wird nie übersetzt.
pub const SPRACHEN: &[(&str, &str, bool)] = &[
    ("de", "Deutsch", false),
    ("en", "English", false),
];

/// Quelltexte der Sprachdateien (beim Bauen eingebettet, Reihenfolge wie `SPRACHEN`).
const QUELLEN: &[(&str, &str)] = &[
    ("de", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../locales/de.ftl"))),
    ("en", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../locales/en.ftl"))),
];

/// Gibt es diese Sprache? Liefert den Code aus der Liste (mit fester Lebensdauer).
fn bekannt(code: &str) -> Option<&'static str> {
    let code = code.trim().to_ascii_lowercase();
    SPRACHEN.iter().map(|(c, ..)| *c).find(|c| *c == code)
}

/// Sprachwunsch auflösen: `wunsch` ist ein Sprachcode oder None/„system“.
/// Dann entscheiden die Systemsprachen (z. B. „de-DE“ → de); sonst Englisch.
pub fn aufloesen(wunsch: Option<&str>, system: &[String]) -> &'static str {
    if let Some(code) = wunsch.filter(|w| !w.is_empty() && *w != "system").and_then(bekannt) {
        return code;
    }
    for locale in system {
        let kurz = locale.split(['-', '_']).next().unwrap_or(locale);
        if let Some(code) = bekannt(kurz) {
            return code;
        }
    }
    "en"
}

/// Ist diese Sprache maschinell übersetzt?
pub fn ist_ki(code: &str) -> bool {
    SPRACHEN.iter().any(|(c, _, ki)| *c == code && *ki)
}

fn lade_bundle(code: &str) -> Option<FluentBundle<FluentResource>> {
    let quelle = QUELLEN.iter().find(|(c, _)| *c == code)?.1;
    let id: LanguageIdentifier = code.parse().ok()?;
    let mut bundle = FluentBundle::new_concurrent(vec![id]);
    // Keine unsichtbaren Richtungszeichen um Platzhalter (stören Anzeige und Tests)
    bundle.set_use_isolating(false);
    // Bei Syntaxfehlern bleibt der lesbare Teil nutzbar; Vollständigkeit prüfen die Tests
    let ressource = match FluentResource::try_new(quelle.to_string()) {
        Ok(r) => r,
        Err((r, _fehler)) => r,
    };
    let _ = bundle.add_resource(ressource);
    Some(bundle)
}

/// Übersetzt Schlüssel in die gewählte Sprache; fehlt etwas, springt Englisch ein,
/// zuletzt der Schlüssel selbst (sollte durch die Tests nie passieren).
pub struct Uebersetzer {
    pub code: &'static str,
    bundle: Option<FluentBundle<FluentResource>>,
    englisch: Option<FluentBundle<FluentResource>>,
}

impl Uebersetzer {
    pub fn neu(code: &str) -> Uebersetzer {
        let code = bekannt(code).unwrap_or("en");
        Uebersetzer {
            code,
            bundle: lade_bundle(code),
            englisch: (code != "en").then(|| lade_bundle("en")).flatten(),
        }
    }

    pub fn ist_ki(&self) -> bool {
        ist_ki(self.code)
    }

    pub fn text(&self, schluessel: &str, args: Option<&FluentArgs>) -> String {
        for bundle in [self.bundle.as_ref(), self.englisch.as_ref()].into_iter().flatten() {
            if let Some(nachricht) = bundle.get_message(schluessel)
                && let Some(muster) = nachricht.value()
            {
                let mut fehler = vec![];
                return bundle.format_pattern(muster, args, &mut fehler).into_owned();
            }
        }
        schluessel.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn aufloesung() {
        assert_eq!(aufloesen(Some("de"), &[]), "de");
        assert_eq!(aufloesen(Some("system"), &["de-DE".into()]), "de");
        assert_eq!(aufloesen(None, &["fr_FR".into(), "de_DE".into()]), "de");
        assert_eq!(aufloesen(None, &["xx".into()]), "en");
        assert_eq!(aufloesen(Some("xx"), &[]), "en");
        assert_eq!(aufloesen(None, &[]), "en");
    }

    #[test]
    fn uebersetzen_und_notfall() {
        let de = Uebersetzer::neu("de");
        assert_eq!(de.text("sprache-titel", None), "Sprache");
        assert!(!de.ist_ki());
        let en = Uebersetzer::neu("en");
        assert_eq!(en.text("sprache-titel", None), "Language");
        // Unbekannter Schlüssel → Schlüssel selbst
        assert_eq!(de.text("gibt-es-nicht", None), "gibt-es-nicht");
        // Unbekannte Sprache → Englisch
        assert_eq!(Uebersetzer::neu("xx").code, "en");
    }

    fn schluessel_von(quelle: &str) -> BTreeSet<String> {
        use fluent_syntax::ast::Entry;
        let r = fluent_syntax::parser::parse(quelle).unwrap_or_else(|(r, _)| r);
        r.body
            .iter()
            .filter_map(|e| match e {
                Entry::Message(m) => Some(m.id.name.to_string()),
                _ => None,
            })
            .collect()
    }

    /// Jede Sprache hat genau die Schlüssel von Deutsch – nichts fehlt, nichts ist übrig.
    #[test]
    fn alle_sprachen_vollstaendig() {
        let de = schluessel_von(QUELLEN[0].1);
        assert!(!de.is_empty());
        for (code, quelle) in QUELLEN {
            let s = schluessel_von(quelle);
            let fehlt: Vec<_> = de.difference(&s).collect();
            let zuviel: Vec<_> = s.difference(&de).collect();
            assert!(fehlt.is_empty() && zuviel.is_empty(), "{code}: fehlt {fehlt:?}, übrig {zuviel:?}");
        }
    }

    /// Jede Sprachdatei ist fehlerfrei lesbar (Fluent-Syntax).
    #[test]
    fn alle_sprachen_fehlerfrei() {
        for (code, quelle) in QUELLEN {
            assert!(fluent_syntax::parser::parse(*quelle).is_ok(), "Syntaxfehler in {code}.ftl");
            assert!(SPRACHEN.iter().any(|(c, ..)| c == code), "{code} fehlt in SPRACHEN");
        }
        assert_eq!(QUELLEN.len(), SPRACHEN.len(), "SPRACHEN und QUELLEN passen nicht zusammen");
    }
}
