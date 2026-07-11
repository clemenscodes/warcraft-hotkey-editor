pub mod components;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialog;
use components::upload_button_input::UploadButtonInput;
use dioxus::prelude::*;
use presentation::{use_upload_button, UploadButtonModel};
use style::CLASS;
use tw_macro::assert_component;

/// The toolbar's import control: a hidden file input plus the visible upload button
/// that opens the "how to import" dialog. Sources the document service and the
/// upload status from context itself, so nothing is threaded in.
#[component]
pub fn UploadButton() -> Element {
    let UploadButtonModel {
        info_open: open,
        on_file_change: on_change,
        on_open_info: onclick,
        icon,
        aria_label,
    } = use_upload_button();
    rsx! {
        div {
            class: CLASS,
            UploadButtonInput { on_change }
            ToolbarButton {
                icon,
                aria_label,
                onclick,
            }
            UploadInfoDialog { open }
        }
    }
}

assert_component!(UploadButton);
