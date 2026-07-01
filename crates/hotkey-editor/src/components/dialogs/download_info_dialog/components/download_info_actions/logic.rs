use super::props::DownloadInfoActionsProps;
use crate::components::shared::button::{ButtonProps, ButtonVariant};
use dioxus::prelude::*;

/// The dialog's two footer buttons, each finished with its variant, label, and
/// handler.
pub(super) struct DownloadInfoActionsButtons {
    pub(super) cancel: ButtonProps,
    pub(super) download: ButtonProps,
}

impl From<&DownloadInfoActionsProps> for DownloadInfoActionsButtons {
    fn from(props: &DownloadInfoActionsProps) -> Self {
        let cancel = ButtonProps {
            variant: ButtonVariant::Secondary,
            onclick: props.on_cancel,
            children: rsx! { "Cancel" },
        };
        let download = ButtonProps {
            variant: ButtonVariant::Primary,
            onclick: props.on_download,
            children: rsx! { "Download" },
        };
        Self { cancel, download }
    }
}
