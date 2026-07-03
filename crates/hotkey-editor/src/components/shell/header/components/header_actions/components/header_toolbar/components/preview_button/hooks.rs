use crate::components::shell::header::components::header_actions::components::header_toolbar::components::shared::toolbar_button::ToolbarButtonProps;
use crate::components::shared::icons::ICON_PREVIEW;
use crate::services::overlay_state::OverlayState;
use dioxus::prelude::*;

/// Reads the overlay context and shapes the preview toggle: the label and the
/// pressed state flip with the pane's visibility.
pub(super) fn use_preview_button() -> ToolbarButtonProps {
    let overlay = use_context::<OverlayState>();
    let mut preview_open = overlay.preview_open;
    let visible = *preview_open.read();
    let aria_label = if visible { "Hide preview" } else { "Preview" };
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*preview_open.read();
        preview_open.set(next);
    });
    ToolbarButtonProps {
        icon: ICON_PREVIEW,
        aria_label,
        aria_pressed: Some(visible),
        onclick,
        ..ToolbarButtonProps::default()
    }
}
