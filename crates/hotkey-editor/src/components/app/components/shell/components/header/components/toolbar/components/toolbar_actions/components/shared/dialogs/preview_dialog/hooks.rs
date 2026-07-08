use super::components::preview_dialog_body::components::preview_textarea::PreviewTextareaProps;
use super::props::PreviewDialogProps;
use dioxus::prelude::*;

/// The preview dialog's shaped view: the open signal driving the shell and the
/// serialized-text textarea props, recomputed whenever the loaded keys change.
pub(super) struct PreviewDialogView {
    pub(super) open: Signal<bool>,
    pub(super) textarea: PreviewTextareaProps,
}

/// Composes the preview dialog's body data. The one piece of work the body is not
/// allowed to do, the serialize, lives here behind a single flat call. It serializes
/// the loaded document as-is — never re-normalizing (R5): the stored document is
/// already normalized (R2) and the loaded-keys signal mirrors localStorage (R1), so
/// this is the stored text, and re-normalizing here would only mask un-normalized
/// state instead of surfacing it.
pub(super) fn use_preview_dialog(props: &PreviewDialogProps) -> PreviewDialogView {
    let open = props.preview_open;
    let loaded_keys = props.loaded_keys;
    let text = use_memo(move || {
        let loaded = loaded_keys.read();
        match loaded.as_ref() {
            Some(file) => file.to_string(),
            None => String::new(),
        }
    });
    let textarea = PreviewTextareaProps { text: text.into() };
    PreviewDialogView { open, textarea }
}
