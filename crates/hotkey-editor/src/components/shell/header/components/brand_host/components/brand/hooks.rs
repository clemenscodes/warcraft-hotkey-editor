use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

/// Wires the brand's return-to-editor click from the navigation context. The brand
/// configures its own home navigation — the header only places it, deciding nothing
/// about where it leads.
pub(super) fn use_brand() -> EventHandler<MouseEvent> {
    let navigation = use_view_navigation();
    EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Editor))
}
