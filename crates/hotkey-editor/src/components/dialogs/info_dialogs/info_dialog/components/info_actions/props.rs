use crate::components::dialogs::info_dialogs::info_dialog::InfoDialogConfig;
use dioxus::prelude::*;

/// The action row's inputs: the primary button's label and handler, plus the
/// cancel handler. Built from the dialog config by conversion.
#[derive(Props, Clone, PartialEq)]
pub struct InfoActionsProps {
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}

impl From<&InfoDialogConfig> for InfoActionsProps {
    fn from(props: &InfoDialogConfig) -> Self {
        let primary_label = props.primary_label;
        let on_primary = props.on_primary;
        let on_cancel = props.on_cancel;
        Self {
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}
