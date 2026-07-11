use super::view::UnitCategoryHeadingView;
use dioxus::prelude::*;

/// A category heading: its label, collapsed state, and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCategoryHeadingModel {
    #[props(into)]
    pub label: String,
    pub is_collapsed: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&UnitCategoryHeadingView> for UnitCategoryHeadingModel {
    fn from(view: &UnitCategoryHeadingView) -> Self {
        let UnitCategoryHeadingView {
            label,
            is_collapsed,
            on_toggle,
        } = view.clone();
        Self {
            label,
            is_collapsed,
            on_toggle,
        }
    }
}

impl ddd::Model for UnitCategoryHeadingModel {
    type View = UnitCategoryHeadingView;
}
