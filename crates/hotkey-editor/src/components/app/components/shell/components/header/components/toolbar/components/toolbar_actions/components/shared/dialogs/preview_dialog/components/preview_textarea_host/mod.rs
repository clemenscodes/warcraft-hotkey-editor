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
