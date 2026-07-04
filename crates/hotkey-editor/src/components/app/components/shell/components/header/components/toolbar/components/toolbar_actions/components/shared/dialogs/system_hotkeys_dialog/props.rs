use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// What the system hotkeys dialog needs: the loaded keys its editors read and
/// write, and the open signal that drives the shell.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub system_hotkeys_open: Signal<bool>,
}
