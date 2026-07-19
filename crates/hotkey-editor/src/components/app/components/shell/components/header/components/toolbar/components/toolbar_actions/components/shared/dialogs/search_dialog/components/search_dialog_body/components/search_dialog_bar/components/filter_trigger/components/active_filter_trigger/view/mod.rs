use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveFilterTriggerView {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveFilterTriggerView {}
