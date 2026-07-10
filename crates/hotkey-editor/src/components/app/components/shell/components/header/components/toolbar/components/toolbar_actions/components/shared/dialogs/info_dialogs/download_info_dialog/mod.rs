mod data;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::InfoDialog;
use dioxus::prelude::*;
use logic::DownloadInfoDialogModel;
use props::DownloadInfoDialogProps;
use tw_macro::assert_component;

/// Tells the player where CustomKeys.txt must go and confirms the download. A
/// thin variant that fills the shared `InfoDialog` shell with the download
/// title, copy, warning, and confirm handler.
#[component]
pub fn DownloadInfoDialog(props: DownloadInfoDialogProps) -> Element {
    let DownloadInfoDialogModel {
        open,
        title,
        intro,
        warning,
        primary_label,
        on_primary,
        on_cancel,
    } = DownloadInfoDialogModel::from(&props);
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

assert_component!(DownloadInfoDialog);
