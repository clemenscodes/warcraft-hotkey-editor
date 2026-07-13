mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The "GRID LAYOUT" caption inside the grid-layout button.
#[component]
pub fn GridLayoutEditorButtonLabel() -> Element {
    rsx! {
        span { class: CLASS, "GRID LAYOUT" }
    }
}

assert_component!(GridLayoutEditorButtonLabel);
