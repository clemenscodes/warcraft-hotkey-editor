use super::view::CatalogVisibilityButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CatalogVisibilityButtonModel {
    pub label: &'static str,
    pub title: &'static str,
    pub is_active: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&CatalogVisibilityButtonView> for CatalogVisibilityButtonModel {
    fn from(view: &CatalogVisibilityButtonView) -> Self {
        let CatalogVisibilityButtonView {
            label,
            title,
            is_active,
            on_toggle,
        } = view.clone();
        Self {
            label,
            title,
            is_active,
            on_toggle,
        }
    }
}

impl ddd::Model for CatalogVisibilityButtonModel {
    type View = CatalogVisibilityButtonView;
}
