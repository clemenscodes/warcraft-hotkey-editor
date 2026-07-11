use dioxus::prelude::*;

/// The published `View` contract mirroring [`ActiveHeroLevelOptionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveHeroLevelOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveHeroLevelOptionView {}
