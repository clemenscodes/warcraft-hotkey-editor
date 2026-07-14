use super::view::SystemHotkeysBreadcrumbsTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerModel {
    #[props(into)]
    pub label: String,
    pub is_open: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

impl From<&SystemHotkeysBreadcrumbsTriggerView> for SystemHotkeysBreadcrumbsTriggerModel {
    fn from(view: &SystemHotkeysBreadcrumbsTriggerView) -> Self {
        let SystemHotkeysBreadcrumbsTriggerView {
            label,
            is_open,
            on_toggle,
        } = view.clone();
        Self {
            label,
            is_open,
            on_toggle,
        }
    }
}

impl ddd::Model for SystemHotkeysBreadcrumbsTriggerModel {
    type View = SystemHotkeysBreadcrumbsTriggerView;
}
