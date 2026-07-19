use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CollisionKindTabView {
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for CollisionKindTabView {}
