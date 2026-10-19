use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InactiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::Layered for InactiveCategoryTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InactiveCategoryTabView {}
