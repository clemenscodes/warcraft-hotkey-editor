use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveCollisionKindTabView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ActiveCollisionKindTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveCollisionKindTabView {}
