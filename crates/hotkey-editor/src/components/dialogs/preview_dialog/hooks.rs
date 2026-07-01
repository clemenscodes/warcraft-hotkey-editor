use super::props::PreviewDialogProps;
use dioxus::prelude::*;

/// The preview dialog's shaped view: the serialized CustomKeys.txt the textarea
/// shows. Recomputed whenever the loaded keys change.
pub(super) struct PreviewDialogView {
    pub(super) text: Memo<String>,
}

/// Composes the preview dialog's body data. The one piece of work the body is
/// not allowed to do, the normalize-and-serialize, lives here behind a single
/// flat call.
pub(super) fn use_preview_dialog(props: &PreviewDialogProps) -> PreviewDialogView {
    let loaded_keys = props.loaded_keys;
    let text = use_memo(move || {
        let loaded = loaded_keys.read();
        match loaded.as_ref() {
            Some(file) => file.normalize().to_string(),
            None => String::new(),
        }
    });
    PreviewDialogView { text }
}
