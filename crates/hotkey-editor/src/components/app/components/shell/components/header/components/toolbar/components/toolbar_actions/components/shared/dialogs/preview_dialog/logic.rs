use super::components::preview_dialog_panel::PreviewDialogPanelProps;
use super::components::preview_dialog_panel::components::preview_dialog_body::PreviewDialogBodyProps;
use super::hooks::PreviewDialogView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The preview dialog's own shell, shaped from its view: the open value driving the
/// backdrop, the change handler that writes the open signal, and the bordered panel
/// (its header and scroll-region body). Every dialog owns its shell now — there is no
/// base.
pub(super) struct PreviewDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) panel: PreviewDialogPanelProps,
}

impl From<&PreviewDialogView> for PreviewDialogShell {
    fn from(view: &PreviewDialogView) -> Self {
        let mut open_signal = view.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = view.open;
        let title = String::from("Preview");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let textarea = view.textarea.clone();
        let body = PreviewDialogBodyProps { textarea };
        let panel = PreviewDialogPanelProps { header, body };
        Self {
            open,
            on_open_change,
            panel,
        }
    }
}
