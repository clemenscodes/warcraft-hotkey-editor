use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogCloseView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for DialogCloseView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DialogCloseView {}
