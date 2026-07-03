use super::props::InfoActionsProps;
use crate::components::dialogs::info_dialogs::info_dialog::data::CANCEL;
use crate::components::dialogs::shared::button::{ButtonProps, ButtonVariant};
use dioxus::prelude::*;

/// The action row's two buttons, each finished with its variant, label, and
/// handler.
pub(super) struct InfoActionsButtons {
    pub(super) cancel: ButtonProps,
    pub(super) primary: ButtonProps,
}

impl From<&InfoActionsProps> for InfoActionsButtons {
    fn from(props: &InfoActionsProps) -> Self {
        let primary_label = props.primary_label;
        let cancel = ButtonProps {
            variant: ButtonVariant::Secondary,
            onclick: props.on_cancel,
            children: rsx! { "{CANCEL}" },
        };
        let primary = ButtonProps {
            variant: ButtonVariant::Primary,
            onclick: props.on_primary,
            children: rsx! { "{primary_label}" },
        };
        Self { cancel, primary }
    }
}
