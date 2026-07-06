use super::logic::{SlotBinding, SlotPresentation};
use super::props::SlotButtonProps;
use super::state::SlotButtonState;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::{CustomKeys, KeyCode};

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

/// The slot's editing state and the handlers that drive it: whether this slot's
/// picker is open, the click that opens it, the pick that writes the chosen key
/// through the file and closes, and the close that cancels.
pub(super) struct SlotEditing {
    pub(super) is_editing: bool,
    pub(super) on_click: EventHandler<MouseEvent>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) on_close: EventHandler<()>,
}

fn use_slot_editing(props: &SlotButtonProps) -> SlotEditing {
    let mut loaded_keys = props.loaded_keys;
    let mut editing_section = props.editing_section;
    let lookup_id = props.section_id;
    let is_editing = *editing_section.read() == Some(lookup_id);
    let on_click =
        EventHandler::new(move |_event: MouseEvent| editing_section.set(Some(lookup_id)));
    let on_pick = EventHandler::new(move |code: KeyCode| {
        let mut guard = loaded_keys.write();
        let file = guard.get_or_insert_with(CustomKeys::default);
        file.set_system_hotkey(lookup_id, code);
        drop(guard);
        editing_section.set(None);
    });
    let on_close = EventHandler::new(move |_event: ()| editing_section.set(None));
    SlotEditing {
        is_editing,
        on_click,
        on_pick,
        on_close,
    }
}

/// Resolves the slot's effective binding, detects conflicts, and wires the edit /
/// pick / close handlers. All the work the body may not do lives here.
pub(super) fn use_slot_button(props: &SlotButtonProps) -> SlotButtonModel {
    let editing = use_slot_editing(props);
    let loaded_keys = props.loaded_keys;
    let binding_map = props.binding_map;
    let default_hotkey = props.default_hotkey;
    let default_modifier = props.default_modifier;
    let section_id = props.section_id;
    let slot_label = props.slot_label.clone();
    let compact = props.compact;

    let read_guard = loaded_keys.read();
    let custom_keys = read_guard.as_ref();
    let map_guard = binding_map.read();
    let binding = SlotBinding::resolve(
        custom_keys,
        &map_guard,
        section_id,
        default_hotkey,
        default_modifier,
    );
    drop(map_guard);
    drop(read_guard);

    let presentation = SlotPresentation::resolve(editing.is_editing, &binding);
    let compact_attr = if compact { "true" } else { "false" };
    SlotButtonModel {
        state: presentation.state,
        slot_label,
        compact,
        compact_attr,
        key_label: presentation.key_label,
        conflict_title: binding.conflict_title,
        is_conflict: binding.is_conflict,
        is_editing: editing.is_editing,
        current_code: binding.current_code,
        picker_conflicts: binding.picker_conflicts,
        on_click: editing.on_click,
        on_pick: editing.on_pick,
        on_close: editing.on_close,
    }
}
