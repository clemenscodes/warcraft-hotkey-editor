use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CatalogVisibilitySwitchView {
    pub label: &'static str,
    pub title: &'static str,
    pub is_active: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl ddd::View for CatalogVisibilitySwitchView {}
