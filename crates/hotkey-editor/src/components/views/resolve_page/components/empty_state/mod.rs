pub mod components;
mod data;
mod style;
use crate::assert_component;
use components::empty_message::EmptyMessage;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(EmptyState);

/// The "upload a file" prompt shown before any CustomKeys.txt is loaded.
#[component]
pub fn EmptyState() -> Element {
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "no-file",
            EmptyMessage { text: data::UPLOAD_PROMPT }
        }
    }
}
