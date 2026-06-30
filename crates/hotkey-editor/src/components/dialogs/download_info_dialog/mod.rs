pub mod components;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use crate::assert_component;
use components::download_info_actions::{DownloadInfoActions, DownloadInfoActionsProps};
use components::download_info_content::DownloadInfoContent;

pub use props::DownloadInfoDialogProps;

assert_component!(DownloadInfoDialog);

/// Tells the player where CustomKeys.txt must go and confirms the download. A
/// component that composes the `Dialog` base with the content block and the
/// action row, no markup of its own beyond that.
#[component]
pub fn DownloadInfoDialog(props: DownloadInfoDialogProps) -> Element {
    let actions = DownloadInfoActionsProps::from(&props);
    rsx! {
        Dialog {
            open: props.open,
            title: "Download CustomKeys.txt",
            DownloadInfoContent {}
            DownloadInfoActions { ..actions }
        }
    }
}
