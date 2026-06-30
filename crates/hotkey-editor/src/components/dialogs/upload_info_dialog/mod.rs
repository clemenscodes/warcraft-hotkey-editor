pub mod components;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use components::upload_info_actions::{UploadInfoActions, UploadInfoActionsProps};
use components::upload_info_content::UploadInfoContent;

pub use props::UploadInfoDialogProps;

/// Tells the player where to find CustomKeys.txt and opens the file picker. A
/// variant of the `Dialog` base: it composes the shell with the content block
/// and the action row.
#[component]
pub fn UploadInfoDialog(props: UploadInfoDialogProps) -> Element {
    let actions = UploadInfoActionsProps::from(&props);
    rsx! {
        Dialog {
            open: props.open,
            title: "Import CustomKeys.txt",
            UploadInfoContent {}
            UploadInfoActions { ..actions }
        }
    }
}
