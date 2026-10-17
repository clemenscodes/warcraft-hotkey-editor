use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleFilterTriggerView {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for IdleFilterTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IdleFilterTriggerView {}
