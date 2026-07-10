use super::view::SystemHotkeysBreadcrumbsMenuView;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The menu's inputs: the active category (read + written by its tabs), the shared
/// open signal (tabs close it on select), and whether the dropdown is open (selects
/// the popover xor tab-bar look).
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsMenuProps {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub picker_open: Signal<bool>,
    pub is_open: bool,
}

impl From<&SystemHotkeysBreadcrumbsMenuView> for SystemHotkeysBreadcrumbsMenuProps {
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

impl ddd::Props for SystemHotkeysBreadcrumbsMenuProps {
    type View = SystemHotkeysBreadcrumbsMenuView;
}
