//! Brücke zu Tauri: Befehle im Rust-Backend aufrufen und auf Ereignisse hören.
//! Die eigentliche Verbindung stellt Tauri bereit (`window.__TAURI__`, withGlobalTauri).

use serde::Serialize;
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn tauri_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen, catch)]
    async fn tauri_listen(
        event: &str,
        handler: &Closure<dyn FnMut(JsValue)>,
    ) -> Result<JsValue, JsValue>;
}

/// Fehlertext so, wie das Backend ihn schickt (bereits deutsch und verständlich).
fn text_of(e: JsValue) -> String {
    e.as_string().unwrap_or_else(|| format!("{e:?}"))
}

/// Tauri-Befehl aufrufen. `args` wird wie JSON übergeben (Feldnamen in camelCase).
pub async fn invoke<T: DeserializeOwned>(cmd: &str, args: impl Serialize) -> Result<T, String> {
    let args = args
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| e.to_string())?;
    let value = tauri_invoke(cmd, args).await.map_err(text_of)?;
    serde_wasm_bindgen::from_value(value).map_err(|e| format!("Antwort von „{cmd}“ unlesbar: {e}"))
}

/// Befehl ohne Argumente.
pub async fn call<T: DeserializeOwned>(cmd: &str) -> Result<T, String> {
    invoke(cmd, serde_json::json!({})).await
}

/// Auf ein Ereignis des Backends hören; `handler` bekommt die Nutzlast.
#[allow(dead_code)] // ab Schritt 2 (Konsole, Fortschritt)
pub fn listen<T: DeserializeOwned + 'static>(
    event: &'static str,
    mut handler: impl FnMut(T) + 'static,
) {
    let cb = Closure::<dyn FnMut(JsValue)>::new(move |ev: JsValue| {
        let payload =
            js_sys::Reflect::get(&ev, &JsValue::from_str("payload")).unwrap_or(JsValue::NULL);
        if let Ok(v) = serde_wasm_bindgen::from_value(payload) {
            handler(v);
        }
    });
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tauri_listen(event, &cb).await;
        cb.forget(); // lebt so lange wie das Fenster
    });
}
