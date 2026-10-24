//! Kleine Helfer: Formatierung (deutsch), Uhrzeit, JSON lesen, Links/Ordner öffnen, Animationen.

use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::tauri;

/// Größe lesbar: 512 B, 87 KB, 1,9 GB …
pub fn fmt_bytes(n: Option<f64>) -> String {
    let Some(mut n) = n else { return "–".into() };
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut i = 0;
    while n >= 1024.0 && i < units.len() - 1 {
        n /= 1024.0;
        i += 1;
    }
    if i >= 3 {
        format!("{} {}", num(n, 1), units[i])
    } else {
        format!("{} {}", n.round(), units[i])
    }
}

/// Dauer: „2 Std 14 Min“, „3 Min 5 Sek“ … (Einheiten aus der Sprachdatei)
pub fn fmt_duration(sec: f64) -> String {
    use crate::sprache::t;
    let sec = sec.max(0.0).floor() as u64;
    let (d, h, m, s) = (
        sec / 86400,
        (sec % 86400) / 3600,
        (sec % 3600) / 60,
        sec % 60,
    );
    if d > 0 {
        format!("{d} {} {h} {}", t!("einheit-tag"), t!("einheit-std"))
    } else if h > 0 {
        format!("{h} {} {m} {}", t!("einheit-std"), t!("einheit-min"))
    } else if m > 0 {
        format!("{m} {} {s} {}", t!("einheit-min"), t!("einheit-sek"))
    } else {
        format!("{s} {}", t!("einheit-sek"))
    }
}

/// Zahl mit Dezimaltrennzeichen der Sprache: 19,8 bzw. 19.8
pub fn num(x: f64, digits: usize) -> String {
    format!("{x:.digits$}").replace('.', &crate::sprache::t!("zahl-dezimal"))
}

/// Große Zahlen kurz: 1,2 Mio. · 34 Tsd. (Einheiten aus der Sprachdatei)
pub fn fmt_count(n: f64) -> String {
    use crate::sprache::t;
    if n >= 1e6 {
        format!("{} {}", num(n / 1e6, 1), t!("einheit-mio"))
    } else if n >= 1e3 {
        format!("{} {}", (n / 1e3).round(), t!("einheit-tsd"))
    } else {
        format!("{n}")
    }
}

/// Tausendertrennzeichen der Sprache: 12.345 bzw. 12,345
pub fn thousands(n: u64) -> String {
    let sep = crate::sprache::t!("zahl-tausender");
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push_str(&sep);
        }
        out.push(c);
    }
    out
}

/// Jetzt als „HH:MM“.
pub fn clock() -> String {
    let d = js_sys::Date::new_0();
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}

/// Unix-Sekunden als „HH:MM“ (lokale Zeit).
pub fn clock_of(secs: f64) -> String {
    let d = js_sys::Date::new(&JsValue::from_f64(secs * 1000.0));
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}

/// Letzter Teil eines Pfads (Dateiname).
pub fn file_name(path: &str) -> String {
    path.rsplit(['/', '\\']).next().unwrap_or(path).to_string()
}

// ---------------------------------------------------------------- JSON bequem lesen

pub trait J {
    fn s(&self, k: &str) -> String;
    fn f(&self, k: &str) -> Option<f64>;
    fn b(&self, k: &str) -> bool;
    fn a(&self, k: &str) -> Vec<Value>;
    fn o(&self, k: &str) -> Value;
}

impl J for Value {
    /// Text (Zahlen werden zu Text, fehlend = leer)
    fn s(&self, k: &str) -> String {
        match self.pointer(&ptr(k)) {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Null) | None => String::new(),
            Some(v) => v.to_string(),
        }
    }
    fn f(&self, k: &str) -> Option<f64> {
        self.pointer(&ptr(k)).and_then(|v| v.as_f64())
    }
    fn b(&self, k: &str) -> bool {
        self.pointer(&ptr(k))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
    fn a(&self, k: &str) -> Vec<Value> {
        self.pointer(&ptr(k))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
    }
    fn o(&self, k: &str) -> Value {
        self.pointer(&ptr(k)).cloned().unwrap_or(Value::Null)
    }
}

