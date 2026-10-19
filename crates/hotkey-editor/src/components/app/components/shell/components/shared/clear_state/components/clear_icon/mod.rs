mod style;
use crate::components::app::components::shell::components::shared::icons::ICON_COLLISIONS_CLEAR;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(ClearIcon);
