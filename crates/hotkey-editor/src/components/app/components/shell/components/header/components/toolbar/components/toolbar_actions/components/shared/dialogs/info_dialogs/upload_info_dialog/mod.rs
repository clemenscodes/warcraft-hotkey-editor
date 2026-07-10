mod data;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::InfoDialog;
use dioxus::prelude::*;
use logic::UploadInfoDialogModel;
use props::UploadInfoDialogProps;
use tw_macro::assert_component;

/// Tells the player where to find CustomKeys.txt and opens the file picker. A
/// thin variant that fills the shared `InfoDialog` shell with the import title,
/// copy, and choose-file handler.
#[component]
pub fn UploadInfoDialog(props: UploadInfoDialogProps) -> Element {
    let UploadInfoDialogModel {
        open,
        title,
        intro,
        warning,
        primary_label,
        on_primary,
        on_cancel,
    } = UploadInfoDialogModel::from(&props);
    rsx! {
        InfoDialog {
            open,
            title,
            intro,
            warning,
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}

assert_component!(UploadInfoDialog);
