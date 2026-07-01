use super::props::ResolveButtonProps;
use crate::components::shared::icons::ICON_RESOLVE;
use crate::components::shared::toolbar_button::ToolbarButtonProps;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// Shapes the resolve button: disabled until a file is loaded, clicking navigates
/// to the Resolve page where the cascade plan is previewed and applied.
pub(super) fn use_resolve_button(props: &ResolveButtonProps) -> ToolbarButtonProps {
    let loaded_keys = props.loaded_keys;
    let navigation = use_context::<ViewNavigationContext>();
    let disabled = loaded_keys.read().is_none();
    let onclick = EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Resolve));
    ToolbarButtonProps {
        icon: ICON_RESOLVE,
        aria_label: "Resolve conflicts",
        disabled,
        data_action: Some("view-resolve"),
        onclick,
        ..ToolbarButtonProps::default()
    }
}
