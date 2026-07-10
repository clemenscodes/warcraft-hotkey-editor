use dioxus::prelude::*;

/// What the system hotkeys dialog needs: the open signal that drives the shell. Its
/// editors read and write the document through the CustomKeys service, not a
/// threaded signal.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogProps {
    pub system_hotkeys_open: Signal<bool>,
}
