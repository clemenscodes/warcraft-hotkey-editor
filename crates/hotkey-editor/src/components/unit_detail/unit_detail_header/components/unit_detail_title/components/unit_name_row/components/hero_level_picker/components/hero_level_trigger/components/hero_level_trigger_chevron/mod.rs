mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(HeroLevelTriggerChevron);

/// The trigger's caret; it flips when the parent trigger is open (via `group`).
#[component]
pub fn HeroLevelTriggerChevron() -> Element {
    rsx! {
        span {
            class: CLASS,
            "▾"
        }
    }
}
