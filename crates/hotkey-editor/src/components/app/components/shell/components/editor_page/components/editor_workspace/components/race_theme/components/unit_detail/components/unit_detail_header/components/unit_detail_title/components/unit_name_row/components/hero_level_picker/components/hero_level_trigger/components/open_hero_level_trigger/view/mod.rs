use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct OpenHeroLevelTriggerView {
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for OpenHeroLevelTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for OpenHeroLevelTriggerView {}
