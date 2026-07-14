use crate::components::app::components::shell::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement,
};
use warcraft_keybinds::KeyCode;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum KeyWidth {
    #[default]
    Standard,
    Wide,
}

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

    pub fn into_rows(self) -> Vec<Vec<KeyCell>> {
        self.rows
    }

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
