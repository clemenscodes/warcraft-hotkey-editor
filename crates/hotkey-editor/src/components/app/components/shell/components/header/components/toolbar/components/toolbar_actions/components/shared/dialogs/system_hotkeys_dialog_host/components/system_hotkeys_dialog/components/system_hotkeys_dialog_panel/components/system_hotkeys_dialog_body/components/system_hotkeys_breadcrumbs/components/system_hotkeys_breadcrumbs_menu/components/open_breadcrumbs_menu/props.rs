use super::super::super::hooks::SystemHotkeysCategoryTabDescriptor;
use dioxus::prelude::*;

/// The open menu's input: one tab descriptor per category, each carrying
/// `menu_open = true` so the tabs render their popover look. Carrying domain
/// descriptors as data is passing data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct OpenBreadcrumbsMenuProps {
    pub tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}
