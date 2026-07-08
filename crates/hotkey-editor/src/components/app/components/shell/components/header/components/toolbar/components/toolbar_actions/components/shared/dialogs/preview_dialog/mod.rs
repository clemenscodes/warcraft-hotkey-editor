pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::preview_dialog_body::PreviewDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use hooks::use_preview_dialog;
use logic::PreviewDialogShell;
pub use props::PreviewDialogProps;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(PreviewDialog);

/// Shows the serialized CustomKeys.txt in a read-only textarea. It owns its own
/// dialog shell: the hook shapes the text, the shell struct names the header and
/// scroll body, and this places them inside the backdrop and bordered box.
#[component]
pub fn PreviewDialog(props: PreviewDialogProps) -> Element {
    use_body_scroll_lock(props.preview_open);
    let view = use_preview_dialog(&props);
    let PreviewDialogShell {
        open,
        on_open_change,
        header,
        body,
    } = PreviewDialogShell::from(&view);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                PreviewDialogBody { ..body }
            }
        }
    }
}
