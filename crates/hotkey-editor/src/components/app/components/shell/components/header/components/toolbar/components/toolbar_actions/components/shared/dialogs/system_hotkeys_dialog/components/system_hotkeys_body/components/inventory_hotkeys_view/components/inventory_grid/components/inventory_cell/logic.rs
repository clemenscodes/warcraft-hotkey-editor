use super::hooks::InventoryCellModel;
use super::props::InventoryCellProps;
use super::state::InventoryCellState;
use dioxus::prelude::*;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;
use std::collections::HashMap;
use warcraft_keybinds::{EffectiveBinding, KeyCode};

/// The presentational state of one inventory cell, resolved from the live document and
/// binding map: its glow state, drag flag, caption/key labels, conflict tooltip, and
/// picker conflict set. Pure derivation — no signals owned, no handlers.
pub(super) struct InventoryCellView {
    pub(super) state: InventoryCellState,
    pub(super) dragging_attr: &'static str,
    pub(super) slot_label: String,
    pub(super) key_label: String,
    pub(super) conflict_title: String,
    pub(super) is_conflict: bool,
    pub(super) is_editing: bool,
    pub(super) current_code: KeyCode,
    pub(super) picker_conflicts: HashMap<KeyCode, Vec<String>>,
}

impl InventoryCellView {
    pub(super) fn resolve(props: &InventoryCellProps) -> Self {
        let loaded_keys = props.loaded_keys;
        let editing_section = props.editing_section;
        let dragging_source = props.dragging_source;
        let drop_target = props.drop_target;
        let binding_map = props.binding_map;
        let section_id = &props.section_id;
        let default_hotkey = props.default_hotkey;
        let default_modifier = props.default_modifier;
        let slot_index = props.slot_index;
        let read_guard = loaded_keys.read();
        let file_ref = read_guard.as_ref();
        let effective = EffectiveBinding::resolve_from_file(
            file_ref,
            section_id,
            default_hotkey,
            default_modifier,
        );
        drop(read_guard);
        let map_guard = binding_map.read();
        let effective_key = effective.key();
        let effective_modifier = effective.modifier();
        let collisions = map_guard.collisions_for(section_id, effective_key, effective_modifier);
        let is_conflict = !collisions.is_empty();
        let conflict_title = if is_conflict {
            let names: Vec<String> = collisions
                .iter()
                .map(|resolved| resolved.section_comment().to_string())
                .collect();
            format!("Also used by {}", names.join(", "))
        } else {
            String::new()
        };
        let picker_conflicts = map_guard.picker_conflicts(section_id, effective_modifier);
        drop(map_guard);
        let is_editing = editing_section
            .read()
            .as_deref()
            .map(|active| active == section_id.as_str())
            .unwrap_or(false);
        let is_being_dragged = dragging_source
            .read()
            .as_ref()
            .map(|source| source.section_id == *section_id)
            .unwrap_or(false);
        let is_drop_target = drop_target
            .read()
            .as_deref()
            .map(|target| target == section_id.as_str())
            .unwrap_or(false);
        let state = if is_conflict {
            InventoryCellState::Conflict
        } else if is_editing || is_drop_target {
            InventoryCellState::Active
        } else {
            InventoryCellState::Idle
        };
        let dragging_attr = if is_being_dragged { "true" } else { "false" };
        let key_label = if is_editing {
            String::from("…")
        } else {
            effective.label()
        };
        let slot_label = format!("Slot {}", slot_index + 1);
        let current_code = effective.key();
        Self {
            state,
            dragging_attr,
            slot_label,
            key_label,
            conflict_title,
            is_conflict,
            is_editing,
            current_code,
            picker_conflicts,
        }
    }
}

impl From<&InventoryCellModel> for SystemSlotLabelProps {
    fn from(model: &InventoryCellModel) -> Self {
        let text = model.slot_label.clone();
        let compact = false;
        Self { text, compact }
    }
}

impl From<&InventoryCellModel> for SystemSlotKeyProps {
    fn from(model: &InventoryCellModel) -> Self {
        let label = model.key_label.clone();
        let compact = false;
        let conflict = model.is_conflict;
        Self {
            label,
            compact,
            conflict,
        }
    }
}

impl From<&InventoryCellModel> for SystemKeyPickerDialogProps {
    fn from(model: &InventoryCellModel) -> Self {
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
