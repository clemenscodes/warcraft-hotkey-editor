use dioxus::prelude::*;

/// The published `View` contract mirroring [`ModeTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ModeTabView {
    pub label: &'static str,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for ModeTabView {}
