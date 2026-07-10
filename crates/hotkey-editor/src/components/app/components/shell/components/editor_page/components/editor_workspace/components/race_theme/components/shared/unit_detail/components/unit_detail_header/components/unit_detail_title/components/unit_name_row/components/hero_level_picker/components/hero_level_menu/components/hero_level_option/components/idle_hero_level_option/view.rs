use dioxus::prelude::*;

/// The published `View` contract mirroring [`IdleHeroLevelOptionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IdleHeroLevelOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleHeroLevelOptionView {}
