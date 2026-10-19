use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveHeroLevelOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ActiveHeroLevelOptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveHeroLevelOptionView {}
