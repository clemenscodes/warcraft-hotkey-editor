use super::view::SystemHotkeysBreadcrumbsTriggerCaretView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerCaretModel {
    pub is_open: bool,
}

impl From<&SystemHotkeysBreadcrumbsTriggerCaretView> for SystemHotkeysBreadcrumbsTriggerCaretModel {
    fn from(view: &SystemHotkeysBreadcrumbsTriggerCaretView) -> Self {
        let SystemHotkeysBreadcrumbsTriggerCaretView { is_open } = view.clone();
        Self { is_open }
    }
}

impl ddd::Model for SystemHotkeysBreadcrumbsTriggerCaretModel {
    type View = SystemHotkeysBreadcrumbsTriggerCaretView;
}
