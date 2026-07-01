use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;

use super::components::system_hotkeys_category_tab::SystemHotkeysCategoryTabProps;
use super::props::SystemHotkeysBreadcrumbsMenuProps;

/// Builds one tab's props per category, marking the active one and whether a
/// separator follows it.
pub(super) fn use_system_hotkeys_breadcrumbs_menu(
    props: &SystemHotkeysBreadcrumbsMenuProps,
) -> Vec<SystemHotkeysCategoryTabProps> {
    let active = *props.active_category.read();
    let category_count = SystemHotkeysCategory::ALL.len();
    SystemHotkeysCategory::ALL
        .iter()
        .copied()
        .enumerate()
        .map(|(index, category)| {
            let is_active = category == active;
            let has_separator = index + 1 < category_count;
            SystemHotkeysCategoryTabProps {
                category,
                is_active,
                has_separator,
                active_category: props.active_category,
                picker_open: props.picker_open,
            }
        })
        .collect()
}
