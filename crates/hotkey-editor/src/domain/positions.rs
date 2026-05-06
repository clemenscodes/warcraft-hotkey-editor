use dioxus::prelude::{ReadableExt, Signal, WritableExt};
use warcraft_api::ButtonPosition;
use warcraft_keybinds::cascade::{current_for, current_for_ability_off, resolved_for, slots_match};
use warcraft_keybinds::{CustomKeys, CustomKeysFile, Hotkey};

use crate::domain::ability_cell::{AbilityCell, BindingHotkey};
use crate::domain::grid_layout::GridLayout;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::object_lookup::ObjectLookup;

pub(crate) struct Positions;

impl Positions {
    pub(crate) fn current_for(
        slot: &GridSlotId,
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<ButtonPosition> {
        current_for(slot, custom_keys, is_research_context)
    }

    pub(crate) fn current_for_ability_off(
        ability_id: &str,
        custom_keys: Option<&CustomKeysFile>,
    ) -> Option<ButtonPosition> {
        current_for_ability_off(ability_id, custom_keys)
    }

    pub(crate) fn resolved_for(
        slot: &GridSlotId,
        candidate_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<ButtonPosition> {
        resolved_for(slot, candidate_slots, custom_keys, is_research_context)
    }

    pub(crate) fn cell_for_position(
        candidate_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
        column: u8,
        row: u8,
    ) -> Option<(GridSlotId, AbilityCell)> {
        for slot in candidate_slots {
            let Some(position) =
                resolved_for(slot, candidate_slots, custom_keys, is_research_context)
            else {
                continue;
            };
            if position.column() == column && position.row() == row {
                let cell = match slot {
                    GridSlotId::Ability(ability_id) => {
                        let binding = custom_keys.and_then(|file| file.binding(ability_id));
                        AbilityCell::for_ability(ability_id, binding)
                    }
                    GridSlotId::AbilityOff(ability_id) => {
                        let binding = custom_keys.and_then(|file| file.binding(ability_id));
                        AbilityCell::for_ability_off(ability_id, binding)
                    }
                    GridSlotId::Command(command_name) => {
                        let binding = custom_keys.and_then(|file| file.command(command_name));
                        AbilityCell::for_command(command_name, binding)
                    }
                };
                return Some((slot.clone(), cell));
            }
        }
        None
    }

    pub(crate) fn assign(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        layout: GridLayout,
        slot: &GridSlotId,
        column: u8,
        row: u8,
        is_research_context: bool,
    ) {
        let Some(letter) = layout.letter_at(column, row) else {
            return;
        };
        let new_position = warcraft_keybinds::ButtonPosition::new(column, row);

        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));
        match slot {
            GridSlotId::Ability(ability_id) => {
                let is_passive = ObjectLookup::is_passive_ability(ability_id);
                if let Some(binding) = file.binding_or_default_mut(ability_id) {
                    if is_research_context {
                        binding.set_research_button_position(Some(new_position));
                        let research_hotkey = Hotkey::from(letter);
                        binding.set_research_hotkey(Some(research_hotkey));
                    } else {
                        binding.set_button_position(Some(new_position));
                        if !is_passive {
                            let ability_hotkey = Hotkey::from(letter);
                            binding.set_hotkey(Some(ability_hotkey));
                        }
                    }
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                if let Some(binding) = file.binding_or_default_mut(ability_id) {
                    binding.set_unbutton_position(Some(new_position));
                    let unhotkey = Hotkey::from(letter);
                    binding.set_unhotkey(Some(unhotkey));
                }
            }
            GridSlotId::Command(command_name) => {
                if let Some(binding) = file.command_or_default_mut(command_name) {
                    binding.set_button_position(Some(new_position));
                    let command_hotkey = Hotkey::from(letter);
                    binding.set_hotkey(Some(command_hotkey));
                    binding.set_unbutton_position(Some(new_position));
                }
            }
        }
    }

    pub(crate) fn move_or_swap(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        request: MoveRequest<'_>,
    ) {
        let read_guard = custom_keys_signal.read();
        let custom_keys = read_guard.as_ref();
        let moving_old_position = resolved_for(
            request.moving_slot,
            request.slot_ids,
            custom_keys,
            request.is_research_context,
        );
        let displaced_pair = Self::cell_for_position(
            request.slot_ids,
            custom_keys,
            request.is_research_context,
            request.target_column,
            request.target_row,
        );
        let off_state_blocks = displaced_pair.is_none()
            && !request.is_research_context
            && request.slot_ids.iter().any(|slot| {
                let GridSlotId::Ability(ability_id) = slot else {
                    return false;
                };
                if ability_id.eq_ignore_ascii_case(request.moving_slot.as_str()) {
                    return false;
                }
                current_for_ability_off(ability_id, custom_keys).is_some_and(|off_pos| {
                    off_pos.column() == request.target_column && off_pos.row() == request.target_row
                })
            });
        let explicit_custom_unbutton = |id: &str| -> Option<ButtonPosition> {
            custom_keys
                .and_then(|file| file.binding(id))
                .and_then(|binding| binding.unbutton_position())
                .map(|position| ButtonPosition::new(position.column(), position.row()))
        };
        let off_state_in_grid = |id: &str| -> bool {
            request.slot_ids.iter().any(
                |s| matches!(s, GridSlotId::AbilityOff(off_id) if off_id.eq_ignore_ascii_case(id)),
            )
        };
        let moving_off_colocated = !request.prevent_co_move
            && match (request.moving_slot, &moving_old_position) {
                (GridSlotId::Ability(id), Some(old_pos)) => {
                    off_state_in_grid(id)
                        && explicit_custom_unbutton(id).is_some_and(|off_pos| {
                            off_pos.column() == old_pos.column() && off_pos.row() == old_pos.row()
                        })
                }
                _ => false,
            };
        let displaced_off_colocated = match &displaced_pair {
            Some((GridSlotId::Ability(id), _)) => {
                off_state_in_grid(id)
                    && explicit_custom_unbutton(id).is_some_and(|off_pos| {
                        off_pos.column() == request.target_column
                            && off_pos.row() == request.target_row
                    })
            }
            _ => false,
        };
        drop(read_guard);

        if off_state_blocks {
            return;
        }

        let displaced_slot_option = displaced_pair.map(|(slot, _cell)| slot);
        if let Some(ref displaced_slot) = displaced_slot_option
            && slots_match(displaced_slot, request.moving_slot)
        {
            return;
        }
        if request.prevent_swap
            && let Some(ref displaced_slot) = displaced_slot_option
            && !displaced_slot
                .as_str()
                .eq_ignore_ascii_case(request.moving_slot.as_str())
        {
            return;
        }

        Self::assign(
            custom_keys_signal,
            request.layout,
            request.moving_slot,
            request.target_column,
            request.target_row,
            request.is_research_context,
        );

        if moving_off_colocated && let GridSlotId::Ability(moving_id) = request.moving_slot {
            Self::assign(
                custom_keys_signal,
                request.layout,
                &GridSlotId::AbilityOff(moving_id.clone()),
                request.target_column,
                request.target_row,
                false,
            );
        }

        if !request.prevent_swap
            && let (Some(displaced_slot), Some(old_position)) =
                (displaced_slot_option, moving_old_position)
        {
            let old_column = old_position.column();
            let old_row = old_position.row();
            Self::assign(
                custom_keys_signal,
                request.layout,
                &displaced_slot,
                old_column,
                old_row,
                request.is_research_context,
            );
            if displaced_off_colocated && let GridSlotId::Ability(displaced_id) = &displaced_slot {
                Self::assign(
                    custom_keys_signal,
                    request.layout,
                    &GridSlotId::AbilityOff(displaced_id.clone()),
                    old_column,
                    old_row,
                    false,
                );
            }
        }
    }

    pub(crate) fn fully_normalize(file: &mut CustomKeysFile) {
        warcraft_keybinds::cascade::fully_normalize(file);
    }

    pub(crate) fn apply_grid_to_all_known_objects(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        layout: GridLayout,
    ) -> usize {
        let mut changed_count: usize = 0;
        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));

