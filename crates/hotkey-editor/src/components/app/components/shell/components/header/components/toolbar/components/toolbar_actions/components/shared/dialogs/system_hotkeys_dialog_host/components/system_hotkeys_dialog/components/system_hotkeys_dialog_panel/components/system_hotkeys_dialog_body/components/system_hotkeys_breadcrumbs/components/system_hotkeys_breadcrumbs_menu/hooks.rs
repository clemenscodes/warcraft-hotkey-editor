use super::components::shared::system_hotkeys_category_tab::SystemHotkeysCategoryTabProps;
use super::props::SystemHotkeysBreadcrumbsMenuProps;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// Builds one tab's props per category, marking the active one and whether a
/// separator follows it.
pub(super) fn use_system_hotkeys_breadcrumbs_menu(
    props: &SystemHotkeysBreadcrumbsMenuProps,
) -> Vec<SystemHotkeysCategoryTabProps> {
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
            SystemHotkeysCategoryTabProps {
                category,
                is_active,
                has_separator,
                menu_open,
                picker_open: props.picker_open,
            }
        })
        .collect()
}
