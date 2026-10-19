use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SpillSectionTabView {
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for SpillSectionTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SpillSectionTabView {}
