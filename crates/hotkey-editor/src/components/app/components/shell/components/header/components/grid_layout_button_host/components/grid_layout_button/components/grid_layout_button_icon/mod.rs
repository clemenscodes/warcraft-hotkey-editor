mod style;

use crate::assert_component;
use crate::components::app::components::shell::components::shared::icons::ICON_GRID;
use dioxus::prelude::*;
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
