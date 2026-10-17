use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleMobileCategoryTabView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for IdleMobileCategoryTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IdleMobileCategoryTabView {}
