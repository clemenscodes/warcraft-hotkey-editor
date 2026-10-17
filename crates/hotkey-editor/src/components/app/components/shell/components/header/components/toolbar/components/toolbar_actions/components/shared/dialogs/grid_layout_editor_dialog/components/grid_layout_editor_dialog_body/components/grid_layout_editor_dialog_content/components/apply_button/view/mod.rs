use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ApplyButtonView {
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::Layered for ApplyButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ApplyButtonView {}
