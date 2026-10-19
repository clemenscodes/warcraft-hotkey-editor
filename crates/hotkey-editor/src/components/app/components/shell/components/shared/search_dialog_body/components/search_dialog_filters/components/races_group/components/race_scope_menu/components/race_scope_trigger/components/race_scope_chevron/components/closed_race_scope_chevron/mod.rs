mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_CHEVRON_RIGHT;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ClosedRaceScopeChevron() -> Element {
    rsx! {
        span {
            class: CLASS,
            dangerous_inner_html: ICON_CHEVRON_RIGHT,
        }
    }
}

assert_component!(ClosedRaceScopeChevron);
