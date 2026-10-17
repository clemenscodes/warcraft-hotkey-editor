use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveResolveSectionTabView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ActiveResolveSectionTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveResolveSectionTabView {}
