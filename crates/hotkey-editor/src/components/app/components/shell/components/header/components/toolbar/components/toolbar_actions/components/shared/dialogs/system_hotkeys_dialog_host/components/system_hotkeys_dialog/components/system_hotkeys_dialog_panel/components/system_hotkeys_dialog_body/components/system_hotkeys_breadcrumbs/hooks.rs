use super::props::SystemHotkeysBreadcrumbsProps;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The breadcrumbs' shaped view: the active category, the dropdown open signal and
/// its resolved `is_open` flag, the trigger caption, and the toggle handler.
pub(super) struct SystemHotkeysBreadcrumbsModel {
    pub(super) active_category: Signal<SystemHotkeysCategory>,
    pub(super) open: Signal<bool>,
    pub(super) is_open: bool,
    pub(super) trigger_label: String,
    pub(super) on_toggle: EventHandler<MouseEvent>,
}

/// Sets up the mobile dropdown's open state and the trigger caption.
pub(super) fn use_system_hotkeys_breadcrumbs(
    props: &SystemHotkeysBreadcrumbsProps,
) -> SystemHotkeysBreadcrumbsModel {
    let active_category = props.active_category;
    let mut open = use_signal::<bool>(|| false);
    let is_open = open();
    let trigger_label = active_category.read().to_string();
    let on_toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open.read();
        open.set(next);
    });
    SystemHotkeysBreadcrumbsModel {
        active_category,
        open,
        is_open,
        trigger_label,
        on_toggle,
    }
}
