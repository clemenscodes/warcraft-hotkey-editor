use dioxus::prelude::*;

/// The published `View` contract mirroring [`RaceTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RaceTabView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for RaceTabView {}
