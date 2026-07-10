use super::props::InfoActionsProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::data::CANCEL;
use dioxus::prelude::*;

/// The action row's two buttons as plain domain values: each label and click
/// handler. The variant is fixed per slot (cancel is secondary, primary is
/// primary), so the row places each `Button` with these named fields.
pub(super) struct InfoActionsButtons {
    pub(super) cancel_label: String,
    pub(super) on_cancel: EventHandler<MouseEvent>,
    pub(super) primary_label: String,
    pub(super) on_primary: EventHandler<MouseEvent>,
}

impl From<&InfoActionsProps> for InfoActionsButtons {
    fn from(props: &InfoActionsProps) -> Self {
        let cancel_label = String::from(CANCEL);
        let primary_label = String::from(props.primary_label);
        let on_cancel = props.on_cancel;
        let on_primary = props.on_primary;
        Self {
            cancel_label,
            on_cancel,
            primary_label,
            on_primary,
        }
    }
}
