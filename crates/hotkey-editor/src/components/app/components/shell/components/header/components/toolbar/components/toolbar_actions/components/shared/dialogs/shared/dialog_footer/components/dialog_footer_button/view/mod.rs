use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogFooterButtonView {
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::Layered for DialogFooterButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DialogFooterButtonView {}
