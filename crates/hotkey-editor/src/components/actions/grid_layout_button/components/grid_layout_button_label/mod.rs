mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

assert_component!(GridLayoutButtonLabel);

/// The "GRID LAYOUT" caption inside the grid-layout button.
#[component]
pub fn GridLayoutButtonLabel() -> Element {
    rsx! {
        span { class: CLASS, "GRID LAYOUT" }
    }
}
