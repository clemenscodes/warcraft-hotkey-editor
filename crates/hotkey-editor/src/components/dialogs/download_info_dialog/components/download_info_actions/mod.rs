mod props;
mod style;

use dioxus::prelude::*;

use crate::components::shared::button::{Button, ButtonVariant};
use style::DOWNLOAD_INFO_ACTIONS_STYLES;

pub use props::DownloadInfoActionsProps;

/// The download dialog's right-aligned action row. Owns `.download-info-actions`
/// and holds the cancel and download buttons.
#[component]
pub fn DownloadInfoActions(props: DownloadInfoActionsProps) -> Element {
    let on_cancel = props.on_cancel;
    let on_download = props.on_download;
    rsx! {
        document::Stylesheet { href: DOWNLOAD_INFO_ACTIONS_STYLES }
        div {
            class: "download-info-actions",
            Button {
                variant: ButtonVariant::Secondary,
                onclick: on_cancel,
                "Cancel"
            }
            Button {
                variant: ButtonVariant::Primary,
                onclick: on_download,
                "Download"
            }
        }
    }
}
