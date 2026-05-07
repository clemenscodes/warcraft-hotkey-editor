use dioxus::prelude::*;
use warcraft_keybinds::{AbilityCell, CustomKeys, HotkeyTarget, HotkeyToken};

use crate::grid_layout::GridLayout;
use crate::grid_slot::GridSlotId;

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
        loaded_keys: &mut Signal<Option<CustomKeys>>,
        target: HotkeyTarget,
        new_token: Option<HotkeyToken>,
    ) {
        let mut writable_guard = loaded_keys.write();
        let empty_source = "";
        let file = writable_guard.get_or_insert_with(|| CustomKeys::from(empty_source));
        file.set_hotkey(target, new_token);
    }

    pub(crate) fn detect_conflict(
        container_slots: &[GridSlotId],
        target_object_id: &str,
        proposed_token: HotkeyToken,
        custom_keys: Option<&CustomKeys>,
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

    fn display_name_for(slot: &GridSlotId, custom_keys: Option<&CustomKeys>) -> String {
        match slot {
            GridSlotId::Ability(ability_id) => {
                let bound_id = *ability_id;
                let binding = custom_keys.and_then(|file| file.binding(bound_id));
                let cell = AbilityCell::for_ability(bound_id, binding);
                let name = cell.display_name();
                name.to_string()
            }
            GridSlotId::AbilityOff(ability_id) => {
                let bound_id = *ability_id;
                let binding = custom_keys.and_then(|file| file.binding(bound_id));
                let cell = AbilityCell::for_ability_off(bound_id, binding);
                let name = cell.display_name();
                name.to_string()
            }
            GridSlotId::Command(command_name) => {
                let bound_name = *command_name;
                let command_name_str = bound_name.value();
                let binding = custom_keys.and_then(|file| file.command(command_name_str));
                let cell = AbilityCell::for_command(bound_name, binding);
                let name = cell.display_name();
                name.to_string()
            }
        }
    }
}
