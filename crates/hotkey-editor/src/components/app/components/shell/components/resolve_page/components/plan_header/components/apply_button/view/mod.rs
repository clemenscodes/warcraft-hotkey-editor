use dioxus::prelude::*;

/// The published `View` contract mirroring [`ApplyButtonModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ApplyButtonView {
    pub running: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ApplyButtonView {}
