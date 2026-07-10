use super::view::KeyPickerBoardHostView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// What the interactive key picker host renders: the columns of keys the caller laid out
/// and the handlers a pick or a keyboard dismiss fires. Threaded straight to the board it
/// wraps; the host adds only the keyboard listener and focus around it. Every key is a
/// [`KeyCode`], so `on_pick` reports one back.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBoardHostProps {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl From<&KeyPickerBoardHostView> for KeyPickerBoardHostProps {
    fn from(view: &KeyPickerBoardHostView) -> Self {
        let KeyPickerBoardHostView {
            columns,
            on_pick,
            on_close,
        } = view.clone();
        Self {
            columns,
            on_pick,
            on_close,
        }
    }
}

impl ddd::Props for KeyPickerBoardHostProps {
    type View = KeyPickerBoardHostView;
}
