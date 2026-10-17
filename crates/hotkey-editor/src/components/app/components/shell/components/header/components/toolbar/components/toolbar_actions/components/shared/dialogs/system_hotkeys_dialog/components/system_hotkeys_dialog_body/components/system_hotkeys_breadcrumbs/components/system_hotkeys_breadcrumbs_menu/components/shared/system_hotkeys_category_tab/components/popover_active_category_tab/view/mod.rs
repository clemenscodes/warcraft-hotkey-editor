use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PopoverActiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::Layered for PopoverActiveCategoryTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PopoverActiveCategoryTabView {}
