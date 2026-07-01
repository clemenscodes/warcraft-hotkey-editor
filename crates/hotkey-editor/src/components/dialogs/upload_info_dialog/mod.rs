pub mod components;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
pub use props::UploadInfoDialogProps;

/// Tells the player where to find CustomKeys.txt and opens the file picker. A
/// component that composes the `Dialog` base with the content block and the
/// action row.
#[component]
pub fn UploadInfoDialog(props: UploadInfoDialogProps) -> Element {
    rsx! {
        Dialog { ..DialogProps::from(&props) }
    }
}
