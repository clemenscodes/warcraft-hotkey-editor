use dioxus::prelude::*;

/// The published `View` contract mirroring [`ConflictMoreProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictMoreView {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ConflictMoreView {}
