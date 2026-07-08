use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use warcraft_keybinds::KeyCode;

/// How wide a key cap is drawn: a standard single cap, or a wide cap for keys whose
/// label does not fit one (`Space`, `Backspace`, the mouse side buttons). Decided by
/// the caller per cell; the key renders the width it is given.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum KeyWidth {
    #[default]
    Standard,
    Wide,
}

/// The look a board key wears, decided by the caller from the current bindings: a
/// free key, the key currently bound here, or a key already taken by another binding.
/// A conflict carries the fully composed hover message (the caller owns the wording —
/// "Already used by X" versus "Pick to swap with X") and where that message is
/// anchored, so the shared tooltip can place it clear of the board edges.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyCellState {
    Available,
    Current,
    Conflict {
        tooltip: String,
        placement: TooltipPlacement,
        anchor: TooltipAnchor,
    },
}

/// One key on the picker board: the keyboard key it stands for, the cap label, its
/// width, the state it is in, and whether it may be chosen (a conflict the caller
/// forbids picking renders disabled). Built by the caller from the domain; the board
/// only renders it and reports the [`KeyCode`] back when it is picked. A board is just
/// a collection of these — every key is a `KeyCode`, so the board needs no type
/// parameter.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyCell {
    key_code: KeyCode,
    label: String,
    width: KeyWidth,
    state: KeyCellState,
    pickable: bool,
}

impl KeyCell {
    pub fn new(
        key_code: KeyCode,
        label: String,
        width: KeyWidth,
        state: KeyCellState,
        pickable: bool,
    ) -> Self {
        Self {
            key_code,
            label,
            width,
            state,
            pickable,
        }
    }

    pub fn key_code(&self) -> KeyCode {
        self.key_code
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn width(&self) -> KeyWidth {
        self.width
    }

    pub fn state(&self) -> &KeyCellState {
        &self.state
    }

    pub fn pickable(&self) -> bool {
        self.pickable
    }
}

/// One column of the board: its rows of keys. The letter picker supplies a single
/// column; the system keyboard supplies two (the main keyboard beside the numpad),
/// which the board lays out side by side. Letting the caller shape the columns is how
/// two very different keyboard arrangements share one board.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KeyColumn {
    rows: Vec<Vec<KeyCell>>,
}

impl KeyColumn {
    pub fn new(rows: Vec<Vec<KeyCell>>) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[Vec<KeyCell>] {
        &self.rows
    }

    /// The key codes in this column that may be chosen. The board offers these to a
    /// keyboard press: a physical key resolves to a pick only when it names one of
    /// them. A conflict the caller forbade (`pickable == false`) is left out.
    pub fn pickable_codes(&self) -> Vec<KeyCode> {
        let mut codes: Vec<KeyCode> = Vec::new();
        for row in &self.rows {
            for cell in row {
                if cell.pickable() {
                    let code = cell.key_code();
                    codes.push(code);
                }
            }
        }
        codes
    }
}
