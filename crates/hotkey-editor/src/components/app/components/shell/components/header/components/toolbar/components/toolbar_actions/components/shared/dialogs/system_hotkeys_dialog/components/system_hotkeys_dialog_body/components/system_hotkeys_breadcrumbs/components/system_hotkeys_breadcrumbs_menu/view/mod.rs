use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsMenuView {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub picker_open: Signal<bool>,
    pub is_open: bool,
}

impl ddd::Layered for SystemHotkeysBreadcrumbsMenuView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysBreadcrumbsMenuView {}
