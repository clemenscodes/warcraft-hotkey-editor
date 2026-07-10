use crate::components::app::components::shell::components::shared::icons::ICON_PREVIEW;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The preview toggle's shaped view: its icon, label, pressed aria state, and click
/// handler.
pub(super) struct PreviewButtonModel {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_pressed: Option<bool>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the overlay context and shapes the preview toggle: the label and the
/// pressed state flip with the pane's visibility.
pub(super) fn use_preview_button() -> PreviewButtonModel {
    let overlay = use_overlay_state();
    let mut preview_open = overlay.preview_open();
    let visible = *preview_open.read();
    let aria_label = if visible { "Hide preview" } else { "Preview" };
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*preview_open.read();
        preview_open.set(next);
    });
    PreviewButtonModel {
        icon: ICON_PREVIEW,
        aria_label,
        aria_pressed: Some(visible),
        onclick,
    }
}
