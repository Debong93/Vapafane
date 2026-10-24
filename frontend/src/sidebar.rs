//! Seitenleiste im Server-Bereich, in Gruppen (Betrieb, Inhalte, Netzwerk).

use leptos::prelude::*;

use crate::pages::reach;
use crate::state::{Page, use_app};

#[component]
pub fn Sidebar() -> impl IntoView {
    let app = use_app();
    let item = move |page: Page, icon: &'static str, label: String| {
        view! {
            <button class:active=move || app.page.get() == page on:click=move |_| app.show(page)
                class:alarm=move || page == Page::Reach && reach::alarm(app)
                class:red=move || page == Page::Reach && reach::light(app) == "red">
                <span class=format!("px-ico ico-{icon}")></span>
                <span class="lbl">{label}</span>
                {(page == Page::Reach).then(|| view! { <span class="reach-dot" data-light=move || reach::light(app)></span> })}
            </button>
        }
    };
    view! {
        <aside class="side" aria-label=crate::sprache::t!("side-bereiche")>
            <div class="side-h">{crate::sprache::t!("side-betrieb")}</div>
            {item(Page::Overview, "overview", crate::sprache::t!("side-uebersicht"))}
            {item(Page::Console, "console", crate::sprache::t!("side-konsole"))}
            {item(Page::Players, "players", crate::sprache::t!("side-spieler"))}
            <div class="side-h">{crate::sprache::t!("side-inhalte")}</div>
            {item(Page::Mods, "mods", crate::sprache::t!("side-mods"))}
            {item(Page::Updates, "updates", crate::sprache::t!("side-updates"))}
            {item(Page::Backups, "backups", crate::sprache::t!("side-backups"))}
            <div class="side-h">{crate::sprache::t!("side-netzwerk")}</div>
            {item(Page::Reach, "reach", crate::sprache::t!("side-erreichbarkeit"))}
        </aside>
    }
}
