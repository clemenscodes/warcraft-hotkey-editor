use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleHeroLevelOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleHeroLevelOptionView {}
