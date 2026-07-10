use super::components::help_dialog_panel::HelpDialogPanelProps;
use super::components::help_dialog_panel::components::help_dialog_body::HelpDialogBodyProps;
use super::components::help_dialog_panel::components::help_dialog_body::components::help_body::HelpBodyProps;
use super::components::help_dialog_panel::components::help_dialog_body::components::help_dismiss::HelpDismissProps;
use super::data::HELP_CONTENT;
use super::props::HelpDialogProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The help dialog's own shell, shaped directly from its props: the open value driving
/// the backdrop, the change handler that writes the open signal, and the bordered panel
/// (its header and scroll-region body). Every dialog owns its shell now — there is no
/// base.
pub(super) struct HelpDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) panel: HelpDialogPanelProps,
}

impl From<&HelpDialogProps> for HelpDialogShell {
    fn from(props: &HelpDialogProps) -> Self {
        let mut open_signal = props.help_open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = props.help_open;
        let title = String::from("How to use this editor");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let help_body_props = HelpBodyProps {
            content: HELP_CONTENT,
        };
        let dismiss = HelpDismissProps::from(props);
        let body = HelpDialogBodyProps {
            body: help_body_props,
            dismiss,
        };
        let panel = HelpDialogPanelProps { header, body };
        Self {
            open,
            on_open_change,
            panel,
        }
    }
}
