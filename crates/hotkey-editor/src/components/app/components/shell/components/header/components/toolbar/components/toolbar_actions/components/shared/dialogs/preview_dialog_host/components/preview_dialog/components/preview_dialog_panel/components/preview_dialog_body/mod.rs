pub mod components;
mod props;
mod style;

use components::preview_textarea::PreviewTextarea;
use dioxus::prelude::*;
use props::PreviewDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The preview dialog's scrolling content region between the header and the panel
/// edge, holding the read-only serialized-keys textarea.
#[component]
pub fn PreviewDialogBody(props: PreviewDialogBodyProps) -> Element {
    let text = props.text;
    rsx! {
        div {
            class: CLASS,
            PreviewTextarea { text }
        }
    }
}

assert_component!(PreviewDialogBody);
