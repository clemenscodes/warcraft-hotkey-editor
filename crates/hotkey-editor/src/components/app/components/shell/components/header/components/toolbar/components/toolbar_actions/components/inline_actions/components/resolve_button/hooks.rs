use crate::components::app::components::shell::components::shared::icons::ICON_RESOLVE;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButtonProps;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// The seam: reads the live document from the [`CustomKeysService`](crate::services::customkeys::service::CustomKeysService) and the router
/// from context, then shapes the toolbar button — disabled until a file is loaded,
/// clicking routes to the Resolve page.
pub(super) fn use_resolve_button() -> ToolbarButtonProps {
    let custom_keys_service = use_custom_keys_service();
    let navigation = use_view_navigation();
    let keys = custom_keys_service.keys();
    let disabled_memo = use_memo(move || keys.read().is_none());
    let disabled = disabled_memo();
    let onclick = EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Resolve));
    ToolbarButtonProps {
        icon: ICON_RESOLVE,
        aria_label: "Resolve conflicts",
        disabled,
        onclick,
        ..ToolbarButtonProps::default()
    }
}
