use super::hooks::InventoryCellModel;
use crate::components::dialogs::system_hotkeys_dialog::components::system_slot_key::SystemSlotKeyProps;
use crate::components::dialogs::system_hotkeys_dialog::components::system_slot_label::SystemSlotLabelProps;
use crate::components::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;

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
