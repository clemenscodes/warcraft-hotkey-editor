mod data;
mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HeroLevelTriggerLabel() -> Element {
    rsx! {
        span {
            class: CLASS,
            {data::LABEL}
        }
    }
}

assert_component!(HeroLevelTriggerLabel);
