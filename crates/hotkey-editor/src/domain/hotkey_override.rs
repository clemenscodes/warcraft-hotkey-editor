use dioxus::prelude::*;
use warcraft_keybinds::CustomKeysFile;

use crate::domain::ability_cell::{AbilityCell, BindingHotkey};
use crate::domain::grid_layout::GridLayout;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::positions::Positions;

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct HotkeyConflict {
    conflicting_display_name: String,
}

impl HotkeyConflict {
    pub(crate) fn conflicting_display_name(&self) -> &str {
        &self.conflicting_display_name
    }
}

pub(crate) struct HotkeyOverride;

impl HotkeyOverride {
    pub(crate) fn apply(
        loaded_keys: &mut Signal<Option<CustomKeysFile>>,
        object_id: &str,
        is_command: bool,
        new_letter: Option<String>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));
        if is_command {
            let binding = file.command_or_default_mut(object_id);
            let existing_levels = binding
                .hotkey()
                .map(BindingHotkey::comma_segment_count)
                .unwrap_or(0);
            let new_value =
                new_letter.map(|letter| BindingHotkey::replicated_letter(&letter, existing_levels));
            binding.set_hotkey(new_value);
        } else {
            let binding = file.binding_or_default_mut(object_id);
            let existing_levels = binding
                .hotkey()
                .map(BindingHotkey::comma_segment_count)
                .unwrap_or(0);
            let replicated_value =
                new_letter.map(|letter| BindingHotkey::replicated_letter(&letter, existing_levels));
            binding.set_hotkey(replicated_value);
        }
    }

    pub(crate) fn apply_research(
        loaded_keys: &mut Signal<Option<CustomKeysFile>>,
        object_id: &str,
        new_letter: Option<String>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));
        let binding = file.binding_or_default_mut(object_id);
        let research_levels = binding
            .research_hotkey()
            .map(BindingHotkey::comma_segment_count)
            .unwrap_or(0);
        let replicated_value = new_letter
            .as_deref()
            .map(|letter| BindingHotkey::replicated_letter(letter, research_levels));
        binding.set_research_hotkey(replicated_value);
    }

    pub(crate) fn detect_conflict(
        container_slots: &[GridSlotId],
        target_object_id: &str,
        proposed_letter: char,
        custom_keys: Option<&CustomKeysFile>,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<HotkeyConflict> {
        let proposed_upper = proposed_letter.to_ascii_uppercase();
        for candidate_slot in container_slots {
            if candidate_slot
                .as_str()
                .eq_ignore_ascii_case(target_object_id)
            {
                continue;
            }
            let candidate_letter = Self::effective_letter_for(
                candidate_slot,
                container_slots,
                custom_keys,
                layout,
                is_research_context,
            );
            let Some(letter_value) = candidate_letter else {
                continue;
            };
            if letter_value != proposed_upper {
                continue;
            }
            let display_name = Self::display_name_for(candidate_slot, custom_keys);
            let conflict_record = HotkeyConflict {
                conflicting_display_name: display_name,
            };
            return Some(conflict_record);
        }
        None
    }

    fn effective_letter_for(
        slot: &GridSlotId,
        container_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<char> {
        let override_letter = Self::override_letter_for(slot, custom_keys, is_research_context);
        if let Some(letter_value) = override_letter {
            return Some(letter_value);
        }
        let resolved_position =
            Positions::resolved_for(slot, container_slots, custom_keys, is_research_context)?;
        layout.letter_at(resolved_position.column(), resolved_position.row())
    }

    fn override_letter_for(
        slot: &GridSlotId,
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<char> {
        let raw_hotkey_string = match slot {
            GridSlotId::Ability(ability_id) => {
                let binding = custom_keys.and_then(|file| file.binding(ability_id))?;
                if is_research_context {
                    binding.research_hotkey()
                } else {
                    binding.hotkey()
                }
            }
            GridSlotId::Command(command_name) => {
                let binding = custom_keys.and_then(|file| file.command(command_name))?;
                binding.hotkey()
            }
        };
        let first_letter_string = raw_hotkey_string.and_then(BindingHotkey::first_letter)?;
        first_letter_string.chars().next()
    }

    fn display_name_for(slot: &GridSlotId, custom_keys: Option<&CustomKeysFile>) -> String {
        match slot {
            GridSlotId::Ability(ability_id) => {
                let binding = custom_keys.and_then(|file| file.binding(ability_id));
                let cell = AbilityCell::for_ability(ability_id, binding);
                cell.cloned_display_name()
            }
            GridSlotId::Command(command_name) => {
                let binding = custom_keys.and_then(|file| file.command(command_name));
                let cell = AbilityCell::for_command(command_name, binding);
                cell.cloned_display_name()
            }
        }
    }
}
