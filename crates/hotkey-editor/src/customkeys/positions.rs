use dioxus::prelude::{Signal, WritableExt};
use warcraft_keybinds::{CustomKeysFile, GridCoordinate, GridLayout};

pub(crate) use warcraft_keybinds::MoveRequest;

use crate::ability_cell::AbilityCell;
use crate::grid_slot::GridSlotId;

pub(crate) struct GridCellOccupant {
    slot_id: GridSlotId,
    cell: AbilityCell,
}

impl GridCellOccupant {
    pub(crate) fn slot_id(&self) -> GridSlotId {
        self.slot_id
    }

    pub(crate) fn cell(&self) -> &AbilityCell {
        &self.cell
    }
}

pub(crate) struct Positions;

impl Positions {
    pub(crate) fn current_for(
        slot: &GridSlotId,
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<GridCoordinate> {
        let file = custom_keys?;
        file.position_for_slot(slot, is_research_context)
    }

    pub(crate) fn current_for_ability_off(
        ability_id: &str,
        custom_keys: Option<&CustomKeysFile>,
    ) -> Option<GridCoordinate> {
        let file = custom_keys?;
        let binding = file.binding(ability_id)?;
        binding.unbutton_position().copied()
    }

    pub(crate) fn cell_for_position(
        candidate_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
        column: u8,
        row: u8,
    ) -> Option<GridCellOccupant> {
        let file = custom_keys?;
        let slot_id = file.slot_at_position(candidate_slots, is_research_context, column, row)?;
        let cell = match slot_id {
            GridSlotId::Ability(ability_id) => {
                let binding = file.binding(ability_id.value());
                AbilityCell::for_ability(ability_id, binding)
            }
            GridSlotId::AbilityOff(ability_id) => {
                let binding = file.binding(ability_id.value());
                AbilityCell::for_ability_off(ability_id, binding)
            }
            GridSlotId::Command(command_name) => {
                let binding = file.command(command_name.value());
                AbilityCell::for_command(command_name, binding)
            }
        };
        let occupant = GridCellOccupant { slot_id, cell };
        Some(occupant)
    }

    pub(crate) fn move_or_swap(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        request: MoveRequest<'_>,
    ) {
        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));
        file.move_slot(&request);
    }

    pub(crate) fn apply_grid_to_all_known_objects(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        layout: GridLayout,
    ) -> usize {
        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));
        file.apply_grid_to_all_bindings(layout)
    }
}
