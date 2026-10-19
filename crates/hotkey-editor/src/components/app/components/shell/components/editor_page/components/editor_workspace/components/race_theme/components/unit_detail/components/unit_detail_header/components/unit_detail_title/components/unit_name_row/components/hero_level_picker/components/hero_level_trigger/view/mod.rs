use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelTriggerView {
    pub number: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for HeroLevelTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HeroLevelTriggerView {}
