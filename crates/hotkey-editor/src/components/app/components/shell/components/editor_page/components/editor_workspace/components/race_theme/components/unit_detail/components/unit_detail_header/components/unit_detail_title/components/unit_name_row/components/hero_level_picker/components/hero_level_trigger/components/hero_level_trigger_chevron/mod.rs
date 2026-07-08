mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
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
