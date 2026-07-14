mod data;
mod model;
mod presentation;
mod view;

pub use view::DownloadInfoDialogView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::InfoDialog;
use dioxus::prelude::*;
use model::DownloadInfoDialogModel;
use presentation::{use_download_info_dialog, DownloadInfoDialogPresentation};
use tw_macro::assert_component;

#[component]
pub fn DownloadInfoDialog(props: DownloadInfoDialogModel) -> Element {
    let DownloadInfoDialogPresentation {
        open,
        on_open_change,
        title,
        intro,
        warning,
        primary_label,
        on_primary,
        on_cancel,
    } = use_download_info_dialog(&props);
    rsx! {
        InfoDialog {
            open,
            on_open_change,
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
