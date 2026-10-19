use warcraft_keybinds::HotkeyToken;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyPickerCellState {
    Available,
    Current,
    Conflict { display_name: String },
}

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
