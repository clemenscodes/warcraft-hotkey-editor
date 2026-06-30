use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// What the preview dialog needs: the loaded keys to serialize and the open
/// signal that drives the shell.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub preview_open: Signal<bool>,
}
