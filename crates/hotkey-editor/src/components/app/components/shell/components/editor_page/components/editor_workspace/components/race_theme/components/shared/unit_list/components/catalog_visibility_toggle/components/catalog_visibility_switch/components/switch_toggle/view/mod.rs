use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SwitchToggleView {
    pub is_on: bool,
    pub title: Option<&'static str>,
    pub aria_label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for SwitchToggleView {}
