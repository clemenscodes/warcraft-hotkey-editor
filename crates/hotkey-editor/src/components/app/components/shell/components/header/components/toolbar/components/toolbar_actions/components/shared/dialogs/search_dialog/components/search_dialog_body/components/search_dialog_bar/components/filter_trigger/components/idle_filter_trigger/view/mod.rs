use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleFilterTriggerView {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleFilterTriggerView {}
