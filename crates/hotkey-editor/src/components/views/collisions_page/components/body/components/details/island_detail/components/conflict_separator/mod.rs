mod data;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ConflictSeparator);

/// The centered ✕ between the two clashing abilities on a conflict card.
#[component]
pub fn ConflictSeparator() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            {data::SEPARATOR}
        }
    }
}
