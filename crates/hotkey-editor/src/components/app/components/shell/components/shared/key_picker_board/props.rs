use super::cell::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// What the key picker board renders: the columns of keys the caller laid out, and the
/// handlers a pick or a keyboard dismiss fires. It carries no title, no open flag, and
/// no dialog concern — it is a plain focusable board of keys that renders identically
/// wherever it is placed. Every key is a [`KeyCode`], so `on_pick` reports one back;
/// callers whose field is a narrower type adapt at their own edge.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBoardProps {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    /// Fired when Escape is pressed on the board. Dialog dismissal (backdrop, close
    /// button) is the wrapping dialog's concern; this only reports the keyboard
    /// dismiss the board itself observes.
    pub on_close: EventHandler<()>,
}
