use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogCloseHostView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for DialogCloseHostView {}
