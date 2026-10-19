use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BreadcrumbView {
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for BreadcrumbView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BreadcrumbView {}
