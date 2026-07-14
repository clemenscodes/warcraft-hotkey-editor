use super::model::KeyCaptureModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

/// Everything the chip's markup needs: its state, the key label and conflict
/// tooltip, whether its picker is open (and the picker's inputs), and the edit /
/// pick / close handlers.
pub(super) struct KeyCapturePresentation {
    pub(super) conflict: bool,
    pub(super) label: String,
    pub(super) tooltip_text: String,
    pub(super) tooltip_placement: TooltipPlacement,
    pub(super) is_editing: bool,
    pub(super) title: String,
    pub(super) current_code: KeyCode,
    pub(super) conflicts: HashMap<KeyCode, Vec<String>>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) on_close: EventHandler<()>,
}

/// Reads the chip's resolved binding + conflicts from the CustomKeys query and
/// wires the edit / pick / close handlers. The pick routes through the service's
/// `set_system_hotkey` command, never mutating the aggregate inline.
pub(super) fn use_key_capture(props: &KeyCaptureModel) -> KeyCapturePresentation {
    let custom_keys_service = use_custom_keys_service();
    let dialog_state = use_system_hotkeys_dialog_state();
    let mut editing_section = dialog_state.editing_section();
    let lookup_id = props.section_id;

    let binding = custom_keys_service.slot_binding(lookup_id);
    let is_conflict = binding.is_conflict();
    let conflict_title = if is_conflict {
        let joined_names = binding.colliding_names().join(", ");
        format!("Also used by {joined_names}")
    } else {
        String::new()
    };
    let is_editing = *editing_section.read() == Some(lookup_id);
    let key_label = binding.effective_label().to_string();
    let current_code = binding.current_code();
    let picker_conflicts = binding.picker_conflicts().clone();

    let on_click =
        EventHandler::new(move |_event: MouseEvent| editing_section.set(Some(lookup_id)));
    let on_pick = EventHandler::new(move |code: KeyCode| {
        custom_keys_service.set_system_hotkey(lookup_id, code);
        editing_section.set(None);
    });
    let on_close = EventHandler::new(move |_event: ()| editing_section.set(None));
    let tooltip_placement = TooltipPlacement::Above;
    let title = String::from("Pick a hotkey");
    let open = true;
    KeyCapturePresentation {
        conflict: is_conflict,
        label: key_label,
        tooltip_text: conflict_title,
        tooltip_placement,
        is_editing,
        title,
        current_code,
        conflicts: picker_conflicts,
        open,
        onclick: on_click,
        on_pick,
        on_close,
    }
}

impl ddd::Presentation for KeyCapturePresentation {
    type Model = KeyCaptureModel;
}
