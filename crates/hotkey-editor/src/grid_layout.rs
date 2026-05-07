pub(crate) use warcraft_keybinds::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct EditingCell {
    column: u8,
    row: u8,
}

impl EditingCell {
    pub(crate) fn new(column: u8, row: u8) -> Self {
        Self { column, row }
    }

    pub(crate) fn column(&self) -> u8 {
        self.column
    }

    pub(crate) fn row(&self) -> u8 {
        self.row
    }
}
