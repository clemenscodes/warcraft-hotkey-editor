use super::super::shared::system_hotkeys_category_tab::SystemHotkeysCategoryTabProps;
use dioxus::prelude::*;

/// The open menu's input: the shaped tab props, each carrying `menu_open = true` so
/// the tabs render their popover look. Carrying the child props as data is passing
/// data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct OpenBreadcrumbsMenuProps {
    pub tabs: Vec<SystemHotkeysCategoryTabProps>,
}
