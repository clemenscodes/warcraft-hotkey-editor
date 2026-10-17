use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ConflictAbilityTriggerView {
    pub onclick: EventHandler<MouseEvent>,
    pub icon_src: Option<String>,
    pub icon_alt: String,
}

impl ddd::Layered for ConflictAbilityTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictAbilityTriggerView {}
