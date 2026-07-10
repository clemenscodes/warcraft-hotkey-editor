use dioxus::prelude::*;

/// The published `View` contract mirroring [`CatalogVisibilityButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CatalogVisibilityButtonView {
    pub label: &'static str,
    pub title: &'static str,
    pub is_active: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl ddd::View for CatalogVisibilityButtonView {}
