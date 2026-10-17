use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ResolveSectionTabView {
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ResolveSectionTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ResolveSectionTabView {}
