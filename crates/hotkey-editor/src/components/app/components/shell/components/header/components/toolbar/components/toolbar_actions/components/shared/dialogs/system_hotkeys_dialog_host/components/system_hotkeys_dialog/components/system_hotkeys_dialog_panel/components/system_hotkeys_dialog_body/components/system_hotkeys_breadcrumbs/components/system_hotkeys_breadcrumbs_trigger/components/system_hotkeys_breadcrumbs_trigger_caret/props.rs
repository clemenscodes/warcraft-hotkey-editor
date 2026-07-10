use super::view::SystemHotkeysBreadcrumbsTriggerCaretView;
use dioxus::prelude::*;

/// The caret indicator; `is_open` selects the flipped xor resting look.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerCaretProps {
    pub is_open: bool,
}

impl From<&SystemHotkeysBreadcrumbsTriggerCaretView> for SystemHotkeysBreadcrumbsTriggerCaretProps {
    fn from(view: &SystemHotkeysBreadcrumbsTriggerCaretView) -> Self {
        let SystemHotkeysBreadcrumbsTriggerCaretView { is_open } = view.clone();
        Self { is_open }
    }
}

impl ddd::Props for SystemHotkeysBreadcrumbsTriggerCaretProps {
    type View = SystemHotkeysBreadcrumbsTriggerCaretView;
}
