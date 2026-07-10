use dioxus::prelude::*;

/// The action row's inputs: the primary button's label and handler, plus the
/// cancel handler.
#[derive(Props, Clone, PartialEq)]
pub struct InfoActionsProps {
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}
