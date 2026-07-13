use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// Shaped grid-layout button data: whether the layout dialog is open (for aria) and
/// the toggle handler that opens it. A pure-domain model — the host passes its fields
/// to the button as named fields.
#[derive(Clone, PartialEq)]
pub(super) struct GridLayoutEditorButtonPresentation {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

/// Reads the overlay context and shapes the grid-layout button's data: whether the
/// layout dialog is open (for aria) and the toggle handler that opens it.
pub(super) fn use_grid_layout_editor_button() -> GridLayoutEditorButtonPresentation {
    let overlay = use_overlay_state();
    let mut layout_dialog_open = overlay.layout_dialog_open();
    let is_open = layout_dialog_open();
    let on_toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*layout_dialog_open.read();
        layout_dialog_open.set(next);
    });
    GridLayoutEditorButtonPresentation {
        is_open,
        onclick: on_toggle,
    }
}
