use super::cell::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The published `View` contract mirroring [`KeyPickerBoardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct KeyPickerBoardView {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    /// Fired when Escape is pressed on the board. Dialog dismissal (backdrop, close
    /// button) is the wrapping dialog's concern; this only reports the keyboard
    /// dismiss the board itself observes.
    pub on_close: EventHandler<()>,
}

impl ddd::View for KeyPickerBoardView {}
