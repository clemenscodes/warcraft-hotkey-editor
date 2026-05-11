use std::collections::HashMap;

use crate::custom_keys::CustomKeys;
use crate::model::{ColumnIndex, GridCoordinate, RowIndex};

pub const COMMAND_GRID_COLUMNS: u8 = 4;
pub const COMMAND_GRID_ROWS: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridLayout {
    letters: [[char; 4]; 3],
}

impl GridLayout {
    pub const fn qwerty_grid() -> Self {
        Self {
            letters: [
                ['Q', 'W', 'E', 'R'],
                ['A', 'S', 'D', 'F'],
                ['Z', 'X', 'C', 'V'],
            ],
        }
    }

    const fn from_letters(letters: [[char; 4]; 3]) -> Self {
        Self { letters }
    }

    pub fn to_storage_string(self) -> String {
        let mut buffer = String::with_capacity(12);
        for row in self.letters.iter() {
            for letter in row.iter() {
                buffer.push(*letter);
            }
        }
        buffer
    }

    pub fn derived_from(file: &CustomKeys) -> Self {
        let mut histograms: [[HashMap<char, u32>; 4]; 3] =
            std::array::from_fn(|_| std::array::from_fn(|_| HashMap::new()));
        for entry in file.bindings_in_order() {
            let binding = entry.binding();
            let Some(position) = binding.button_position() else {
                continue;
            };
            let Some(hotkey) = binding.hotkey() else {
                continue;
            };
            let hotkey_string = hotkey.to_string();
            let Some(first_character) = hotkey_string.chars().next() else {
                continue;
            };
            let row_index = usize::from(position.row());
            let column_index = usize::from(position.column());
            if row_index >= histograms.len() || column_index >= histograms[row_index].len() {
                continue;
            }
            let upper_letter = first_character.to_ascii_uppercase();
            let cell_histogram = &mut histograms[row_index][column_index];
            let cell_count = cell_histogram.entry(upper_letter).or_insert(0);
            *cell_count += 1;
        }
        let fallback = Self::qwerty_grid();
        let mut derived_letters = [[' '; 4]; 3];
        for row_index in 0..histograms.len() {
            for column_index in 0..histograms[row_index].len() {
                let cell_histogram = &histograms[row_index][column_index];
                let most_common = cell_histogram
                    .iter()
                    .max_by_key(|&(_, count)| *count)
                    .map(|(&letter, _)| letter);
                let Ok(row_u8) = u8::try_from(row_index) else {
                    continue;
                };
                let Ok(column_u8) = u8::try_from(column_index) else {
                    continue;
                };
                let Ok(row) = RowIndex::try_from(row_u8) else {
                    continue;
                };
                let Ok(column) = ColumnIndex::try_from(column_u8) else {
                    continue;
                };
                let chosen = most_common
                    .or_else(|| fallback.letter_at(column, row))
                    .unwrap_or(' ');
                derived_letters[row_index][column_index] = chosen;
            }
        }
        Self::from_letters(derived_letters)
    }

    pub fn letter_at(&self, column: ColumnIndex, row: RowIndex) -> Option<char> {
        let row_index = usize::from(row);
        let column_index = usize::from(column);
        let row_array = self.letters.get(row_index)?;
        row_array.get(column_index).copied()
    }

    pub fn position_for_letter(&self, letter: char) -> Option<GridCoordinate> {
        let target = letter.to_ascii_uppercase();
        for row_index in 0..self.letters.len() {
            for column_index in 0..self.letters[row_index].len() {
                if self.letters[row_index][column_index] == target {
                    let row_u8 = u8::try_from(row_index).ok()?;
                    let column_u8 = u8::try_from(column_index).ok()?;
                    let column = ColumnIndex::try_from(column_u8).ok()?;
                    let row = RowIndex::try_from(row_u8).ok()?;
                    let position = GridCoordinate::new(column, row);
                    return Some(position);
                }
            }
        }
        None
    }

    pub fn swap_cells(
        &mut self,
        source_column: u8,
        source_row: u8,
        target_column: u8,
        target_row: u8,
    ) {
        let source_row_index = usize::from(source_row);
        let source_column_index = usize::from(source_column);
        let target_row_index = usize::from(target_row);
        let target_column_index = usize::from(target_column);
        if source_row_index >= self.letters.len() || target_row_index >= self.letters.len() {
            return;
        }
        if source_column_index >= self.letters[source_row_index].len() {
            return;
        }
        if target_column_index >= self.letters[target_row_index].len() {
            return;
        }
        let source_letter = self.letters[source_row_index][source_column_index];
        let target_letter = self.letters[target_row_index][target_column_index];
        self.letters[source_row_index][source_column_index] = target_letter;
        self.letters[target_row_index][target_column_index] = source_letter;
    }

    pub fn assign_unique(&mut self, column: u8, row: u8, letter: char) {
        let new_letter = letter.to_ascii_uppercase();
        let row_index = usize::from(row);
        let column_index = usize::from(column);
        let target_existing = self
            .letters
            .get(row_index)
            .and_then(|row_slot| row_slot.get(column_index))
            .copied();
        let Some(displaced_letter) = target_existing else {
            return;
        };
        for scan_row in 0..self.letters.len() {
            for scan_column in 0..self.letters[scan_row].len() {
                if scan_row == row_index && scan_column == column_index {
                    continue;
                }
                if self.letters[scan_row][scan_column] == new_letter {
                    self.letters[scan_row][scan_column] = displaced_letter;
                }
            }
        }
        self.letters[row_index][column_index] = new_letter;
    }
}

impl TryFrom<&str> for GridLayout {
    type Error = ();

    fn try_from(raw_value: &str) -> Result<Self, ()> {
        let trimmed_value = raw_value.trim();
        if trimmed_value.len() != 12 {
            return Err(());
        }
        let mut characters = trimmed_value.chars();
        let mut letters = [[' '; 4]; 3];
        for row in letters.iter_mut() {
            for cell in row.iter_mut() {
                let next_character = characters.next().ok_or(())?;
                if !next_character.is_ascii_alphabetic() {
                    return Err(());
                }
                *cell = next_character.to_ascii_uppercase();
            }
        }
        Ok(Self::from_letters(letters))
    }
}
