mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::actions::grid_layout_button::GridLayoutButton;
use style::CLASS;

assert_component!(HeaderLayoutSlot);

/// The centered slot that holds the global grid-layout button in the full header
/// layout. Hidden in the compact layout, where the drawer offers it instead.
#[component]
pub fn HeaderLayoutSlot() -> Element {
    rsx! {
        div {
            class: CLASS,
            GridLayoutButton {}
        }
    }
}
