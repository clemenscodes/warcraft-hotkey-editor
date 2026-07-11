use crate::services::customkeys::context::use_loaded_keys;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The preview dialog host's shaped domain data: the loaded document to serialize and
/// the shared open signal the toolbar buttons flip.
pub(super) struct PreviewDialogHostModel {
    pub(super) loaded_keys: Signal<Option<CustomKeys>>,
    pub(super) preview_open: Signal<bool>,
}

/// Shapes the preview dialog's inputs from context: the loaded document to serialize
/// and the shared open signal the toolbar buttons flip.
pub(super) fn use_preview_dialog_host() -> PreviewDialogHostModel {
    let loaded_keys = use_loaded_keys();
    let overlay = use_overlay_state();
    let preview_open = overlay.preview_open();
    PreviewDialogHostModel {
        loaded_keys,
        preview_open,
    }
}
