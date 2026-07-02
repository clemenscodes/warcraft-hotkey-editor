mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(HeroLevelTriggerLabel);

/// The static "Level" caption at the left of the hero-level trigger.
#[component]
pub fn HeroLevelTriggerLabel() -> Element {
    rsx! {
        span {
            class: CLASS,
            "Level"
        }
    }
}
