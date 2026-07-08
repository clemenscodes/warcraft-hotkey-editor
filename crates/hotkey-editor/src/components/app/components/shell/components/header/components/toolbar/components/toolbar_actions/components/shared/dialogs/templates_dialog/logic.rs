use super::components::templates_dialog_body::TemplatesDialogBodyProps;
use super::hooks::TemplatesDialogView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The templates dialog's own shell, shaped from its view: the open value driving
/// the backdrop, the change handler that writes the open signal, the header props,
/// and the scroll-region body props holding the card gallery.
pub(super) struct TemplatesDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) header: DialogHeaderProps,
    pub(super) body: TemplatesDialogBodyProps,
}

impl From<&TemplatesDialogView> for TemplatesDialogShell {
    fn from(view: &TemplatesDialogView) -> Self {
        let mut open_signal = view.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = view.open;
        let title = String::from("Layout Templates");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let gallery = view.gallery.clone();
        let body = TemplatesDialogBodyProps { gallery };
        Self {
            open,
            on_open_change,
            header,
            body,
        }
    }
}
