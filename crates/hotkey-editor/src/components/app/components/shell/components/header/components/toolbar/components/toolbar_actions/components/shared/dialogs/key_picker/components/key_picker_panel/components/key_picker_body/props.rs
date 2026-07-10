use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The key picker's scroll region input: the columns of keys the shared board holds,
/// the pick handler it fires, and the keyboard-dismiss handler it observes.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBodyProps {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}
