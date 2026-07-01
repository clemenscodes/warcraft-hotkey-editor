use dioxus::prelude::*;

use crate::services::overlay_state::OverlayState;

/// The button's shaped view: whether the layout dialog is open (for aria) and the
/// toggle handler.
pub(super) struct GridLayoutButtonModel {
    pub(super) is_open: bool,
    pub(super) on_toggle: EventHandler<MouseEvent>,
}

/// Reads the overlay context and wires the open/close toggle for the global grid
/// layout dialog.
pub(super) fn use_grid_layout_button() -> GridLayoutButtonModel {
    let overlay = use_context::<OverlayState>();
    let mut layout_dialog_open = overlay.layout_dialog_open;
    let is_open = layout_dialog_open();
    let on_toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*layout_dialog_open.read();
        layout_dialog_open.set(next);
    });
    GridLayoutButtonModel { is_open, on_toggle }
}
