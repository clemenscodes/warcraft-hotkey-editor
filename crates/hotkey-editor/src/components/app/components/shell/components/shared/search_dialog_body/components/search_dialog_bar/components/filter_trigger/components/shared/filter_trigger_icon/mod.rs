mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_FILTER;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilterTriggerIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            dangerous_inner_html: ICON_FILTER,
        }
    }
}

assert_component!(FilterTriggerIcon);
