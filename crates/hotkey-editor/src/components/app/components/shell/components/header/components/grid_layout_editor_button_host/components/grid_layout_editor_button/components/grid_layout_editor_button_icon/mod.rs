mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_GRID;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridLayoutEditorButtonIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON_GRID,
        }
    }
}

assert_component!(GridLayoutEditorButtonIcon);
