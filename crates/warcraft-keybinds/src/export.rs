use std::collections::HashSet;

use warcraft_api::{WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::{ButtonPosition, CustomKeysFile};

const GRID_COLUMNS: u8 = 4;
const GRID_ROWS: u8 = 3;

/// Serialize a user's loaded file into a complete CustomKeys.txt, blending
/// it over `baseline` (the stock default hotkeys file content) and
/// materializing missing button positions from the game database.
pub fn serialize(loaded_file: &CustomKeysFile, baseline: &str) -> String {
    let mut export_file = CustomKeysFile::from(baseline);
    export_file.overlay(loaded_file);
    export_file.materialize_default_positions();
    export_file.materialize_shop_item_positions();
    export_file.to_file_content()
}

impl CustomKeysFile {
    fn materialize_default_positions(&mut self) {
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id_value = object_id.value();
            let default_button = warcraft_object.default_button_position().map(|position| {
                let column = position.column();
                let row = position.row();
                ButtonPosition::new(column, row)
            });
            let default_research =
                warcraft_object
                    .default_research_button_position()
                    .map(|position| {
                        let column = position.column();
                        let row = position.row();
                        ButtonPosition::new(column, row)
                    });

            match warcraft_object.kind() {
                WarcraftObjectKind::Command => continue,
                WarcraftObjectKind::Ability => {
                    if default_button.is_none() && default_research.is_none() {
                        continue;
                    }
                    let Some(binding) = self.binding_or_default_mut(id_value) else {
                        continue;
                    };
                    if binding.button_position().is_none()
                        && let Some(position_value) = default_button
                    {
                        binding.set_button_position(Some(position_value));
                    }
                    if binding.research_button_position().is_none()
                        && let Some(position_value) = default_research
                    {
                        binding.set_research_button_position(Some(position_value));
                    }
                    if binding.unbutton_position().is_none()
                        && !warcraft_object.is_passive_ability()
                    {
                        let database_off = match warcraft_object.meta() {
                            WarcraftObjectMeta::Ability(ability_meta) => {
                                ability_meta.off_button_position().map(|position| {
                                    let column = position.column();
                                    let row = position.row();
                                    ButtonPosition::new(column, row)
                                })
                            }
                            _ => None,
                        };
                        if let Some(off_position) = database_off {
                            binding.set_unbutton_position(Some(off_position));
                        } else if let Some(button_position) = binding.button_position() {
                            let position_copy = *button_position;
                            binding.set_unbutton_position(Some(position_copy));
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    /// Assign button positions to shop items that have none recorded in the file.
    /// Items sold by a unit's shop that already have an explicit position keep it;
    /// positionless items receive the first free cell within that shop's grid.
    fn materialize_shop_item_positions(&mut self) {
        for (_object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            let sell_items = unit_meta.sell_items();
            if sell_items.is_empty() {
                continue;
            }

            let mut occupied_positions: HashSet<ButtonPosition> = HashSet::new();
            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let existing_position = self
                    .binding(item_id)
                    .and_then(|item_binding| item_binding.button_position())
                    .copied();
                if let Some(position) = existing_position {
                    occupied_positions.insert(position);
                }
            }

            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let has_position = self
                    .binding(item_id)
                    .and_then(|item_binding| item_binding.button_position())
                    .is_some();
                if has_position {
                    continue;
                }
                let Some(free_position) = Self::next_free_grid_cell(&occupied_positions) else {
                    continue;
                };
                occupied_positions.insert(free_position);
                if let Some(item_binding) = self.binding_or_default_mut(item_id) {
                    item_binding.set_button_position(Some(free_position));
                }
            }
        }
    }

    fn next_free_grid_cell(occupied_positions: &HashSet<ButtonPosition>) -> Option<ButtonPosition> {
        for row in 0..GRID_ROWS {
            for column in 0..GRID_COLUMNS {
                let candidate = ButtonPosition::new(column, row);
                if !occupied_positions.contains(&candidate) {
                    return Some(candidate);
                }
            }
        }
        None
    }
}
