mod style;
use crate::components::shared::icons::ICON_COLLISIONS_CLEAR;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolveClearIcon);
#[component]
pub fn ResolveClearIcon() -> Element {
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: ICON_COLLISIONS_CLEAR }
    }
}
