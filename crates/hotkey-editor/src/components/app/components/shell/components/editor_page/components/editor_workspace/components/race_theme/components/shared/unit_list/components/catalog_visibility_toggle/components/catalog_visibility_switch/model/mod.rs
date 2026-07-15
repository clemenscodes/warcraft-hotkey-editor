use super::view::CatalogVisibilitySwitchView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CatalogVisibilitySwitchModel {
    pub label: &'static str,
    pub title: &'static str,
    pub is_active: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&CatalogVisibilitySwitchView> for CatalogVisibilitySwitchModel {
    fn from(view: &CatalogVisibilitySwitchView) -> Self {
        let CatalogVisibilitySwitchView {
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

impl ddd::Model for CatalogVisibilitySwitchModel {
    type View = CatalogVisibilitySwitchView;
}
