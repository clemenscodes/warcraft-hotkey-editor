use super::components::system_hotkeys_dialog_body::SystemHotkeysDialogBodyProps;
use super::hooks::SystemHotkeysDialogModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The system-hotkeys dialog's own shell, shaped from its model: the open value
/// driving the backdrop, the change handler that writes the open signal, the header
/// props, and the scroll-region body props carrying the category tab, the shared
/// editing-section signal, and the inventory drag follower.
pub(super) struct SystemHotkeysDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) header: DialogHeaderProps,
    pub(super) body: SystemHotkeysDialogBodyProps,
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
        let body = SystemHotkeysDialogBodyProps {
            active_category: model.active_category,
            editing_section: model.editing_section,
            drag_follower: model.drag_follower,
        };
        Self {
            open,
            on_open_change,
            header,
            body,
        }
    }
}
