use std::collections::HashMap;

use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// What the system key picker needs: the title the shell shows, the key currently
/// bound (highlighted), the map of already-taken keys to the hotkeys that hold
/// them, the open flag that mounts it, and the pick/close handlers.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogProps {
    #[props(into)]
    pub title: String,
    pub current_code: KeyCode,
    pub conflicts: HashMap<KeyCode, Vec<String>>,
    pub open: bool,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}
