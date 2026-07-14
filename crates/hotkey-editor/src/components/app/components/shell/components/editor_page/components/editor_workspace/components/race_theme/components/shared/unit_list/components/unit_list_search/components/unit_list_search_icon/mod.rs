mod style;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::shared::icons::ICON_SEARCH;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitListSearchIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            dangerous_inner_html: ICON_SEARCH,
        }
    }
}

assert_component!(UnitListSearchIcon);
