use dioxus::prelude::*;

/// The published `View` contract mirroring [`IdleToggleButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IdleToggleButtonView {
    pub label: &'static str,
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for IdleToggleButtonView {}
