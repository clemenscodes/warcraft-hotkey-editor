use super::components::info_dialog_panel::InfoDialogPanelProps;
use super::components::info_dialog_panel::components::info_dialog_body::InfoDialogBodyProps;
use super::components::info_dialog_panel::components::info_dialog_body::components::info_actions::InfoActionsProps;
use super::components::info_dialog_panel::components::info_dialog_body::components::info_content::InfoContentProps;
use super::props::InfoDialogConfig;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The info dialog's own shell, shaped from its config: the open value driving the
/// backdrop, the change handler that writes the open signal, and the bordered panel
/// (its header and scroll-region body). Every dialog owns its shell now — there is no
/// base.
pub(super) struct InfoDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) panel: InfoDialogPanelProps,
}

impl From<&InfoDialogConfig> for InfoDialogShell {
    fn from(props: &InfoDialogConfig) -> Self {
        let mut open_signal = props.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = props.open;
        let title = props.title.to_owned();
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let content = InfoContentProps::from(props);
        let actions = InfoActionsProps::from(props);
        let body = InfoDialogBodyProps { content, actions };
        let panel = InfoDialogPanelProps { header, body };
        Self {
            open,
            on_open_change,
            panel,
        }
    }
}
