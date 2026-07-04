use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButtonProps;
use crate::components::app::components::shell::components::shared::icons::ICON_PREVIEW;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// Reads the overlay context and shapes the preview toggle: the label and the
/// pressed state flip with the pane's visibility.
pub(super) fn use_preview_button() -> ToolbarButtonProps {
    let overlay = use_overlay_state();
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
