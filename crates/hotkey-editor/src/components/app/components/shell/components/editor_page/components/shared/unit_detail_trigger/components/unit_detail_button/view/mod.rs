use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UnitDetailButtonView {
    pub src: Option<String>,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for UnitDetailButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitDetailButtonView {}
