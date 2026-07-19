use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InfoPopoverBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for InfoPopoverBackdropView {}
