pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::PreviewTextareaHostView;

use components::preview_textarea::PreviewTextarea;
use dioxus::prelude::*;
use presentation::{PreviewTextareaHostPresentation, use_preview_textarea_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connected, isolated preview content: serializes the loaded CustomKeys.txt document and
/// feeds the read-only `PreviewTextarea` its text. Zero dialog chrome — it is
/// page-renderable on its own, and a dialog places it as a body region. The serialize is
/// the one piece of work, in its presentation builder, never here. Its root is the
/// scrolling box that fills whatever space it is given.
#[component]
pub fn PreviewTextareaHost() -> Element {
    let PreviewTextareaHostPresentation { text } = use_preview_textarea_host();
    rsx! {
        div {
            class: CLASS,
            PreviewTextarea {
                text,
            }
        }
    }
}

assert_component!(PreviewTextareaHost);
