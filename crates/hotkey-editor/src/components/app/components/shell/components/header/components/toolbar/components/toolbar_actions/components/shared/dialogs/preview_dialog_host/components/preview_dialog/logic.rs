use super::hooks::PreviewDialogView;
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
