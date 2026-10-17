use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CatalogVisibilityButtonView {
    pub label: &'static str,
    pub title: &'static str,
    pub is_active: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl ddd::Layered for CatalogVisibilityButtonView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CatalogVisibilityButtonView {}
