pub mod components;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
pub use props::DownloadInfoDialogProps;

/// Tells the player where CustomKeys.txt must go and confirms the download. A
/// component that composes the `Dialog` base with the content block and the
/// action row, no markup of its own beyond that.
#[component]
pub fn DownloadInfoDialog(props: DownloadInfoDialogProps) -> Element {
    rsx! {
        Dialog { ..DialogProps::from(&props) }
    }
}
