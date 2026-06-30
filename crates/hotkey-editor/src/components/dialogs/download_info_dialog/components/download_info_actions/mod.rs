mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::shared::button::{Button, ButtonVariant};
use style::CLASS;

pub use props::DownloadInfoActionsProps;

assert_component!(DownloadInfoActions);

/// The download dialog's right-aligned action row: the cancel and download
/// buttons.
#[component]
pub fn DownloadInfoActions(props: DownloadInfoActionsProps) -> Element {
    let on_cancel = props.on_cancel;
    let on_download = props.on_download;
    rsx! {
        div {
            class: CLASS,
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
