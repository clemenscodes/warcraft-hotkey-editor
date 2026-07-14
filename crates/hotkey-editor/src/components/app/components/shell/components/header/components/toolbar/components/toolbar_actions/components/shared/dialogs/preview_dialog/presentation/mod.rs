use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The preview dialog's shaped wiring: whether the preview is open and the change handler
/// mirroring the headless dialog's own close back to the shared overlay signal.
pub(super) struct PreviewDialogPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the shared preview open signal from the overlay context — the one the toolbar
/// preview button and the burger drawer both flip — and shapes it into a signal-free open
/// value plus a change handler mirroring the headless dialog's own close.
pub(super) fn use_preview_dialog() -> PreviewDialogPresentation {
    let overlay = use_overlay_state();
    let preview_open = overlay.preview_open();
    let open = *preview_open.read();
    let mut change_open = preview_open;
    let on_open_change = Callback::new(move |is_open| change_open.set(is_open));
    PreviewDialogPresentation {
        open,
        on_open_change,
    }
}
