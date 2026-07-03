pub mod components;
mod hooks;
mod props;
mod style;

use crate::components::shell::header::components::header_toolbar::components::shared::toolbar_button::{ToolbarButton, ToolbarButtonProps};
use crate::assert_component;
use crate::components::dialogs::info_dialogs::upload_info_dialog::{UploadInfoDialog, UploadInfoDialogProps};
use components::upload_button_input::{UploadButtonInput, UploadButtonInputProps};
use dioxus::prelude::*;
use hooks::use_upload_button;
pub use props::UploadButtonProps;
use style::CLASS;
assert_component!(UploadButton);

/// The toolbar's import control: a hidden file input plus the visible upload button
/// that opens the "how to import" dialog.
#[component]
pub fn UploadButton(props: UploadButtonProps) -> Element {
    let model = use_upload_button(&props);
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
