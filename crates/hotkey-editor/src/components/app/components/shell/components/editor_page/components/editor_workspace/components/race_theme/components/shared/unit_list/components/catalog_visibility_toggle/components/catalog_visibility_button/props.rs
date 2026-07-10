use super::view::CatalogVisibilityButtonView;
use dioxus::prelude::*;

/// One catalog-visibility toggle: its label, tooltip, current on/off state, and the
/// handler that flips it.
#[derive(Props, Clone, PartialEq)]
pub struct CatalogVisibilityButtonProps {
    pub label: &'static str,
    pub title: &'static str,
    pub is_active: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&CatalogVisibilityButtonView> for CatalogVisibilityButtonProps {
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

impl ddd::Props for CatalogVisibilityButtonProps {
    type View = CatalogVisibilityButtonView;
}
