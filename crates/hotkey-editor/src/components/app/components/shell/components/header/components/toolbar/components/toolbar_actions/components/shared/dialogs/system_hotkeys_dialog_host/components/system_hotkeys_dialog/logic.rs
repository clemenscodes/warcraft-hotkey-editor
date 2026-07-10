use super::components::system_hotkeys_dialog_panel::SystemHotkeysDialogPanelProps;
use super::hooks::SystemHotkeysDialogModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The system-hotkeys dialog's own shell, shaped from its model: the open value
/// driving the backdrop, the change handler that writes the open signal, and the
/// bordered panel (its header row above the scroll body). The body reads its category
/// tab, editing-section, and inventory drag follower from context, so the panel carries
/// only the header.
pub(super) struct SystemHotkeysDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) panel: SystemHotkeysDialogPanelProps,
}

impl From<&SystemHotkeysDialogModel> for SystemHotkeysDialogShell {
    fn from(model: &SystemHotkeysDialogModel) -> Self {
        let mut open_signal = model.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = model.open;
        let title = String::from("System Hotkeys");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let panel = SystemHotkeysDialogPanelProps { header };
        Self {
            open,
            on_open_change,
            panel,
        }
    }
}
