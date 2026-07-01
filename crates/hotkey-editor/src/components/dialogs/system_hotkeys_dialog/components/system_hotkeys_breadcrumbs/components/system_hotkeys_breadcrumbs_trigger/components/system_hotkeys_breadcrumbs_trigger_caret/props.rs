use dioxus::prelude::*;

/// The caret indicator; `open` drives the flip when the dropdown is expanded.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerCaretProps {
    pub open: &'static str,
}
