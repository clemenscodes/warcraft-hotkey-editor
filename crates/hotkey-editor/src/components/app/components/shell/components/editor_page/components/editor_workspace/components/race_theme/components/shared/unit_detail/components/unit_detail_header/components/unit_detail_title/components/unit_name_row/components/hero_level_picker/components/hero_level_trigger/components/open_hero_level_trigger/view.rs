use dioxus::prelude::*;

/// The published `View` contract mirroring [`OpenHeroLevelTriggerProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct OpenHeroLevelTriggerView {
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for OpenHeroLevelTriggerView {}
