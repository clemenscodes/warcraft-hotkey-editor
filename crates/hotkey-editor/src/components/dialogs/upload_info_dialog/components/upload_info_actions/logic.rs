use super::props::UploadInfoActionsProps;
use crate::components::shared::button::{ButtonProps, ButtonVariant};
use dioxus::prelude::*;

/// The import dialog's two footer buttons, each finished with its variant, label,
/// and handler.
pub(super) struct UploadInfoActionsButtons {
    pub(super) cancel: ButtonProps,
    pub(super) choose_file: ButtonProps,
}

impl From<&UploadInfoActionsProps> for UploadInfoActionsButtons {
    fn from(props: &UploadInfoActionsProps) -> Self {
        let cancel = ButtonProps {
            variant: ButtonVariant::Secondary,
            onclick: props.on_cancel,
            children: rsx! { "Cancel" },
        };
        let choose_file = ButtonProps {
            variant: ButtonVariant::Primary,
            onclick: props.on_choose_file,
            children: rsx! { "Choose File" },
        };
        Self {
            cancel,
            choose_file,
        }
    }
}
