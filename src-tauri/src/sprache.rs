//! Sprache des Programms: globaler Übersetzer (Fenster, Dienst und Befehlszeile).

use std::sync::RwLock;

use vapafane_shared::sprache::{FluentArgs, Uebersetzer, aufloesen};

static UEB: RwLock<Option<Uebersetzer>> = RwLock::new(None);

/// Sprachen des Systems (z. B. ["de-DE", "en-US"]).
pub fn systemsprachen() -> Vec<String> {
    // Im Testmodus erzwingbar, damit Tests überall gleich laufen
    if crate::config::test_home().is_some()
        && let Ok(l) = std::env::var("VAPAFANE_LANG")
        && !l.is_empty()
    {
        return vec![l];
    }
    sys_locale::get_locales().collect()
}

/// Beim Programmstart und nach jedem Sprachwechsel: Übersetzer passend zur Einstellung laden.
pub fn init() {
    let wunsch = crate::config::Config::load().language;
    let code = aufloesen(wunsch.as_deref(), &systemsprachen());
    *crate::util::lock_rw(&UEB) = Some(Uebersetzer::neu(code));
}

/// Aufgelöster Sprachcode (z. B. "de").
pub fn code() -> &'static str {
    crate::util::read_rw(&UEB).as_ref().map(|u| u.code).unwrap_or("en")
}

pub fn text(schluessel: &str, args: Option<&FluentArgs>) -> String {
    if let Some(u) = crate::util::read_rw(&UEB).as_ref() {
        return u.text(schluessel, args);
    }
    // init() noch nicht gelaufen (sollte nicht vorkommen): einmalig englisch
    Uebersetzer::neu("en").text(schluessel, args)
}

/// Übersetzen: `t!("schluessel")` oder `t!("schluessel", name = wert, zahl = 3)`.
macro_rules! t {
    ($k:literal) => { $crate::sprache::text($k, None) };
    ($k:literal, $($n:ident = $v:expr),+ $(,)?) => {{
        let mut args = ::vapafane_shared::sprache::FluentArgs::new();
        $( args.set(stringify!($n), ::vapafane_shared::sprache::FluentValue::from($v)); )+
        $crate::sprache::text($k, Some(&args))
    }};
}
pub(crate) use t;
