use super::props::SystemHotkeysCategoryTabProps;
use super::state::SystemHotkeysCategoryTabState;
use dioxus::prelude::*;

/// The tab's shaped view: its state, caption, active flag (for aria), whether a
/// separator follows, and the select handler.
pub(super) struct SystemHotkeysCategoryTabModel {
    pub(super) state: SystemHotkeysCategoryTabState,
    pub(super) label: String,
    pub(super) is_active: bool,
    pub(super) has_separator: bool,
    pub(super) on_click: EventHandler<MouseEvent>,
}

/// Selecting a tab sets it active and closes the mobile popover.
pub(super) fn use_system_hotkeys_category_tab(
    props: &SystemHotkeysCategoryTabProps,
) -> SystemHotkeysCategoryTabModel {
    let category = props.category;
    let is_active = props.is_active;
    let has_separator = props.has_separator;
    let mut active_category = props.active_category;
    let mut picker_open = props.picker_open;
    let state = if is_active {
        SystemHotkeysCategoryTabState::Active
    } else {
        SystemHotkeysCategoryTabState::Inactive
    };
    let label = category.to_string();
    let on_click = EventHandler::new(move |_event: MouseEvent| {
        active_category.set(category);
        picker_open.set(false);
    });
    SystemHotkeysCategoryTabModel {
        state,
        label,
        is_active,
        has_separator,
        on_click,
    }
}
