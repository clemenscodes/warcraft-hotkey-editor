use dioxus::prelude::*;

/// The preview dialog's own shell, shaped from its view: the open value driving the
/// backdrop, the change handler that writes the open signal, and the panel's own domain
/// values — its header title, the close handler, and the serialized text. Every dialog
/// owns its shell now — there is no base.
pub(super) struct PreviewDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
    pub(super) text: ReadSignal<String>,
}

impl From<&PreviewDialogView> for PreviewDialogShell {
    fn from(view: &PreviewDialogView) -> Self {
        let mut open_signal = view.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = view.open;
        let title = String::from("Preview");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let text = view.text;
        Self {
            open,
            on_open_change,
            title,
            on_close,
            text,
        }
    }
}
use super::model::PreviewDialogModel;

/// The preview dialog's shaped view: the open signal driving the shell and the
/// serialized text, recomputed whenever the loaded keys change.
pub(super) struct PreviewDialogView {
    pub(super) open: Signal<bool>,
    pub(super) text: ReadSignal<String>,
}

/// Composes the preview dialog's body data. The one piece of work the body is not
/// allowed to do, the serialize, lives here behind a single flat call. It serializes
/// the loaded document as-is — never re-normalizing (R5): the stored document is
/// already normalized (R2) and the loaded-keys signal mirrors localStorage (R1), so
/// this is the stored text, and re-normalizing here would only mask un-normalized
/// state instead of surfacing it.
pub(super) fn use_preview_dialog(props: &PreviewDialogModel) -> PreviewDialogView {
    let open = props.preview_open;
    let loaded_keys = props.loaded_keys;
    let text = use_memo(move || {
        let loaded = loaded_keys.read();
        match loaded.as_ref() {
            Some(file) => file.to_string(),
            None => String::new(),
        }
    });
    let text = text.into();
    PreviewDialogView { open, text }
}

impl ddd::Presentation for PreviewDialogView {
    type Model = PreviewDialogModel;
}
