use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::model::grid::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridSlotId};
use crate::services::customkeys::positions::Positions;

/// Returns the set of hotkey letters that occupy more than one cell of the grid,
/// so a tile sharing a letter with another can render the conflict styling.
/// Reads `loaded_keys` during render, so it re-runs whenever the canonical keys
/// change.
pub(crate) fn use_conflicting_hotkeys(
    loaded_keys: Signal<Option<CustomKeys>>,
    slot_ids: &[GridSlotId],
    is_research_grid: bool,
) -> Rc<HashSet<String>> {
    let read_guard = loaded_keys.read();
    let custom_keys_option = read_guard.as_ref();
    let mut counts: HashMap<String, u32> = HashMap::new();
    for row in 0..COMMAND_GRID_ROWS {
        for column in 0..COMMAND_GRID_COLUMNS {
            let cell_with_slot = Positions::cell_for_position(
                slot_ids,
                custom_keys_option,
                is_research_grid,
                column,
                row,
            );
            let letter = cell_with_slot.as_ref().and_then(|occupant| {
                let cell = occupant.cell();
                let token = if is_research_grid {
                    cell.binding_research_hotkey()
                        .or_else(|| cell.binding_hotkey())
                } else {
                    cell.binding_hotkey()
                };
                token.map(|token| token.display_label())
            });
            if let Some(letter_label) = letter {
                *counts.entry(letter_label).or_insert(0) += 1;
            }
        }
    }
    let conflict_set: HashSet<String> = counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(key, _)| key)
        .collect();
    Rc::new(conflict_set)
}
