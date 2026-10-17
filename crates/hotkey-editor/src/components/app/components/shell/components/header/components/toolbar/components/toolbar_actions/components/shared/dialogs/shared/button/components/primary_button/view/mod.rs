use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PrimaryButtonView {
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::Layered for PrimaryButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PrimaryButtonView {}
