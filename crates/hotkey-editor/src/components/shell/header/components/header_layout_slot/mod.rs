pub mod components;
mod style;

use crate::assert_component;
use components::grid_layout_button::GridLayoutButton;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(HeaderLayoutSlot);

/// The centered slot that holds the global grid-layout button in the full header
/// layout. Hidden in the compact layout, where the drawer offers it instead.
#[component]
pub fn HeaderLayoutSlot() -> Element {
    rsx! {
        div { class: CLASS, GridLayoutButton {} }
    }
}
