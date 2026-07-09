mod style;

use super::super::data::EMPTY_PROMPT;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EmptyIslandDetail);

/// The empty island detail pane: the base pane surface, centered and muted, showing the
/// prompt to select a collision.
#[component]
pub fn EmptyIslandDetail() -> Element {
    rsx! {
        section {
            class: CLASS,
            p { {EMPTY_PROMPT} }
        }
    }
}
