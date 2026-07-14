use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToggleButtonView {
    pub label: &'static str,
    pub active: bool,
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for ToggleButtonView {}
