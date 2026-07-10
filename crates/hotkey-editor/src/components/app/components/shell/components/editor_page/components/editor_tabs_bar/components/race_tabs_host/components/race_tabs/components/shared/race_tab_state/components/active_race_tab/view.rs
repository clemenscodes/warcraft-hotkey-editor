use dioxus::prelude::*;

/// The published `View` contract mirroring [`ActiveRaceTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveRaceTabView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for ActiveRaceTabView {}
