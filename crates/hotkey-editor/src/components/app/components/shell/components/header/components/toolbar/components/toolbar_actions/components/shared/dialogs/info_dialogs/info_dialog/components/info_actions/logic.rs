use super::props::InfoActionsProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::data::CANCEL;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::button::{ButtonProps, ButtonVariant};

/// The action row's two buttons, each finished with its variant, label, and
/// handler.
pub(super) struct InfoActionsButtons {
    pub(super) cancel: ButtonProps,
    pub(super) primary: ButtonProps,
}

impl From<&InfoActionsProps> for InfoActionsButtons {
    fn from(props: &InfoActionsProps) -> Self {
        let cancel_label = String::from(CANCEL);
        let primary_label = String::from(props.primary_label);
        let cancel = ButtonProps {
            variant: ButtonVariant::Secondary,
            onclick: props.on_cancel,
            label: cancel_label,
        };
        let primary = ButtonProps {
            variant: ButtonVariant::Primary,
            onclick: props.on_primary,
            label: primary_label,
        };
        Self { cancel, primary }
    }
}
