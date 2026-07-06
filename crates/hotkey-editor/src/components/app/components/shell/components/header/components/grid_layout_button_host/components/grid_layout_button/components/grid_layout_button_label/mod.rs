mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(GridLayoutButtonLabel);

/// The "GRID LAYOUT" caption inside the grid-layout button.
#[component]
pub fn GridLayoutButtonLabel() -> Element {
    rsx! {
        span { class: CLASS, "GRID LAYOUT" }
    }
}
