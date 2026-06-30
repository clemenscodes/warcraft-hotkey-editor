mod props;
mod style;

use dioxus::prelude::*;

use crate::components::shared::button::{Button, ButtonVariant};
use style::UPLOAD_INFO_ACTIONS_STYLES;

pub use props::UploadInfoActionsProps;

/// The import dialog's right-aligned action row. Owns `.upload-info-actions` and
/// holds the cancel and choose-file buttons.
#[component]
pub fn UploadInfoActions(props: UploadInfoActionsProps) -> Element {
    let on_cancel = props.on_cancel;
    let on_choose_file = props.on_choose_file;
    rsx! {
        document::Stylesheet { href: UPLOAD_INFO_ACTIONS_STYLES }
        div {
            class: "upload-info-actions",
            Button {
                variant: ButtonVariant::Secondary,
                onclick: on_cancel,
                "Cancel"
            }
            Button {
                variant: ButtonVariant::Primary,
                onclick: on_choose_file,
                "Choose File"
            }
        }
    }
}
