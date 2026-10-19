mod style;
use crate::components::app::components::shell::components::shared::icons::ICON_ARROW_RIGHT;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MoveArrow() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON_ARROW_RIGHT,
        }
    }
}

assert_component!(MoveArrow);
