mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The hero-level trigger's caret in its resting, upright look.
#[component]
pub fn ClosedHeroLevelTriggerChevron() -> Element {
    rsx! {
        span {
            class: CLASS,
            "▾"
        }
    }
}

assert_component!(ClosedHeroLevelTriggerChevron);
