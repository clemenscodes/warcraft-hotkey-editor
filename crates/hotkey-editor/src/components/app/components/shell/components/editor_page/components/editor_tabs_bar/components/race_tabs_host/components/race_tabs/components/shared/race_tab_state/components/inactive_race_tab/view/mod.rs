use dioxus::prelude::*;

/// The published `View` contract mirroring [`InactiveRaceTabModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InactiveRaceTabView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for InactiveRaceTabView {}
