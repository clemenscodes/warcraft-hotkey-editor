use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SectionTabBodyView {
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for SectionTabBodyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SectionTabBodyView {}
