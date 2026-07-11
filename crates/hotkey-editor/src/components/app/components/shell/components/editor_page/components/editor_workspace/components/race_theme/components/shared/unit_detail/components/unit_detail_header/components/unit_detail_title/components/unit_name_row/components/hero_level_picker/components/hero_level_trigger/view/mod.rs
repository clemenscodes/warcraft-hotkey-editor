use dioxus::prelude::*;

/// The published `View` contract mirroring [`HeroLevelTriggerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HeroLevelTriggerView {
    pub number: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for HeroLevelTriggerView {}
