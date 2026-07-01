pub mod components;
mod style;
use crate::assert_component;
use components::resolve_empty_message::ResolveEmptyMessage;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolveEmptyState);

/// The "upload a file" prompt shown before any CustomKeys.txt is loaded.
#[component]
pub fn ResolveEmptyState() -> Element {
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "no-file",
            ResolveEmptyMessage { text: "Upload your CustomKeys.txt to preview the cascade plan." }
        }
    }
}
