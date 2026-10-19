use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UnitCategoryHeadingView {
    pub label: String,
    pub is_collapsed: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl ddd::Layered for UnitCategoryHeadingView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitCategoryHeadingView {}
