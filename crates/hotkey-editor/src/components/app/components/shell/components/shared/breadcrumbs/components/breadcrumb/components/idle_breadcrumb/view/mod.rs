use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleBreadcrumbView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for IdleBreadcrumbView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IdleBreadcrumbView {}
