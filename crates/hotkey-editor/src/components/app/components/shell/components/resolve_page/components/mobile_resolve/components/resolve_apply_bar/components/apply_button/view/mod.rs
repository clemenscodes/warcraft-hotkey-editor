use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ApplyButtonView {
    pub running: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ApplyButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ApplyButtonView {}
