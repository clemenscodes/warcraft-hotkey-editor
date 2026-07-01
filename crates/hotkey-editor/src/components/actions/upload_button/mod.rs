pub mod components;
mod hooks;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::upload_info_dialog::UploadInfoDialog;
use crate::components::shared::icons::ICON_UPLOAD;
use crate::components::shared::toolbar_button::ToolbarButton;
use components::upload_button_input::UploadButtonInput;
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
    rsx! {
        div { class: CLASS,
            UploadButtonInput { on_change: model.on_file_change }
            ToolbarButton {
                icon: ICON_UPLOAD,
                aria_label: "Upload CustomKeys.txt",
                onclick: model.on_open_info,
            }
            UploadInfoDialog { open: model.info_open }
        }
    }
}
