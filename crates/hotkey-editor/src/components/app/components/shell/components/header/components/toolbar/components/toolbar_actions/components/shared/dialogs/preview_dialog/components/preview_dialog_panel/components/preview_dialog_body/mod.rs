pub mod components;
mod props;
mod style;

use components::preview_textarea::{PreviewTextarea, PreviewTextareaProps};
use dioxus::prelude::*;
pub use props::PreviewDialogBodyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PreviewDialogBody);

/// The preview dialog's scrolling content region between the header and the panel
/// edge, holding the read-only serialized-keys textarea.
#[component]
pub fn PreviewDialogBody(props: PreviewDialogBodyProps) -> Element {
    let textarea = PreviewTextareaProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            PreviewTextarea { ..textarea }
        }
    }
}
