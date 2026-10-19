mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_CHEVRON_DOWN;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn OpenHeroLevelTriggerChevron() -> Element {
    rsx! {
        span {
            class: CLASS,
            dangerous_inner_html: ICON_CHEVRON_DOWN,
        }
    }
}

assert_component!(OpenHeroLevelTriggerChevron);
