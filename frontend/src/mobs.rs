//! Figuren auf dem Gras: tagsüber Creeper, nachts Enderman (bei Crying Obsidian aktiver).
//! Kopf bzw. Augen schauen in Pixelschritten zur Maus; der Enderman teleportiert sich ab und zu.

use leptos::ev;
use leptos::html::Div;
use leptos::prelude::*;
use std::time::Duration;

use crate::state::{document_visible, is_dark, is_obsidian, reduced_motion};

/// Blickrichtung (-1, 0, 1) für eine Figur relativ zur Maus.
fn look(el: &web_sys::HtmlElement, x: f64, y: f64) -> (i32, i32) {
    let r = el.get_bounding_client_rect();
    if r.width() == 0.0 {
        return (0, 0);
    }
    let step = |d: f64| if d.abs() < 30.0 { 0 } else { d.signum() as i32 };
    (
        step(x - (r.left() + r.width() / 2.0)),
        step(y - (r.top() + 8.0)),
    )
}

fn teleport(right: RwSignal<u32>, gone: RwSignal<bool>) {
    let obsidian = is_obsidian();
    if !reduced_motion() && document_visible() && is_dark() {
        gone.set(true);
        set_timeout(
            move || {
                let span = if obsidian { 260.0 } else { 140.0 };
                right.set(110 + (js_sys::Math::random() * span) as u32);
                gone.set(false);
            },
            Duration::from_millis(450),
        );
    }
    let wait = if obsidian { 7000.0 } else { 25000.0 }
        + js_sys::Math::random() * if obsidian { 7000.0 } else { 20000.0 };
    set_timeout(
        move || teleport(right, gone),
        Duration::from_millis(wait as u64),
    );
}

#[component]
pub fn Mobs() -> impl IntoView {
    let creeper = NodeRef::<Div>::new();
    let ender = NodeRef::<Div>::new();
    let creeper_look = RwSignal::new((0, 0));
    let ender_look = RwSignal::new((0, 0));
    let right = RwSignal::new(120u32);
    let gone = RwSignal::new(false);

    let _ = window_event_listener(ev::mousemove, move |e| {
        let (x, y) = (e.client_x() as f64, e.client_y() as f64);
        if let Some(el) = creeper.get_untracked() {
            creeper_look.set(look(&el, x, y));
        }
        if let Some(el) = ender.get_untracked() {
            ender_look.set(look(&el, x, y));
        }
    });
    let _ = window_event_listener(ev::mouseout, move |e| {
        if e.related_target().is_none() {
            creeper_look.set((0, 0));
            ender_look.set((0, 0));
        }
    });
    set_timeout(move || teleport(right, gone), Duration::from_secs(8));

    let style = |l: RwSignal<(i32, i32)>| move || format!("--lx:{};--ly:{}", l.get().0, l.get().1);
    view! {
        <div class="mobs" id="mobs" aria-hidden="true" class:weg=move || gone.get()
            style=move || format!("right:{}px", right.get())>
            <div class="mob creeper" node_ref=creeper style=style(creeper_look)>
                <i class="head"></i><i class="body"></i>
            </div>
            <div class="mob ender" node_ref=ender style=style(ender_look)>
                <i class="head"><i class="eyes"></i></i><i class="body"></i>
            </div>
        </div>
    }
}
