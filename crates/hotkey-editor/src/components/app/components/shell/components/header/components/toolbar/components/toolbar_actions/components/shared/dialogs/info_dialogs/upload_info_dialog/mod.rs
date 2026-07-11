mod data;
mod model;
mod presentation;
mod view;

pub use view::UploadInfoDialogView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::InfoDialog;
use dioxus::prelude::*;
use presentation::UploadInfoDialogPresentation;
use model::UploadInfoDialogModel;
use tw_macro::assert_component;

/// Tells the player where to find CustomKeys.txt and opens the file picker. A
/// thin variant that fills the shared `InfoDialog` shell with the import title,
/// copy, and choose-file handler.
#[component]
pub fn UploadInfoDialog(props: UploadInfoDialogModel) -> Element {
    let UploadInfoDialogPresentation {
        open,
        title,
        intro,
        warning,
        primary_label,
        on_primary,
        on_cancel,
    } = UploadInfoDialogPresentation::from(&props);
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
