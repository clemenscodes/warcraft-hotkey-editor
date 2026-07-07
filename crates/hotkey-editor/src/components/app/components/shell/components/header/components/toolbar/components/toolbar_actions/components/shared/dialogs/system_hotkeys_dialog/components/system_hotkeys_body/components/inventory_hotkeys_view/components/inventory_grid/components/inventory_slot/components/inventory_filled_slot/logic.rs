use super::hooks::InventoryFilledSlotModel;
use super::props::InventoryFilledSlotProps;
use super::state::InventoryFilledSlotState;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;
use crate::services::customkeys::queries::slot_binding_query::SlotBindingView;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

/// The presentational state of one inventory cell: its glow state, drag flag,
/// caption/key labels, conflict tooltip, and picker conflict set. The binding half
/// comes from the CustomKeys query; the drag/editing half from the shared UI
/// signals. Pure derivation — no signals owned, no handlers.
pub(super) struct InventoryFilledSlotView {
    pub(super) state: InventoryFilledSlotState,
    pub(super) dragging_attr: &'static str,
    pub(super) slot_label: String,
    pub(super) key_label: String,
    pub(super) conflict_title: String,
    pub(super) is_conflict: bool,
    pub(super) is_editing: bool,
    pub(super) current_code: KeyCode,
    pub(super) picker_conflicts: HashMap<KeyCode, Vec<String>>,
}

/// The inputs that shape an [`InventoryFilledSlotView`]: the slot's props (its UI
/// drag / editing signals) and the resolved binding from the CustomKeys query.
pub(super) struct InventoryFilledSlotInputs<'a> {
    pub(super) props: &'a InventoryFilledSlotProps,
    pub(super) binding: &'a SlotBindingView,
}

impl From<InventoryFilledSlotInputs<'_>> for InventoryFilledSlotView {
    fn from(inputs: InventoryFilledSlotInputs<'_>) -> Self {
        let InventoryFilledSlotInputs { props, binding } = inputs;
        let editing_section = props.editing_section;
        let dragging_source = props.dragging_source;
        let drop_target = props.drop_target;
        let section_id = props.section_id;
        let slot_index = props.slot_index;
        let is_conflict = binding.is_conflict();
        let conflict_title = if is_conflict {
            let joined_names = binding.colliding_names().join(", ");
            format!("Also used by {joined_names}")
        } else {
            String::new()
        };
        let picker_conflicts = binding.picker_conflicts().clone();
        let is_editing = *editing_section.read() == Some(section_id);
        let is_being_dragged = dragging_source
            .read()
            .as_ref()
            .map(|source| source.section_id == section_id)
            .unwrap_or(false);
        let is_drop_target = *drop_target.read() == Some(section_id);
        let state = if is_conflict {
            InventoryFilledSlotState::Conflict
        } else if is_editing || is_drop_target {
            InventoryFilledSlotState::Active
        } else {
            InventoryFilledSlotState::Idle
        };
        let dragging_attr = if is_being_dragged { "true" } else { "false" };
        let key_label = if is_editing {
            String::from("…")
        } else {
            binding.effective_label().to_string()
        };
        let slot_label = format!("Slot {}", slot_index + 1);
        let current_code = binding.current_code();
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

impl From<&InventoryFilledSlotModel> for SystemSlotLabelProps {
    fn from(model: &InventoryFilledSlotModel) -> Self {
        let text = model.slot_label.clone();
        let compact = false;
        Self { text, compact }
    }
}

impl From<&InventoryFilledSlotModel> for SystemSlotKeyProps {
    fn from(model: &InventoryFilledSlotModel) -> Self {
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

impl From<&InventoryFilledSlotModel> for SystemKeyPickerDialogProps {
    fn from(model: &InventoryFilledSlotModel) -> Self {
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
