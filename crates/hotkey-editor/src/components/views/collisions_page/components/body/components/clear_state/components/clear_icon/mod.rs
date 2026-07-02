mod style;
use crate::assert_component;
use crate::components::shared::icons::ICON_COLLISIONS_CLEAR;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ClearIcon);

/// The gold "all clear" glyph shown when a collision kind has no conflicts.
#[component]
pub fn ClearIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON_COLLISIONS_CLEAR,
        }
    }
}
