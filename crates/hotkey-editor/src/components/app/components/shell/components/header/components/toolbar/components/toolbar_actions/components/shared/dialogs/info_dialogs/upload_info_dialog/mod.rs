mod data;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::{InfoDialog, InfoDialogConfig};
use dioxus::prelude::*;
use tw_macro::assert_component;
pub use props::UploadInfoDialogProps;

/// Tells the player where to find CustomKeys.txt and opens the file picker. A
/// thin variant that fills the shared `InfoDialog` shell with the import title,
/// copy, and choose-file handler.
#[component]
pub fn UploadInfoDialog(props: UploadInfoDialogProps) -> Element {
    rsx! {
        InfoDialog { ..InfoDialogConfig::from(&props) }
    }
}

assert_component!(UploadInfoDialog);
