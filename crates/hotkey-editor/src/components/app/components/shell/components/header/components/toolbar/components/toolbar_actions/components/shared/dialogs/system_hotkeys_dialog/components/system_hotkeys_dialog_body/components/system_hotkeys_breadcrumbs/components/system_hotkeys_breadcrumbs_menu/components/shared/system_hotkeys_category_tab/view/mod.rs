use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysCategoryTabView {
    pub category: SystemHotkeysCategory,
    pub is_active: bool,
    pub has_separator: bool,
    pub menu_open: bool,
    pub picker_open: Signal<bool>,
}

impl ddd::View for SystemHotkeysCategoryTabView {}
