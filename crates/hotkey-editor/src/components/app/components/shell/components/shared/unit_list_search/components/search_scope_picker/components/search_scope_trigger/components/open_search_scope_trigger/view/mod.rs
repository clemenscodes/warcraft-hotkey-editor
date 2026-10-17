use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct OpenSearchScopeTriggerView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for OpenSearchScopeTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for OpenSearchScopeTriggerView {}
