mod data;
mod logic;
mod props;

use crate::components::dialogs::info_dialogs::info_dialog::{InfoDialog, InfoDialogConfig};
use dioxus::prelude::*;
pub use props::DownloadInfoDialogProps;

/// Tells the player where CustomKeys.txt must go and confirms the download. A
/// thin variant that fills the shared `InfoDialog` shell with the download
/// title, copy, warning, and confirm handler.
#[component]
pub fn DownloadInfoDialog(props: DownloadInfoDialogProps) -> Element {
    rsx! {
        InfoDialog { ..InfoDialogConfig::from(&props) }
    }
}
