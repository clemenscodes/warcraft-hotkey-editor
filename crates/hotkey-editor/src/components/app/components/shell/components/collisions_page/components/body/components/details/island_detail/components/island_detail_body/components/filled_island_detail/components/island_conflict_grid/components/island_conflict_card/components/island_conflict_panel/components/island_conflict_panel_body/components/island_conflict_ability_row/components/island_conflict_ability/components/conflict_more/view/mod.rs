use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ConflictMoreView {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ConflictMoreView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictMoreView {}
