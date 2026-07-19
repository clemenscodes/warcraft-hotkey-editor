use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FilterSwitchView {
    pub is_on: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for FilterSwitchView {}
