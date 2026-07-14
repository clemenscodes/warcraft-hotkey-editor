use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsMenuView {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub picker_open: Signal<bool>,
    pub is_open: bool,
}

impl ddd::View for SystemHotkeysBreadcrumbsMenuView {}
