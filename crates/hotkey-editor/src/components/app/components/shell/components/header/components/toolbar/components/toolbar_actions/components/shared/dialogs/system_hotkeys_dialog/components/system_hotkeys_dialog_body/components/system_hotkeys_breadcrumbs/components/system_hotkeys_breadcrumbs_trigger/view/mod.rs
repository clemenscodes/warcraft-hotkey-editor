use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerView {
    pub label: String,
    pub is_open: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl ddd::View for SystemHotkeysBreadcrumbsTriggerView {}
