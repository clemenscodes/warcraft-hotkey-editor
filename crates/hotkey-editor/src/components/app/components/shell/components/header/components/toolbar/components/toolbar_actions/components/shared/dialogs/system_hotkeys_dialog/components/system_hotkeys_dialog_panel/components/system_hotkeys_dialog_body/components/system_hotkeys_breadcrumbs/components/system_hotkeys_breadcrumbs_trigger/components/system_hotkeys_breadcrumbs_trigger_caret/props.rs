use dioxus::prelude::*;

/// The caret indicator; `is_open` selects the flipped xor resting look.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerCaretProps {
    pub is_open: bool,
}
