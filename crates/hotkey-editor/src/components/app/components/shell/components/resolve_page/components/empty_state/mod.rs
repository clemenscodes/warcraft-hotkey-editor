mod data;
mod style;
use crate::components::app::components::shell::components::shared::empty_message::EmptyMessage;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The "upload a file" prompt shown before any CustomKeys.txt is loaded. It centers
/// its prompt message in the available space.
#[component]
pub fn EmptyState() -> Element {
    rsx! {
        section {
            class: CLASS,
            EmptyMessage { text: data::UPLOAD_PROMPT }
        }
    }
}

assert_component!(EmptyState);
