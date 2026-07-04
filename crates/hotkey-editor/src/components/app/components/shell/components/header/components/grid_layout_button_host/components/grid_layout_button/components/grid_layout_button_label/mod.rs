mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(GridLayoutButtonLabel);

/// The "GRID LAYOUT" caption inside the grid-layout button.
#[component]
pub fn GridLayoutButtonLabel() -> Element {
    rsx! {
        span { class: CLASS, "GRID LAYOUT" }
    }
}
