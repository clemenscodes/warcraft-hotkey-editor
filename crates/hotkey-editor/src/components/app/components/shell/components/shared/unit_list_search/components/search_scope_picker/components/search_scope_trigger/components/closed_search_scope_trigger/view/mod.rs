use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ClosedSearchScopeTriggerView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ClosedSearchScopeTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ClosedSearchScopeTriggerView {}
