use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ConflictMoreView {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ConflictMoreView {}
