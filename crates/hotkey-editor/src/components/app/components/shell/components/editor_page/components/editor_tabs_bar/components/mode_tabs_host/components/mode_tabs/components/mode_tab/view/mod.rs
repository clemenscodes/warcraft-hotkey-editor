use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ModeTabView {
    pub label: &'static str,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for ModeTabView {}
