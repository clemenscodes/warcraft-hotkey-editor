pub mod components;
mod hooks;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use crate::assert_component;
use components::preview_textarea::PreviewTextarea;
use hooks::use_preview_dialog;

pub use props::PreviewDialogProps;

assert_component!(PreviewDialog);

/// Shows the serialized CustomKeys.txt in a read-only textarea. A component that
/// composes the `Dialog` base: the hook shapes the text, the textarea renders it.
#[component]
pub fn PreviewDialog(props: PreviewDialogProps) -> Element {
    let preview = use_preview_dialog(&props);
    rsx! {
        Dialog {
            open: props.preview_open,
            title: "Preview",
            PreviewTextarea { text: preview.text }
        }
    }
}
