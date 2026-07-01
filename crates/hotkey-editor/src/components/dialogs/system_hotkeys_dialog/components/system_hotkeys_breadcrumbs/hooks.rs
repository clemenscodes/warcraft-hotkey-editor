use dioxus::prelude::*;

use super::props::SystemHotkeysBreadcrumbsProps;

/// The breadcrumbs' shaped view: the active category, the dropdown open signal and
/// its `"true"`/`"false"` attribute string, the trigger caption, and the toggle
/// handler.
pub(super) struct SystemHotkeysBreadcrumbsModel {
    pub(super) open: Signal<bool>,
    pub(super) open_attr: &'static str,
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
    let open_attr = if is_open { "true" } else { "false" };
    let trigger_label = active_category.read().to_string();
    let on_toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open.read();
        open.set(next);
    });
    SystemHotkeysBreadcrumbsModel {
        open,
        open_attr,
        is_open,
        trigger_label,
        on_toggle,
    }
}
