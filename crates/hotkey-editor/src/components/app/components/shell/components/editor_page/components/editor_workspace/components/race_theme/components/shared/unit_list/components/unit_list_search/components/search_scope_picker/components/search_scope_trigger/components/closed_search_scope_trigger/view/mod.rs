use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ClosedSearchScopeTriggerView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ClosedSearchScopeTriggerView {}
