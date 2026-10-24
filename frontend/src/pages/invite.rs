//! „Freunde einladen“: Einladungs-Code oder -Datei erzeugen.

use leptos::prelude::*;
use serde_json::{Value, json};
use std::rc::Rc;

use crate::dialog::{self, Btn, error};
use crate::tauri;
use crate::util::{J, copy_text};

pub fn open_invite() {
    leptos::task::spawn_local(async move {
        let d = match tauri::call::<Value>("invite_defaults").await {
            Ok(d) => d,
            Err(e) => return error(crate::sprache::t!("inv-titel"), e).await,
        };
        let addr = RwSignal::new(d.s("lan"));
        let out = RwSignal::new(None::<Result<(Value, bool), String>>);
        let busy = RwSignal::new(false);
        let copied = RwSignal::new(false);
        let public = move |_| {
            busy.set(true);
            leptos::task::spawn_local(async move {
                match tauri::call::<String>("invite_public_address").await {
                    Ok(a) => addr.set(a),
                    Err(e) => out.set(Some(Err(e))),
                }
                busy.set(false);
            });
        };
        let make = move |file: bool| {
            copied.set(false);
            let a = addr.get_untracked();
            leptos::task::spawn_local(async move {
                let cmd = if file { "invite_file" } else { "invite_code" };
                match tauri::invoke::<Option<Value>>(cmd, json!({ "address": a })).await {
                    Ok(Some(r)) => out.set(Some(Ok((r, file)))),
                    Ok(None) => {}
                    Err(e) => out.set(Some(Err(e))),
                }
            });
        };
        let result = move || {
            out.get().map(|r| match r {
            Err(e) => view! { <p class="hint">{e}</p> }.into_any(),
            Ok((r, file)) => {
                let code = r.s("code");
                let files: Vec<String> = r.a("files").iter().filter_map(|f| f.as_str().map(String::from)).collect();
                let server_only: Vec<String> = r.a("server_only").iter().filter_map(|f| f.as_str().map(String::from)).collect();
                let mods = r.f("mods").unwrap_or(0.0);
                let a6 = r.s("address6");
                view! {
                    {if file {
                        view! { <p>"✓ "{crate::sprache::t!("inv-datei-gespeichert")}" "<code>{r.s("path")}</code>" – "{crate::sprache::t!("inv-datei-schicken")}</p> }.into_any()
                    } else {
                        let c2 = code.clone();
                        view! {
                            <label>{crate::sprache::t!("inv-code-label", zeichen = code.chars().count() as u32)}</label>
                            <textarea id="inv-code" rows="4" readonly>{code.clone()}</textarea>
                            <div class="row"><button class="primary" type="button" on:click=move |_| { copy_text(&c2); copied.set(true); }>
                                {move || if copied.get() { format!("✓ {}", crate::sprache::t!("inv-kopiert")) } else { crate::sprache::t!("inv-kopieren") }}</button></div>
                        }.into_any()
                    }}
                    <p>{if mods > 0.0 {
                        format!("{}{}", crate::sprache::t!("inv-mods", anzahl = mods), r.f("optional").filter(|o| *o > 0.0).map(|o| format!(" {}", crate::sprache::t!("inv-mods-optional", anzahl = o))).unwrap_or_default())
                    } else { crate::sprache::t!("inv-keine-mods", version = r.s("mc")) }}</p>
                    {(!a6.is_empty()).then(|| view! { <p class="dim small">{format!("{}{}", crate::sprache::t!("inv-ipv6", adresse = a6),
                        if r.b("only_v6") { format!(" {}", crate::sprache::t!("inv-nur-ipv6")) } else { String::new() })}</p> })}
                    {(!server_only.is_empty()).then(|| view! { <p class="dim small">{crate::sprache::t!("inv-nur-server", mods = server_only.join(", "))}</p> })}
                    {(!files.is_empty() && !file).then(|| view! { <p class="hint">{crate::sprache::t!("inv-datei-noetig", mods = files.join(", "))}</p> })}
                }.into_any()
            }
        })
        };
        let body: dialog::BodyFn = Rc::new(move || {
            view! {
            <p>{crate::sprache::t!("inv-erklaerung")}</p>
            <label>{crate::sprache::t!("inv-adresse-label")}</label>
            <div class="row" style="margin:4px 0">
                <input spellcheck="false" prop:value=addr on:input=move |e| addr.set(event_target_value(&e))/>
                <button type="button" disabled=move || busy.get() on:click=public>{crate::sprache::t!("inv-oeffentlich")}</button>
            </div>
            <p class="dim small">{crate::sprache::t!("inv-adresse-hinweis")}</p>
            <div class="row">
                <button class="primary" type="button" on:click=move |_| make(false)>{crate::sprache::t!("inv-code-erzeugen")}</button>
                <button type="button" on:click=move |_| make(true)>{crate::sprache::t!("inv-als-datei")}</button>
            </div>
            <div>{result}</div>
            <p class="dim small">{crate::sprache::t!("inv-kein-passwort")}</p>
        }.into_any()
        });
        dialog::ask_view(
            crate::sprache::t!("inv-titel"),
            body,
            vec![Btn::primary(crate::sprache::t!("gem-schliessen"), "")],
            true,
        )
        .await;
    });
}
