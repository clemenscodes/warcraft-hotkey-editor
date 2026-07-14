use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HelpDismissView {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl ddd::View for HelpDismissView {}
