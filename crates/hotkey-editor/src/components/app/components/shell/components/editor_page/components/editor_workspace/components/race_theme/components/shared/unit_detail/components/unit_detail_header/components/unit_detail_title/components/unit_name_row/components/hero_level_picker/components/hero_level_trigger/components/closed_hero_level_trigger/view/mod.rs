use dioxus::prelude::*;

/// The published `View` contract mirroring [`ClosedHeroLevelTriggerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ClosedHeroLevelTriggerView {
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ClosedHeroLevelTriggerView {}
