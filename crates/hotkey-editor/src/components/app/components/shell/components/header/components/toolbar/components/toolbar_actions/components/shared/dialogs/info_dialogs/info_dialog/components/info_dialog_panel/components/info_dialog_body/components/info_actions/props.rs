use super::view::InfoActionsView;
use dioxus::prelude::*;

/// The action row's inputs: the primary button's label and handler, plus the
/// cancel handler.
#[derive(Props, Clone, PartialEq)]
pub struct InfoActionsProps {
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}

impl From<&InfoActionsView> for InfoActionsProps {
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

impl ddd::Props for InfoActionsProps {
    type View = InfoActionsView;
}
