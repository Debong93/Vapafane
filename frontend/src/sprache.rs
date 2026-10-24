//! Sprache der Oberfläche: Übersetzer, der vor dem Aufbau der Seiten feststeht.
//! Der Wunsch („system“ oder ein Code) liegt in der Programm-Einstellung (Backend);
//! hier gespiegelt im localStorage, damit der Start nicht aufs Backend warten muss.

use std::cell::RefCell;

use serde_json::json;
use vapafane_shared::sprache::{FluentArgs, Uebersetzer, aufloesen};

use crate::state::{store, stored};

thread_local! {
    static UEB: RefCell<Uebersetzer> = RefCell::new(Uebersetzer::neu("en"));
}

/// Sprachen des Browsers/Systems (z. B. ["de-DE", "en"]).
fn systemsprachen() -> Vec<String> {
    web_sys::window()
        .map(|w| w.navigator().languages())
        .map(|l| l.iter().filter_map(|v| v.as_string()).collect())
        .unwrap_or_default()
}

/// Vor dem Aufbau der Oberfläche: Sprache aus Spiegel (localStorage) bzw. System wählen.
pub fn init() {
    let wunsch = stored("sprache-wunsch").unwrap_or_else(|| "system".into());
    let code = match stored("sprache-code") {
        // Backend hat schon einmal aufgelöst – das gilt
        Some(c) if wunsch == "system" => aufloesen(Some(&c), &systemsprachen()).to_string(),
        _ => aufloesen(Some(&wunsch), &systemsprachen()).to_string(),
    };
    let code = if wunsch == "system" && stored("sprache-code").is_none() {
        aufloesen(None, &systemsprachen()).to_string()
    } else {
        code
    };
    UEB.with(|u| *u.borrow_mut() = Uebersetzer::neu(&code));
}

/// Nach dem Start: mit der Programm-Einstellung abgleichen (Quelle der Wahrheit).
/// Weicht sie ab (z. B. erster Start oder anderswo geändert), einmal neu laden.
pub fn abgleichen() {
    leptos::task::spawn_local(async {
        use crate::util::J;
        let Ok(r) = crate::tauri::call::<serde_json::Value>("get_language").await else {
            return;
        };
        let (wish, resolved) = (r.s("wish"), r.s("resolved"));
        store("sprache-wunsch", Some(&wish));
        store("sprache-code", Some(&resolved));
        if !resolved.is_empty() && resolved != code() {
            neu_laden();
        }
    });
}

/// Sprache wechseln: Einstellung speichern, Spiegel setzen, Oberfläche neu laden.
pub fn wechseln(wunsch: String) {
    leptos::task::spawn_local(async move {
        match crate::tauri::invoke::<String>("set_language", json!({ "language": wunsch })).await {
            Ok(code) => {
                store("sprache-wunsch", Some(&wunsch));
                store("sprache-code", Some(&code));
                neu_laden();
            }
            Err(e) => crate::dialog::error(t!("sprache-titel"), e).await,
        }
    });
}

fn neu_laden() {
    if let Some(w) = web_sys::window() {
        let _ = w.location().reload();
    }
}

/// Aktiver Sprachcode (z. B. "de").
pub fn code() -> String {
    UEB.with(|u| u.borrow().code.to_string())
}

/// Ist die aktive Sprache maschinell übersetzt?
pub fn ist_ki() -> bool {
    UEB.with(|u| u.borrow().ist_ki())
}

pub fn text(schluessel: &str, args: Option<&FluentArgs>) -> String {
    UEB.with(|u| u.borrow().text(schluessel, args))
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
