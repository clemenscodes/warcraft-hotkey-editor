use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The published `View` contract mirroring [`SystemHotkeysCategoryTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysCategoryTabView {
    pub category: SystemHotkeysCategory,
    pub is_active: bool,
    pub has_separator: bool,
    pub menu_open: bool,
    pub picker_open: Signal<bool>,
}

impl ddd::View for SystemHotkeysCategoryTabView {}
