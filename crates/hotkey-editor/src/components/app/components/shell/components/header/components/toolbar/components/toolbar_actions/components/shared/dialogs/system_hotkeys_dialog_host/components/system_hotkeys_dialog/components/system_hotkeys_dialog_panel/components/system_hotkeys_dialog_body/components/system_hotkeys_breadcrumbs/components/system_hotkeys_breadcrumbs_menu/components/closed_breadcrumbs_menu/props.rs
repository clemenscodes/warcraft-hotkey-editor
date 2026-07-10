use super::super::super::hooks::SystemHotkeysCategoryTabDescriptor;
use dioxus::prelude::*;

/// The closed menu's input: one tab descriptor per category, each carrying
/// `menu_open = false` so the tabs render their tab-bar look. Carrying domain
/// descriptors as data is passing data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct ClosedBreadcrumbsMenuProps {
    pub tabs: Vec<SystemHotkeysCategoryTabDescriptor>,
}
