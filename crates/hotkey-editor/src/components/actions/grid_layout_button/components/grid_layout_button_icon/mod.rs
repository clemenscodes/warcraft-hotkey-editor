mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::shared::icons::ICON_GRID;
use style::CLASS;

assert_component!(GridLayoutButtonIcon);

/// The grid glyph inside the grid-layout button.
#[component]
pub fn GridLayoutButtonIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON_GRID,
        }
    }
}
