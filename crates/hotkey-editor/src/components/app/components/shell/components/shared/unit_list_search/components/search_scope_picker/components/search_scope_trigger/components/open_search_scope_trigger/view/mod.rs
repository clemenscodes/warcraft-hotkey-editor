use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct OpenSearchScopeTriggerView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for OpenSearchScopeTriggerView {}
