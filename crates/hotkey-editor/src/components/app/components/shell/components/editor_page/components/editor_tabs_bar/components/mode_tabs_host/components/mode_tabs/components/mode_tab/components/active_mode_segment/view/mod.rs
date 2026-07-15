use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveModeSegmentView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for ActiveModeSegmentView {}
