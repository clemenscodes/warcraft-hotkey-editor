use super::components::resolve_button::ResolveButtonProps;
use crate::services::customkeys::service::CustomKeysService;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The seam: reads the live document from the [`CustomKeysService`] and the router
/// from context, then shapes the resolve button's props — disabled until a file is
/// loaded, click routes to the Resolve page.
pub(super) fn use_resolve_button() -> ResolveButtonProps {
    let custom_keys_service = use_context::<CustomKeysService>();
    let navigation = use_context::<ViewNavigationContext>();
    let keys = custom_keys_service.keys();
    let disabled = keys.read().is_none();
    let onclick = EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Resolve));
    ResolveButtonProps { disabled, onclick }
}