/// „a/b“ → JSON-Pointer „/a/b“
fn ptr(k: &str) -> String {
    format!("/{k}")
}

// ---------------------------------------------------------------- Backend-Kurzbefehle

/// Link öffnen (nur Adressen, die das Backend erlaubt).
pub fn open_link(target: impl Into<String>) {
    let target = target.into();
    leptos::task::spawn_local(async move {
        if let Err(e) =
            tauri::invoke::<()>("open_link", serde_json::json!({ "target": target })).await
        {
            crate::dialog::error("Link", e).await;
        }
    });
}

/// Ordner des aktiven Servers öffnen: server, mods, datapacks, backups, logs.
pub fn open_folder(which: &'static str) {
    leptos::task::spawn_local(async move {
        if let Err(e) =
            tauri::invoke::<()>("open_folder", serde_json::json!({ "which": which })).await
        {
            crate::dialog::error("Ordner", e).await;
        }
    });
}

/// Systemmeldung (stumm; nur wenn in den Einstellungen eingeschaltet).
pub fn notify(title: &str, body: &str) {
    let (title, body) = (title.to_string(), body.to_string());
    leptos::task::spawn_local(async move {
        let _ = tauri::invoke::<()>(
            "notify",
            serde_json::json!({ "title": title, "body": body }),
        )
        .await;
    });
}

/// Text in die Zwischenablage.
pub fn copy_text(text: &str) {
    let Some(w) = web_sys::window() else { return };
    let clip =
        js_sys::Reflect::get(&w.navigator(), &"clipboard".into()).unwrap_or(JsValue::UNDEFINED);
    if let Ok(f) = js_sys::Reflect::get(&clip, &"writeText".into())
        .and_then(|f| f.dyn_into::<js_sys::Function>())
    {
        let _ = f.call1(&clip, &JsValue::from_str(text));
    }
}

// ---------------------------------------------------------------- Animationen (Web Animations API)

/// `el.animate(keyframes, options)` – Schlüsselbilder und Optionen als JSON.
/// Liefert das Animation-Objekt (für `onfinish`).
pub fn animate(el: &web_sys::Element, keyframes: Value, options: Value) -> Option<JsValue> {
    use serde::Serialize;
    let ser = serde_wasm_bindgen::Serializer::json_compatible();
    let kf = keyframes.serialize(&ser).ok()?;
    let op = options.serialize(&ser).ok()?;
    let f = js_sys::Reflect::get(el, &"animate".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    f.call2(el, &kf, &op).ok()
}

/// Wert einer CSS-Variable auf <html>.
pub fn css_var(name: &str) -> String {
    let Some(w) = web_sys::window() else {
        return String::new();
    };
    let Some(root) = w.document().and_then(|d| d.document_element()) else {
        return String::new();
    };
    w.get_computed_style(&root)
        .ok()
        .flatten()
        .and_then(|s| s.get_property_value(name).ok())
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Berechneter Stil eines Elements (z. B. „background-color“).
pub fn computed(el: &web_sys::Element, prop: &str) -> String {
    web_sys::window()
        .and_then(|w| w.get_computed_style(el).ok().flatten())
        .and_then(|s| s.get_property_value(prop).ok())
        .unwrap_or_default()
}

pub fn by_id(id: &str) -> Option<web_sys::HtmlElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id(id)?
        .dyn_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatierung() {
        // Ohne init() steht der Übersetzer auf Englisch – also englische Einheiten
        assert_eq!(fmt_bytes(Some(512.0)), "512 B");
        assert_eq!(fmt_bytes(Some(1.9 * 1024.0 * 1024.0 * 1024.0)), "1.9 GB");
        assert_eq!(fmt_bytes(None), "–");
        assert_eq!(fmt_duration(7300.0), "2 h 1 min");
        assert_eq!(fmt_duration(65.0), "1 min 5 s");
        assert_eq!(thousands(1234567), "1,234,567");
        assert_eq!(fmt_count(2_500_000.0), "2.5 M");
        assert_eq!(file_name("/a/b/c.zip"), "c.zip");
    }
}
