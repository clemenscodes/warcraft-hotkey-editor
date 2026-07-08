mod data;
mod style;
use crate::components::app::components::shell::components::shared::empty_message::EmptyMessage;
use crate::components::app::components::shell::components::shared::page_state::PageState;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EmptyState);

/// The "upload a file" prompt shown before any CustomKeys.txt is loaded. A thin
/// identity wrapper that tags the resolve state for e2e and hands the shared
/// `PageState` shell its prompt message.
#[component]
pub fn EmptyState() -> Element {
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "no-file",
            PageState {
                EmptyMessage { text: data::UPLOAD_PROMPT }
            }
        }
    }
}
