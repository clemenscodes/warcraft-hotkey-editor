use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerView {
    pub label: String,
    pub is_open: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl ddd::Layered for SystemHotkeysBreadcrumbsTriggerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysBreadcrumbsTriggerView {}
