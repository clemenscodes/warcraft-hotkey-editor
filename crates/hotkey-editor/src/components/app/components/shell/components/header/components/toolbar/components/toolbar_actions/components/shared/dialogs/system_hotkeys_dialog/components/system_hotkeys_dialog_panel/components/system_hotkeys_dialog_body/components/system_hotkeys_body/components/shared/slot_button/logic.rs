use super::hooks::SlotButtonModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::SystemSlotProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;

impl From<&SlotButtonModel> for SystemSlotProps {
    fn from(model: &SlotButtonModel) -> Self {
        let state = model.state;
        let slot_label = model.slot_label.clone();
        let key_label = model.key_label.clone();
        let conflict = model.is_conflict;
        let tooltip_text = model.conflict_title.clone();
        let tooltip_placement = TooltipPlacement::Below;
        let dragging = false;
        Self {
            state,
            slot_label,
            key_label,
            conflict,
            tooltip_text,
            tooltip_placement,
            dragging,
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
