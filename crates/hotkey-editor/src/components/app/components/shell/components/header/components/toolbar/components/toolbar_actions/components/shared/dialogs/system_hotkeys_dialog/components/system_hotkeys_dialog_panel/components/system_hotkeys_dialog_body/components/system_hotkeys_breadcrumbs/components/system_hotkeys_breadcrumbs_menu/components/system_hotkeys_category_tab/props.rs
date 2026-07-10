use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// One tab's inputs: which category it is, whether it is the active one, whether a
/// separator follows it, whether it sits in the open popover (`menu_open`, selecting
/// the popover xor tab-bar look), and the mobile-popover open signal it closes on
/// select. It writes the active category from the dialog state context on select.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysCategoryTabProps {
    pub category: SystemHotkeysCategory,
    pub is_active: bool,
    pub has_separator: bool,
    #[props(default = false)]
    pub menu_open: bool,
    pub picker_open: Signal<bool>,
}
