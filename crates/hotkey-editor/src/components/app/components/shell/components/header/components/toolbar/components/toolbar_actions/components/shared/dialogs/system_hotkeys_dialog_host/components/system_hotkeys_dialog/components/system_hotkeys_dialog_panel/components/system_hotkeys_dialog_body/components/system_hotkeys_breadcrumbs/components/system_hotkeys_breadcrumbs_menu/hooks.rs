use super::props::SystemHotkeysBreadcrumbsMenuProps;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// One category tab's raw inputs: which category it is, whether it is the active one,
/// whether a separator follows it, whether the popover is open (selecting the tab's
/// look), and the mobile-popover open signal it closes on select.
#[derive(Clone, PartialEq)]
pub(super) struct SystemHotkeysCategoryTabDescriptor {
    pub(super) category: SystemHotkeysCategory,
    pub(super) is_active: bool,
    pub(super) has_separator: bool,
    pub(super) menu_open: bool,
    pub(super) picker_open: Signal<bool>,
}

/// Builds one descriptor per category, marking the active one and whether a
/// separator follows it.
pub(super) fn use_system_hotkeys_breadcrumbs_menu(
    props: &SystemHotkeysBreadcrumbsMenuProps,
) -> Vec<SystemHotkeysCategoryTabDescriptor> {
    let active = *props.active_category.read();
    let menu_open = props.is_open;
    let category_count = SystemHotkeysCategory::ALL.len();
    SystemHotkeysCategory::ALL
        .iter()
        .copied()
        .enumerate()
        .map(|(index, category)| {
            let is_active = category == active;
            let has_separator = index + 1 < category_count;
            let picker_open = props.picker_open;
            SystemHotkeysCategoryTabDescriptor {
                category,
                is_active,
                has_separator,
                menu_open,
                picker_open,
            }
        })
        .collect()
}
