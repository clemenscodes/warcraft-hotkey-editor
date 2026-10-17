use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveToggleButtonView {
    pub label: &'static str,
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::Layered for ActiveToggleButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveToggleButtonView {}
