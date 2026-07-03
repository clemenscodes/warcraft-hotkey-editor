pub mod components;
mod hooks;

use components::collisions_button::CollisionsButton;
use dioxus::prelude::*;
use hooks::use_collisions_button;

/// Connected wrapper: reads the live document and grid layout, asks the domain to
/// count collisions, and hands the presentational button that summary plus the
/// route-to-collisions handler. Owns no markup beyond the leaf it wraps.
#[component]
pub fn CollisionsButtonHost() -> Element {
    let button = use_collisions_button();
    rsx! {
        CollisionsButton { ..button }
    }
}
