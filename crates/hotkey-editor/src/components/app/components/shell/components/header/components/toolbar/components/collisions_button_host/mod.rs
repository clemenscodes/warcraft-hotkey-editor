pub mod components;
mod hooks;
mod style;

use components::collisions_button::CollisionsButton;
use dioxus::prelude::*;
use hooks::use_collisions_button;
use style::CLASS;
use tw_macro::assert_component;

/// Connected wrapper and container: reads the live document and grid layout, asks the
/// domain to count collisions, and hands the presentational button that summary plus
/// the route-to-collisions handler. Its classed root owns the button's box per band and
/// is the container-query context the button's `cqi` chrome scales against — so the
/// button's own border and radius resolve against its real size, not the viewport.
#[component]
pub fn CollisionsButtonHost() -> Element {
    let button = use_collisions_button();
    rsx! {
        div {
            class: CLASS,
            CollisionsButton { ..button }
        }
    }
}

assert_component!(CollisionsButtonHost);
