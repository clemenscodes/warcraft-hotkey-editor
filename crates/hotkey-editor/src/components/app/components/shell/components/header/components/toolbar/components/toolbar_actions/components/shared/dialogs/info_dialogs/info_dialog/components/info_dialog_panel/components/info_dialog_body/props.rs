use dioxus::prelude::*;

/// The info dialog's scroll region inputs: the centered instruction copy and the
/// trailing action row's label and handlers.
#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogBodyProps {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}
