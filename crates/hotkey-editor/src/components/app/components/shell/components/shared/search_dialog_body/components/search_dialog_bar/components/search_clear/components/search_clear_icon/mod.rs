mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_CLOSE;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchClearIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON_CLOSE,
        }
    }
}

assert_component!(SearchClearIcon);
