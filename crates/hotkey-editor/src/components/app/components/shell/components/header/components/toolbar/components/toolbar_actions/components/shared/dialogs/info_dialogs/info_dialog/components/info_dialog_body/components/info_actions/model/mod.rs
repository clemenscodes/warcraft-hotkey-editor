use super::view::InfoActionsView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoActionsModel {
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}

impl From<&InfoActionsView> for InfoActionsModel {
    fn from(view: &InfoActionsView) -> Self {
        let InfoActionsView {
            primary_label,
            on_primary,
            on_cancel,
        } = view.clone();
        Self {
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}

impl ddd::Model for InfoActionsModel {
    type View = InfoActionsView;
}
