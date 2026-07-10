pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::preview_dialog_panel::PreviewDialogPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_preview_dialog;
use logic::PreviewDialogShell;
use props::PreviewDialogProps;
use style::CLASS;
use tw_macro::assert_component;

/// Shows the serialized CustomKeys.txt in a read-only textarea. It owns its own
/// dialog shell: the hook shapes the text, the shell struct names the open flag and
/// panel, and this places the panel inside its own backdrop `div` (the dimmed,
/// centring layer) within the library `DialogRoot`. No project class touches the
/// library element.
#[component]
pub fn PreviewDialog(props: PreviewDialogProps) -> Element {
    use_body_scroll_lock(props.preview_open);
    let view = use_preview_dialog(&props);
    let PreviewDialogShell {
        open,
        on_open_change,
        title,
        on_close,
        text,
    } = PreviewDialogShell::from(&view);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                PreviewDialogPanel { title, on_close, text }
            }
        }
    }
}

assert_component!(PreviewDialog);
