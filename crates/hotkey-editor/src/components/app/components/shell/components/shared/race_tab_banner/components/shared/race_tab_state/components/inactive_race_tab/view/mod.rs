use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InactiveRaceTabView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for InactiveRaceTabView {}
