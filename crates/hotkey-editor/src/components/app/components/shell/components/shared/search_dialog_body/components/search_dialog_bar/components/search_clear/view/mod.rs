use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchClearView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for SearchClearView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchClearView {}
