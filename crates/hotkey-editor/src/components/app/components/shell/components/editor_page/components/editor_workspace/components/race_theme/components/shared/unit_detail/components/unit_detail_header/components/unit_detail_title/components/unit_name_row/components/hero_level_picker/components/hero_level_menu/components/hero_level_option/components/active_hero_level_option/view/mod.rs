use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveHeroLevelOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveHeroLevelOptionView {}
