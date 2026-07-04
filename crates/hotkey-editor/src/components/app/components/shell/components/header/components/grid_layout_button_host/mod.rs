pub mod components;
mod hooks;
mod style;

use crate::assert_component;
use components::grid_layout_button::GridLayoutButton;
use dioxus::prelude::*;
use hooks::use_grid_layout_button;
use style::CLASS;
assert_component!(GridLayoutButtonHost);

/// Connects the global grid-layout button to the overlay state and gives it its
/// centered, laptop-and-up box. Hidden in the compact layout, where the drawer
/// offers the action instead.
#[component]
pub fn GridLayoutButtonHost() -> Element {
    let button = use_grid_layout_button();
    rsx! {
        div {
            class: CLASS,
            GridLayoutButton { ..button }
        }
    }
}
