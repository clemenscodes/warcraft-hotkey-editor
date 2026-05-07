use dioxus::prelude::*;
use warcraft_keybinds::CustomKeysFile;

use crate::domain::ability_cell::AbilityCell;
use crate::domain::grid_layout::GridLayout;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::hotkey_token::HotkeyToken;

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
        file.set_hotkey_for_slot(object_id, is_command, new_token);
    }

    pub(crate) fn apply_research(
        loaded_keys: &mut Signal<Option<CustomKeysFile>>,
        object_id: &'static str,
        new_token: Option<HotkeyToken>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let empty_source = "";
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(empty_source));
        file.set_research_hotkey_for_slot(object_id, new_token);
    }

    /// Off-state hotkey for a toggle ability ("Stop Defend", "Unburrow"). The
    /// `Unhotkey` field in CustomKeys.txt — bound independently of the
    /// on-state `Hotkey`.
    pub(crate) fn apply_unhotkey(
        loaded_keys: &mut Signal<Option<CustomKeysFile>>,
        object_id: &'static str,
        new_token: Option<HotkeyToken>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let empty_source = "";
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(empty_source));
        file.set_unhotkey_for_slot(object_id, new_token);
    }

    pub(crate) fn detect_conflict(
        container_slots: &[GridSlotId],
        target_object_id: &str,
        proposed_token: HotkeyToken,
        custom_keys: Option<&CustomKeysFile>,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<HotkeyConflict> {
        let file = custom_keys?;
        let conflicting_slot = file.find_hotkey_conflict(
            container_slots,
            target_object_id,
            proposed_token,
            layout,
            is_research_context,
        )?;
        let display_name = Self::display_name_for(&conflicting_slot, custom_keys);
        let conflict_record = HotkeyConflict {
            conflicting_display_name: display_name,
        };
        Some(conflict_record)
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
