use super::view::SystemHotkeysCategoryTabView;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysCategoryTabModel {
    pub category: SystemHotkeysCategory,
    pub is_active: bool,
    pub has_separator: bool,
    #[props(default = false)]
    pub menu_open: bool,
    pub picker_open: Signal<bool>,
}

impl From<&SystemHotkeysCategoryTabView> for SystemHotkeysCategoryTabModel {
    fn from(view: &SystemHotkeysCategoryTabView) -> Self {
        let SystemHotkeysCategoryTabView {
            category,
            is_active,
            has_separator,
            menu_open,
            picker_open,
        } = view.clone();
        Self {
            category,
            is_active,
            has_separator,
            menu_open,
            picker_open,
        }
    }
}

impl ddd::Model for SystemHotkeysCategoryTabModel {
    type View = SystemHotkeysCategoryTabView;
}
