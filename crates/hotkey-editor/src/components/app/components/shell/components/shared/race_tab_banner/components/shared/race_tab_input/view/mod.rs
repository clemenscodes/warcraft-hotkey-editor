use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceTabInputView {
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for RaceTabInputView {}
