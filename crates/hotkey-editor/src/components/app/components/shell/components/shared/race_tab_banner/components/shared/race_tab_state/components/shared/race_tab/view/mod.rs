use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceTabView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for RaceTabView {}
