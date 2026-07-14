use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsView {
    pub active_category: Signal<SystemHotkeysCategory>,
}

impl ddd::View for SystemHotkeysBreadcrumbsView {}
