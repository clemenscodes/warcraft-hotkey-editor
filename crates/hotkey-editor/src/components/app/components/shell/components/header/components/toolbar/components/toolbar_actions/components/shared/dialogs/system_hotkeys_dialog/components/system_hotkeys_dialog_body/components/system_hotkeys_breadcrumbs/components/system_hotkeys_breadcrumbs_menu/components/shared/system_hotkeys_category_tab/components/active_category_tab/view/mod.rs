use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::Layered for ActiveCategoryTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveCategoryTabView {}
