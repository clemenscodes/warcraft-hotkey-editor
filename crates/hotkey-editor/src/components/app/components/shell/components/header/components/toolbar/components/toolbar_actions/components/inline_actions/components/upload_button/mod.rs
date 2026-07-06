pub mod components;
mod hooks;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::{ToolbarButton, ToolbarButtonProps};
use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::{UploadInfoDialog, UploadInfoDialogProps};
use components::upload_button_input::{UploadButtonInput, UploadButtonInputProps};
use dioxus::prelude::*;
use hooks::use_upload_button;
use style::CLASS;
assert_component!(UploadButton);

/// The toolbar's import control: a hidden file input plus the visible upload button
/// that opens the "how to import" dialog. Sources the document service and the
/// upload status from context itself, so nothing is threaded in.
#[component]
pub fn UploadButton() -> Element {
    let model = use_upload_button();
    let input = UploadButtonInputProps::from(&model);
    let button = ToolbarButtonProps::from(&model);
    let dialog = UploadInfoDialogProps::from(&model);
    rsx! {
        div {
            class: CLASS,
            UploadButtonInput { ..input }
            ToolbarButton { ..button }
            UploadInfoDialog { ..dialog }
        }
    }
}
