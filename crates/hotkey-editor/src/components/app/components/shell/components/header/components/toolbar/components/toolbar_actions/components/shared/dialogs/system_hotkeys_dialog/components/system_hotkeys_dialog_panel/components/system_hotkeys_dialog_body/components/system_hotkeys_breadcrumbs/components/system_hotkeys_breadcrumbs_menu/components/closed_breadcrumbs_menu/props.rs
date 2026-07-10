use super::super::system_hotkeys_category_tab::SystemHotkeysCategoryTabProps;
use dioxus::prelude::*;

/// The closed menu's input: the shaped tab props, each carrying `menu_open = false`
/// so the tabs render their tab-bar look. Carrying the child props as data is passing
/// data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct ClosedBreadcrumbsMenuProps {
    pub tabs: Vec<SystemHotkeysCategoryTabProps>,
}
