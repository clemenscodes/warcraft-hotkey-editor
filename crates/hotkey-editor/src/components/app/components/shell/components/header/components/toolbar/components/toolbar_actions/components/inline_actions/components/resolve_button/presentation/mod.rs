use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_RESOLVE;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// The resolve button's shaped data: the fixed icon and label, whether it is disabled (no file
/// loaded yet), and the click handler that navigates to the conflict-resolution view.
pub(super) struct ResolveButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the navigation service and the live document, and shapes the button's data: disabled
/// until a file is loaded, and the click handler that navigates to the conflict-resolution view.
pub(super) fn use_resolve_button() -> ResolveButtonPresentation {
    let navigation = use_view_navigation();
    let custom_keys_service = use_custom_keys_service();
    let keys = custom_keys_service.keys();
    let has_file_memo = use_memo(move || keys.read().is_some());
    let has_file = has_file_memo();
    let disabled = !has_file;
    let onclick = EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Resolve));
    ResolveButtonPresentation {
        icon: ICON_RESOLVE,
        aria_label: ARIA_LABEL,
        disabled,
        onclick,
    }
}
