use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogCloseView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for DialogCloseView {}
