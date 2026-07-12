use crate::components::app::components::shell::components::shared::icons::ICON_PREVIEW;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The preview toggle's shaped view: the toolbar button's icon, label, pressed aria state,
/// and click handler, plus the dialog's open state and change handler. Body-scroll lock is
/// owned once by `WarcraftDialog`, so this trigger only flips the shared signal.
pub(super) struct PreviewButtonModel {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_pressed: Option<bool>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the overlay context and shapes the preview toggle and its dialog wiring: the
/// label and pressed state flip with the pane's visibility, the click toggles it, and
/// `on_open_change` mirrors the headless dialog's own close (escape, outside click) back
/// to the shared signal.
pub(super) fn use_preview_button() -> PreviewButtonModel {
    let overlay = use_overlay_state();
    let preview_open = overlay.preview_open();
    let open = *preview_open.read();
    let aria_label = if open { "Hide preview" } else { "Preview" };
    let mut toggle_open = preview_open;
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*toggle_open.read();
        toggle_open.set(next);
    });
    let mut change_open = preview_open;
    let on_open_change = Callback::new(move |is_open| change_open.set(is_open));
    PreviewButtonModel {
        icon: ICON_PREVIEW,
        aria_label,
        aria_pressed: Some(open),
        onclick,
        open,
        on_open_change,
    }
}
