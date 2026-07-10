use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The menu's inputs: the active category (read + written by its tabs), the shared
/// open signal (tabs close it on select), and the open attribute string.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsMenuProps {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub picker_open: Signal<bool>,
    pub open: &'static str,
}