        // Round-trip through the canonical facade to get cascade-resolved
        // positions. Reading raw positions from the in-memory file would give
        // pre-cascade values for abilities shared across multiple units.
        let canonical_custom_keys = CustomKeys::from(&*file);
        let canonical_text = canonical_custom_keys.to_text();
        let normalized_file = CustomKeysFile::from(canonical_text);

        let ability_ids: Vec<String> = file
            .bindings_in_order()
            .map(|entry| entry.id().to_string())
            .collect();
        let command_names: Vec<String> = file
            .commands_in_order()
            .map(|entry| entry.name().to_string())
            .collect();

        for ability_id in &ability_ids {
            let is_passive = ObjectLookup::is_passive_ability(ability_id);
            let button_position = if is_passive {
                None
            } else {
                normalized_file
                    .binding(ability_id)
                    .and_then(|binding| binding.button_position())
                    .copied()
            };
            let research_button_position = normalized_file
                .binding(ability_id)
                .and_then(|binding| binding.research_button_position())
                .copied();
            let unbutton_button_position = normalized_file
                .binding(ability_id)
                .and_then(|binding| binding.unbutton_position())
                .copied();
            if button_position.is_none()
                && research_button_position.is_none()
                && unbutton_button_position.is_none()
            {
                continue;
            }
            let Some(binding) = file.binding_or_default_mut(ability_id) else {
                continue;
            };
            if let Some(position) = button_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && BindingHotkey::accepts_grid_letter(binding.hotkey())
            {
                let new_hotkey = Hotkey::from(letter);
                if binding.hotkey() != Some(&new_hotkey) {
                    binding.set_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
            if let Some(position) = research_button_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && BindingHotkey::accepts_grid_letter(binding.research_hotkey())
            {
                let new_hotkey = Hotkey::from(letter);
                if binding.research_hotkey() != Some(&new_hotkey) {
                    binding.set_research_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
            if let Some(position) = unbutton_button_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && BindingHotkey::accepts_grid_letter(binding.unhotkey())
            {
                let new_hotkey = Hotkey::from(letter);
                if binding.unhotkey() != Some(&new_hotkey) {
                    binding.set_unhotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
        }

        for command_name in &command_names {
            let button_position = normalized_file
                .command(command_name)
                .and_then(|binding| binding.button_position())
                .copied();
            let Some(position) = button_position else {
                continue;
            };
            let Some(letter) = layout.letter_at(position.column(), position.row()) else {
                continue;
            };
            let Some(binding) = file.command_or_default_mut(command_name) else {
                continue;
            };
            if BindingHotkey::accepts_grid_letter(binding.hotkey()) {
                let new_hotkey = Hotkey::from(letter);
                if binding.hotkey() != Some(&new_hotkey) {
                    binding.set_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
        }

        changed_count
    }
}

pub(crate) struct MoveRequest<'a> {
    layout: GridLayout,
    slot_ids: &'a [GridSlotId],
    moving_slot: &'a GridSlotId,
    target_column: u8,
    target_row: u8,
    is_research_context: bool,
    prevent_swap: bool,
    prevent_co_move: bool,
}

impl<'a> MoveRequest<'a> {
    pub(crate) fn new(
        layout: GridLayout,
        slot_ids: &'a [GridSlotId],
        moving_slot: &'a GridSlotId,
        target_column: u8,
        target_row: u8,
        is_research_context: bool,
    ) -> Self {
        Self {
            layout,
            slot_ids,
            moving_slot,
            target_column,
            target_row,
            is_research_context,
            prevent_swap: false,
            prevent_co_move: false,
        }
    }

    pub(crate) fn with_prevent_swap(mut self, prevent: bool) -> Self {
        self.prevent_swap = prevent;
        self
    }

    pub(crate) fn with_prevent_co_move(mut self, prevent: bool) -> Self {
        self.prevent_co_move = prevent;
        self
    }
}
