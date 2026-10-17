use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InactiveCollisionKindTabView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for InactiveCollisionKindTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InactiveCollisionKindTabView {}
