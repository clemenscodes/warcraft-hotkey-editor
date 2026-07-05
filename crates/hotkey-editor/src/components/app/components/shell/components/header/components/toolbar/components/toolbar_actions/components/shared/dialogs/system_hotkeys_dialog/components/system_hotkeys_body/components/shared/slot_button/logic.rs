use std::collections::HashMap;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, EffectiveBinding, KeyCode, SystemBindingMap};

use super::hooks::SlotButtonModel;
use super::state::SlotButtonState;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;

/// The slot's resolved binding and its conflict picture, derived purely from the
/// stored keys and the section binding map: the idle key label, the current key
/// code, whether it collides, the collision tooltip, and the per-key conflict
/// names the picker needs.
#[derive(Clone, PartialEq, Debug)]
pub(super) struct SlotBinding {
    pub(super) effective_label: String,
    pub(super) current_code: KeyCode,
    pub(super) is_conflict: bool,
    pub(super) conflict_title: String,
    pub(super) picker_conflicts: HashMap<KeyCode, Vec<String>>,
}

impl SlotBinding {
    pub(super) fn resolve(
        custom_keys: Option<&CustomKeys>,
        binding_map: &SystemBindingMap,
        section_id: &str,
        default_hotkey: u32,
        default_modifier: SystemKeybindModifier,
    ) -> Self {
        let effective = EffectiveBinding::resolve_from_file(
            custom_keys,
            section_id,
            default_hotkey,
            default_modifier,
        );
        let effective_key = effective.key();
        let effective_modifier = effective.modifier();
        let collisions = binding_map.collisions_for(section_id, effective_key, effective_modifier);
        let is_conflict = !collisions.is_empty();
        let conflict_title = if is_conflict {
            let names: Vec<String> = collisions
                .iter()
                .map(|resolved| resolved.section_comment().to_string())
                .collect();
            let joined = names.join(", ");
            format!("Also used by {joined}")
        } else {
            String::new()
        };
        let picker_conflicts = binding_map.picker_conflicts(section_id, effective_modifier);
        let effective_label = effective.label();
        Self {
            effective_label,
            current_code: effective_key,
            is_conflict,
            conflict_title,
            picker_conflicts,
        }
    }
}

/// The slot's editing-dependent presentation: its visual state and the key label,
/// which shows an ellipsis while the picker is open.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub(super) struct SlotPresentation {
    pub(super) state: SlotButtonState,
    pub(super) key_label: String,
}

impl SlotPresentation {
    pub(super) fn resolve(is_editing: bool, binding: &SlotBinding) -> Self {
        let key_label = if is_editing {
            String::from("\u{2026}")
        } else {
            binding.effective_label.clone()
        };
        let state = if is_editing {
            SlotButtonState::Editing
        } else if binding.is_conflict {
            SlotButtonState::Conflict
        } else {
            SlotButtonState::Idle
        };
        Self { state, key_label }
    }
}

impl From<&SlotButtonModel> for SystemSlotLabelProps {
    fn from(model: &SlotButtonModel) -> Self {
        let text = model.slot_label.clone();
        let compact = model.compact;
        Self { text, compact }
    }
}

impl From<&SlotButtonModel> for SystemSlotKeyProps {
    fn from(model: &SlotButtonModel) -> Self {
        let label = model.key_label.clone();
        let compact = model.compact;
        let conflict = model.is_conflict;
        Self {
            label,
            compact,
            conflict,
        }
    }
}

impl From<&SlotButtonModel> for SystemKeyPickerDialogProps {
    fn from(model: &SlotButtonModel) -> Self {
        let title = String::from("Pick a hotkey");
        let current_code = model.current_code;
        let conflicts = model.picker_conflicts.clone();
        let open = true;
        let on_pick = model.on_pick;
        let on_close = model.on_close;
        Self {
            title,
            current_code,
            conflicts,
            open,
            on_pick,
            on_close,
        }
    }
}
