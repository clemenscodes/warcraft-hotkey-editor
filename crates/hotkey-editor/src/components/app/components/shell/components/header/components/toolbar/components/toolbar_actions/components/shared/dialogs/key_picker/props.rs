use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// The look a board key wears, decided by the caller from the current bindings:
/// a free key, the key currently bound here, or a key already taken by another
/// binding (which names the holder so the key can explain the clash).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyPickerCellState {
    Available,
    Current,
    Conflict { display_name: String },
}

/// One key on the picker board: the hotkey it offers and the state that hotkey is
/// in. Built by the caller from the domain; the picker only renders it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyPickerCell {
    token: HotkeyToken,
    state: KeyPickerCellState,
}

impl KeyPickerCell {
    pub fn new(token: HotkeyToken, state: KeyPickerCellState) -> Self {
        Self { token, state }
    }

    pub fn token(&self) -> HotkeyToken {
        self.token
    }

    pub fn state(&self) -> &KeyPickerCellState {
        &self.state
    }
}

/// What the key picker needs: the title the shell shows, the board of keys to
/// offer, the open flag that mounts it, and the handlers for a pick and a close.
/// `allow_conflict_pick` lets a conflicting key stay pickable (the layout editor
/// swaps the two), which the spell picker leaves off so a clash is flagged but
/// cannot be chosen.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerProps {
    #[props(into)]
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    #[props(default = false)]
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}
