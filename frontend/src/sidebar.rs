//! Seitenleiste im Server-Bereich, in Gruppen (Betrieb, Inhalte, Netzwerk).

use leptos::prelude::*;

use crate::state::{Page, use_app};

#[component]
pub fn Sidebar() -> impl IntoView {
    let app = use_app();
    let item = move |page: Page, icon: &'static str, label: &'static str| {
        view! {
            <button class:active=move || app.page.get() == page on:click=move |_| app.show(page)>
                <span class=format!("px-ico ico-{icon}")></span>
                <span class="lbl">{label}</span>
                {(page == Page::Reach).then(|| view! { <span class="reach-dot" data-light="none"></span> })}
            </button>
        }
    };
    view! {
        <aside class="side" aria-label="Bereiche">
            <div class="side-h">"Betrieb"</div>
            {item(Page::Overview, "overview", "Übersicht")}
            {item(Page::Console, "console", "Konsole")}
            {item(Page::Players, "players", "Spieler")}
            <div class="side-h">"Inhalte"</div>
            {item(Page::Mods, "mods", "Mods/Plugins")}
            {item(Page::Updates, "updates", "Version & Updates")}
            {item(Page::Backups, "backups", "Backups")}
            <div class="side-h">"Netzwerk"</div>
            {item(Page::Reach, "reach", "Erreichbarkeit")}
        </aside>
    }
}
