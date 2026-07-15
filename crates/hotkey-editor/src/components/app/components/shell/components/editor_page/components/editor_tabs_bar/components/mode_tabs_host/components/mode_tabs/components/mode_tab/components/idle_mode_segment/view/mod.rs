use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleModeSegmentView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for IdleModeSegmentView {}
