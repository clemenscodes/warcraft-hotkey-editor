pub mod components;
mod data;
mod style;
use components::empty_message::EmptyMessage;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
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
