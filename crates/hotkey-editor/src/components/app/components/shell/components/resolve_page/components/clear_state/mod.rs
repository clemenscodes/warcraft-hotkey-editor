mod style;
use crate::components::app::components::shell::components::shared::clear_icon::ClearIcon;
use crate::components::app::components::shell::components::shared::clear_label::ClearLabel;
use crate::components::app::components::shell::components::shared::page_state::PageState;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearState);

/// The "nothing to resolve" state shown when a file is loaded but conflict-free. A
/// thin identity wrapper that tags the resolve state for e2e and hands the shared
/// `PageState` shell its glyph and label.
#[component]
pub fn ClearState() -> Element {
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "clear",
            PageState {
                ClearIcon {}
                ClearLabel { text: "Nothing to resolve." }
            }
        }
    }
}
