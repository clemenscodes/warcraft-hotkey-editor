pub mod components;
mod hooks;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_preview_dialog;
pub use props::PreviewDialogProps;

/// Shows the serialized CustomKeys.txt in a read-only textarea. A component that
/// composes the `Dialog` base: the hook shapes the text, the textarea renders it.
#[component]
pub fn PreviewDialog(props: PreviewDialogProps) -> Element {
    let view = use_preview_dialog(&props);
    let preview_open = props.preview_open;
    if !preview_open() {
        return rsx! {};
    }
    rsx! {
        Dialog { ..DialogProps::from(&view) }
    }
}
