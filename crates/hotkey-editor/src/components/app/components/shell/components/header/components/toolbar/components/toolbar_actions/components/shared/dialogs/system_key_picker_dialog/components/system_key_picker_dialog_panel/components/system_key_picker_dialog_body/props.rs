use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The system key picker's scroll region input: the raw board values (both keyboard
/// columns plus the pick and Escape handlers) it hands to the shared board host.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogBodyProps {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}
