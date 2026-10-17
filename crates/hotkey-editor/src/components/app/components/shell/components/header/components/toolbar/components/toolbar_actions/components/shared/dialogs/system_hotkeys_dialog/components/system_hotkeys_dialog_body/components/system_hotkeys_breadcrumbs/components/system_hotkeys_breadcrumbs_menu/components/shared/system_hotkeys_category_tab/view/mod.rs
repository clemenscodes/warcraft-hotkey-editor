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

impl ddd::Layered for SystemHotkeysCategoryTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysCategoryTabView {}
