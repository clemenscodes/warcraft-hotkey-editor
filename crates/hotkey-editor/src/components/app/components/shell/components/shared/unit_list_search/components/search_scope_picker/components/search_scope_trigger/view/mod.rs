use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchScopeTriggerView {
    pub label: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for SearchScopeTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchScopeTriggerView {}
