use super::props::SlotButtonProps;
use super::state::SlotButtonState;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

/// Everything the slot's markup needs, already shaped: its visual state and
/// compact flag, the key label and conflict tooltip, whether its picker is open
/// (and the picker's inputs), and the click / pick / close handlers.
pub(super) struct SlotButtonModel {
    pub(super) state: SlotButtonState,
    pub(super) slot_label: String,
    pub(super) compact: bool,
    pub(super) compact_attr: &'static str,
    pub(super) key_label: String,
    pub(super) conflict_title: String,
    pub(super) is_conflict: bool,
    pub(super) is_editing: bool,
    pub(super) current_code: KeyCode,
    pub(super) picker_conflicts: HashMap<KeyCode, Vec<String>>,
    pub(super) on_click: EventHandler<MouseEvent>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) on_close: EventHandler<()>,
}

/// Reads the slot's resolved binding + conflicts from the CustomKeys query, picks
/// the editing-dependent presentation, and wires the edit / pick / close handlers.
/// The pick routes through the service's `set_system_hotkey` command, never
/// mutating the aggregate inline.
pub(super) fn use_slot_button(props: &SlotButtonProps) -> SlotButtonModel {
    let custom_keys_service = use_custom_keys_service();
    let mut editing_section = props.editing_section;
    let lookup_id = props.section_id;
    let slot_label = props.slot_label.clone();
    let compact = props.compact;

    let binding = custom_keys_service.slot_binding(lookup_id);
    let is_conflict = binding.is_conflict();
    let conflict_title = if is_conflict {
        let joined_names = binding.colliding_names().join(", ");
        format!("Also used by {joined_names}")
    } else {
        String::new()
    };
    let is_editing = *editing_section.read() == Some(lookup_id);
    let key_label = if is_editing {
        String::from("\u{2026}")
    } else {
        binding.effective_label().to_string()
    };
    let state = if is_editing {
        SlotButtonState::Editing
    } else if is_conflict {
        SlotButtonState::Conflict
    } else {
        SlotButtonState::Idle
    };
    let compact_attr = if compact { "true" } else { "false" };
    let current_code = binding.current_code();
    let picker_conflicts = binding.picker_conflicts().clone();

    let on_click =
        EventHandler::new(move |_event: MouseEvent| editing_section.set(Some(lookup_id)));
    let on_pick = EventHandler::new(move |code: KeyCode| {
        custom_keys_service.set_system_hotkey(lookup_id, code);
        editing_section.set(None);
    });
    let on_close = EventHandler::new(move |_event: ()| editing_section.set(None));
    SlotButtonModel {
        state,
        slot_label,
        compact,
        compact_attr,
        key_label,
        conflict_title,
        is_conflict,
        is_editing,
        current_code,
        picker_conflicts,
        on_click,
        on_pick,
        on_close,
    }
}
