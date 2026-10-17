use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleHeroLevelOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for IdleHeroLevelOptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IdleHeroLevelOptionView {}
