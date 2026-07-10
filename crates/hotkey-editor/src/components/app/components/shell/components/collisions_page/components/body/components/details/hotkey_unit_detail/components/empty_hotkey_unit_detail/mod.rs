mod style;

use super::super::data::EMPTY_PROMPT;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The empty shared-hotkey detail pane: the base pane surface, centered and muted,
/// showing the prompt to select a unit.
#[component]
pub fn EmptyHotkeyUnitDetail() -> Element {
    rsx! {
        section {
            class: CLASS,
            p { {EMPTY_PROMPT} }
        }
    }
}

assert_component!(EmptyHotkeyUnitDetail);
