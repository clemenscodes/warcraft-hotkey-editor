use super::view::SystemHotkeysDialogView;
use dioxus::prelude::*;

/// What the system hotkeys dialog needs: the open signal that drives the shell. Its
/// editors read and write the document through the CustomKeys service, not a
/// threaded signal.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogModel {
    pub system_hotkeys_open: Signal<bool>,
}

impl From<&SystemHotkeysDialogView> for SystemHotkeysDialogModel {
    fn from(view: &SystemHotkeysDialogView) -> Self {
        let SystemHotkeysDialogView {
            system_hotkeys_open,
        } = view.clone();
        Self {
            system_hotkeys_open,
        }
    }
}

impl ddd::Model for SystemHotkeysDialogModel {
    type View = SystemHotkeysDialogView;
}
