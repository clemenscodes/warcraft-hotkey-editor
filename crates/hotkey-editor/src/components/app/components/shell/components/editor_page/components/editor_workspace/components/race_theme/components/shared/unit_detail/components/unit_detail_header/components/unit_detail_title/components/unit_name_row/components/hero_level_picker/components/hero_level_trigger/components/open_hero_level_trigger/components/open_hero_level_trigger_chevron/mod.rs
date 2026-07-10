mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The hero-level trigger's caret in its open, flipped look.
#[component]
pub fn OpenHeroLevelTriggerChevron() -> Element {
    rsx! {
        span {
            class: CLASS,
            "▾"
        }
    }
}

assert_component!(OpenHeroLevelTriggerChevron);
