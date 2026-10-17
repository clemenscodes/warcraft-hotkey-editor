use super::state::ButtonVariant;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ButtonView {
    pub variant: ButtonVariant,
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::Layered for ButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ButtonView {}
