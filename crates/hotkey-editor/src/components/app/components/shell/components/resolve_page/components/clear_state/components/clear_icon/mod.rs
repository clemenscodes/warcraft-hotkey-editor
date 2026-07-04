mod style;
use crate::assert_component;
use crate::components::app::components::shell::components::shared::icons::ICON_COLLISIONS_CLEAR;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ClearIcon);
#[component]
pub fn ClearIcon() -> Element {
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: ICON_COLLISIONS_CLEAR }
    }
}
