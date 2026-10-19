use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RaceScopeTriggerView {
    pub summary: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for RaceScopeTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceScopeTriggerView {}
