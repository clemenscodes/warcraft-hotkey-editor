use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// One tab's inputs: which category it is, whether it is the active one, whether a
/// separator follows it, and the signals it reads and writes on select.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysCategoryTabProps {
    pub category: SystemHotkeysCategory,
    pub is_active: bool,
    pub has_separator: bool,
    pub active_category: Signal<SystemHotkeysCategory>,
    pub picker_open: Signal<bool>,
}
