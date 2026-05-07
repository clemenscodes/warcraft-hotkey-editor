use dioxus::prelude::*;
use warcraft_keybinds::CustomKeysFile;

use crate::domain::ability_cell::{AbilityCell, BindingHotkey};
use crate::domain::grid_layout::GridLayout;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::hotkey_token::HotkeyToken;
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
        object_id: &'static str,
        is_command: bool,
        new_token: Option<HotkeyToken>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let empty_source = "";
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(empty_source));
        if is_command {
            if let Some(binding) = file.command_or_default_mut(object_id) {
                let existing_levels = binding
                    .hotkey()
                    .map(BindingHotkey::comma_segment_count)
                    .unwrap_or(0);
                let replicated_hotkey =
                    new_token.map(|token| BindingHotkey::replicated_token(token, existing_levels));
                binding.set_hotkey(replicated_hotkey);
            }
        } else if let Some(binding) = file.binding_or_default_mut(object_id) {
            let existing_levels = binding
                .hotkey()
                .map(BindingHotkey::comma_segment_count)
                .unwrap_or(0);
            let replicated_hotkey =
                new_token.map(|token| BindingHotkey::replicated_token(token, existing_levels));
            binding.set_hotkey(replicated_hotkey);
        }
    }

    pub(crate) fn apply_research(
        loaded_keys: &mut Signal<Option<CustomKeysFile>>,
        object_id: &'static str,
        new_token: Option<HotkeyToken>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let empty_source = "";
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(empty_source));
        if let Some(binding) = file.binding_or_default_mut(object_id) {
            let research_levels = binding
                .research_hotkey()
                .map(BindingHotkey::comma_segment_count)
                .unwrap_or(0);
            let replicated_hotkey =
                new_token.map(|token| BindingHotkey::replicated_token(token, research_levels));
            binding.set_research_hotkey(replicated_hotkey);
        }
    }

    /// Off-state hotkey for a toggle ability ("Stop Defend", "Unburrow"). The
    /// `Unhotkey` field in CustomKeys.txt — bound independently of the
    /// on-state `Hotkey`, so a player can press D to defend and F to stop
    /// defending if they want.
    pub(crate) fn apply_unhotkey(
        loaded_keys: &mut Signal<Option<CustomKeysFile>>,
        object_id: &'static str,
        new_token: Option<HotkeyToken>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let empty_source = "";
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(empty_source));
        if let Some(binding) = file.binding_or_default_mut(object_id) {
            let existing_levels = binding
                .unhotkey()
                .map(BindingHotkey::comma_segment_count)
                .unwrap_or(0);
            let replicated_hotkey =
                new_token.map(|token| BindingHotkey::replicated_token(token, existing_levels));
            binding.set_unhotkey(replicated_hotkey);
        }
    }

    pub(crate) fn detect_conflict(
        container_slots: &[GridSlotId],
        target_object_id: &str,
        proposed_token: HotkeyToken,
        custom_keys: Option<&CustomKeysFile>,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<HotkeyConflict> {
        for candidate_slot in container_slots {
            if candidate_slot
                .as_str()
                .eq_ignore_ascii_case(target_object_id)
            {
                continue;
            }
            let candidate_token = Self::effective_token_for(
                candidate_slot,
                container_slots,
                custom_keys,
                layout,
                is_research_context,
            );
            let Some(token_value) = candidate_token else {
                continue;
            };
            if token_value != proposed_token {
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

    fn effective_token_for(
        slot: &GridSlotId,
        container_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<HotkeyToken> {
        let override_token = Self::override_token_for(slot, custom_keys, is_research_context);
        if let Some(token_value) = override_token {
            return Some(token_value);
        }
        let resolved_position =
            Positions::resolved_for(slot, container_slots, custom_keys, is_research_context)?;
        let column_value = resolved_position.column();
        let row_value = resolved_position.row();
        let layout_letter = layout.letter_at(column_value, row_value)?;
        Some(HotkeyToken::from(layout_letter))
    }

    fn override_token_for(
        slot: &GridSlotId,
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<HotkeyToken> {
        let hotkey = match slot {
            GridSlotId::Ability(ability_id) => {
                let binding = custom_keys.and_then(|file| file.binding(ability_id.value()))?;
                if is_research_context {
                    binding.research_hotkey()
                } else {
                    binding.hotkey()
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                let binding = custom_keys.and_then(|file| file.binding(ability_id.value()))?;
                binding.unhotkey()
            }
            GridSlotId::Command(command_name) => {
                let binding = custom_keys.and_then(|file| file.command(command_name.value()))?;
                binding.hotkey()
            }
        };
        hotkey.and_then(BindingHotkey::first_token)
    }

    fn display_name_for(slot: &GridSlotId, custom_keys: Option<&CustomKeysFile>) -> String {
        match slot {
            GridSlotId::Ability(ability_id) => {
                let binding = custom_keys.and_then(|file| file.binding(ability_id.value()));
                let cell = AbilityCell::for_ability(*ability_id, binding);
                cell.cloned_display_name()
            }
            GridSlotId::AbilityOff(ability_id) => {
                let binding = custom_keys.and_then(|file| file.binding(ability_id.value()));
                let cell = AbilityCell::for_ability_off(*ability_id, binding);
                cell.cloned_display_name()
            }
            GridSlotId::Command(command_name) => {
                let binding = custom_keys.and_then(|file| file.command(command_name.value()));
                let cell = AbilityCell::for_command(*command_name, binding);
                cell.cloned_display_name()
            }
        }
    }
}
