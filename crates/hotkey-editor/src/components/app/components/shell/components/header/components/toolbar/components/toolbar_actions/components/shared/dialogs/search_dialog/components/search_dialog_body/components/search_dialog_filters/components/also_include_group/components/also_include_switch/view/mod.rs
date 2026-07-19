use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AlsoIncludeSwitchView {
    pub label: &'static str,
    pub popover_text: &'static str,
    pub is_on: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for AlsoIncludeSwitchView {}
