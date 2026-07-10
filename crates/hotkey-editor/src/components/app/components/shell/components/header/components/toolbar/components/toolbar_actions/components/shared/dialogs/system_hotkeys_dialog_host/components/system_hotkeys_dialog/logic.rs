use super::hooks::SystemHotkeysDialogModel;
use dioxus::prelude::*;

/// The system-hotkeys dialog's own shell, shaped from its model: the open value
/// driving the backdrop, the change handler that writes the open signal, and the
/// plain header values (title and close handler) the bordered panel draws above its
/// scroll body. The body reads its category tab, editing-section, and inventory drag
/// follower from context, so the shell carries only the header data.
pub(super) struct SystemHotkeysDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
}

impl From<&SystemHotkeysDialogModel> for SystemHotkeysDialogShell {
    fn from(model: &SystemHotkeysDialogModel) -> Self {
        let mut open_signal = model.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = model.open;
        let title = String::from("System Hotkeys");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        Self {
            open,
            on_open_change,
            title,
            on_close,
        }
    }
}
