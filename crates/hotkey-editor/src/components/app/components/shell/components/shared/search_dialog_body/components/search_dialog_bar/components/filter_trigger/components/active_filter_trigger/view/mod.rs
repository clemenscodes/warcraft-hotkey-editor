use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveFilterTriggerView {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ActiveFilterTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveFilterTriggerView {}
