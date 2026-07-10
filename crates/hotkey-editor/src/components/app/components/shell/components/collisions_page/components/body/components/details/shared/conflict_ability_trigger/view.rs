use dioxus::prelude::*;

/// The published `View` contract mirroring [`ConflictAbilityTriggerProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictAbilityTriggerView {
    pub onclick: EventHandler<MouseEvent>,
    pub icon_src: Option<String>,
    pub icon_alt: String,
}

impl ddd::View for ConflictAbilityTriggerView {}
