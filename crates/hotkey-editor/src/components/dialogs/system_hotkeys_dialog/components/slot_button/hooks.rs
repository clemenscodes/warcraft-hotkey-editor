use super::props::SlotButtonProps;
use super::state::SlotButtonState;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::{CustomKeys, EffectiveBinding, KeyCode};

/// Everything the slot's markup needs, already shaped: its visual state and
/// compact flag, the key label and conflict tooltip, whether its picker is open
/// (and the picker's inputs), and the click / pick / close handlers.
pub(super) struct SlotButtonModel {
    pub(super) state: SlotButtonState,
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

/// Resolves the slot's effective binding, detects conflicts, and wires the edit /
/// pick / close handlers. All the work the body may not do lives here.
pub(super) fn use_slot_button(props: &SlotButtonProps) -> SlotButtonModel {
    let mut loaded_keys = props.loaded_keys;
    let mut editing_section = props.editing_section;
    let binding_map = props.binding_map;
    let default_hotkey = props.default_hotkey;
    let default_modifier = props.default_modifier;
    let lookup_id = props.section_id.clone();
    let read_guard = loaded_keys.read();
    let effective = EffectiveBinding::resolve_from_file(
        read_guard.as_ref(),
        &lookup_id,
        default_hotkey,
        default_modifier,
    );
    drop(read_guard);
    let map_guard = binding_map.read();
    let collisions = map_guard.collisions_for(&lookup_id, effective.key(), effective.modifier());
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
    let picker_conflicts = map_guard.picker_conflicts(&lookup_id, effective.modifier());
    drop(map_guard);
    let is_editing = editing_section
        .read()
        .as_deref()
        .map(|active| active == lookup_id.as_str())
        .unwrap_or(false);
    let key_label = if is_editing {
        String::from("…")
    } else {
        effective.label()
    };
    let state = if is_editing {
        SlotButtonState::Editing
    } else if is_conflict {
        SlotButtonState::Conflict
    } else {
        SlotButtonState::Idle
    };
    let compact = props.compact;
    let compact_attr = if compact { "true" } else { "false" };
    let current_code = effective.key();
    let section_id_for_click = lookup_id.clone();
    let section_id_for_pick = lookup_id.clone();
    let on_click = EventHandler::new(move |_event: MouseEvent| {
        editing_section.set(Some(section_id_for_click.clone()))
    });
    let on_pick = EventHandler::new(move |code: KeyCode| {
        let mut guard = loaded_keys.write();
        let file = guard.get_or_insert_with(|| CustomKeys::from(""));
        file.set_system_hotkey(&section_id_for_pick, code);
        drop(guard);
        editing_section.set(None);
    });
    let on_close = EventHandler::new(move |_event: ()| editing_section.set(None));
    SlotButtonModel {
        state,
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
