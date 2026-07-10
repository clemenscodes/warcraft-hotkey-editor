use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The published `View` contract mirroring [`SystemHotkeysBreadcrumbsMenuProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsMenuView {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub picker_open: Signal<bool>,
    pub is_open: bool,
}

impl ddd::View for SystemHotkeysBreadcrumbsMenuView {}
