use super::model::InfoActionsModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::data::CANCEL;
use dioxus::prelude::*;

pub(super) struct InfoActionsPresentation {
    pub(super) cancel_label: String,
    pub(super) on_cancel: EventHandler<MouseEvent>,
    pub(super) primary_label: String,
    pub(super) on_primary: EventHandler<MouseEvent>,
}

impl From<&InfoActionsModel> for InfoActionsPresentation {
    fn from(props: &InfoActionsModel) -> Self {
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

impl ddd::Presentation for InfoActionsPresentation {
    type Model = InfoActionsModel;
}
