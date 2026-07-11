pub mod components;
mod model;
mod view;

pub use view::PreviewDialogBodyView;
mod style;

use components::preview_textarea::PreviewTextarea;
use dioxus::prelude::*;
use model::PreviewDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The preview dialog's scrolling content region between the header and the panel
/// edge, holding the read-only serialized-keys textarea.
#[component]
pub fn PreviewDialogBody(props: PreviewDialogBodyModel) -> Element {
    let text = props.text;
    rsx! {
        div {
            class: CLASS,
            PreviewTextarea { text }
        }
    }
}

assert_component!(PreviewDialogBody);
