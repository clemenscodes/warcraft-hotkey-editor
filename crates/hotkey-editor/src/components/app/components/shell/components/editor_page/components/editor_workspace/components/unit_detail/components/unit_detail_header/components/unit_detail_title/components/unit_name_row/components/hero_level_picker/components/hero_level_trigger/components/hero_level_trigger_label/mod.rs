mod data;
mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HeroLevelTriggerLabel);

/// The static "Level" caption at the left of the hero-level trigger.
#[component]
pub fn HeroLevelTriggerLabel() -> Element {
    rsx! {
        span {
            class: CLASS,
            {data::LABEL}
        }
    }
}
