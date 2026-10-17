use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogCloseHostView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for DialogCloseHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DialogCloseHostView {}
