use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FilterTriggerView {
    pub active: bool,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for FilterTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FilterTriggerView {}
