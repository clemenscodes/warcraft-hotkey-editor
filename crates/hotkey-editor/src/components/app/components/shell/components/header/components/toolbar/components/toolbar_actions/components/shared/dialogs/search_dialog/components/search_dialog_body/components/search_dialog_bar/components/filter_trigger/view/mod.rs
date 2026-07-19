use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FilterTriggerView {
    pub active: bool,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for FilterTriggerView {}
