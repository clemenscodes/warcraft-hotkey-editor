use super::view::SystemHotkeysBreadcrumbsMenuView;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsMenuModel {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub picker_open: Signal<bool>,
    pub is_open: bool,
}

impl From<&SystemHotkeysBreadcrumbsMenuView> for SystemHotkeysBreadcrumbsMenuModel {
    fn from(view: &SystemHotkeysBreadcrumbsMenuView) -> Self {
        let SystemHotkeysBreadcrumbsMenuView {
            active_category,
            picker_open,
            is_open,
        } = view.clone();
        Self {
            active_category,
            picker_open,
            is_open,
        }
    }
}

impl ddd::Model for SystemHotkeysBreadcrumbsMenuModel {
    type View = SystemHotkeysBreadcrumbsMenuView;
}
