pub mod components;
mod props;

use super::dialog::Dialog;
use crate::assert_component;
use components::upload_info_actions::{UploadInfoActions, UploadInfoActionsProps};
use components::upload_info_content::UploadInfoContent;
use dioxus::prelude::*;
pub use props::UploadInfoDialogProps;
assert_component!(UploadInfoDialog);

/// Tells the player where to find CustomKeys.txt and opens the file picker. A
/// component that composes the `Dialog` base with the content block and the
/// action row.
#[component]
pub fn UploadInfoDialog(props: UploadInfoDialogProps) -> Element {
    let actions = UploadInfoActionsProps::from(&props);
    rsx! {
        Dialog { open: props.open, title: "Import CustomKeys.txt",
            UploadInfoContent {}
            UploadInfoActions { ..actions }
        }
    }
}
