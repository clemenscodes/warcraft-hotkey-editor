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
