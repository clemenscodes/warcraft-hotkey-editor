use super::hooks::SlotButtonModel;
use crate::components::app::components::shell::components::shared::tooltip::{TooltipAnchor, TooltipPlacement, TooltipProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;

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

impl From<&SlotButtonModel> for TooltipProps {
    fn from(model: &SlotButtonModel) -> Self {
        let text = model.conflict_title.clone();
        let placement = TooltipPlacement::Below;
        let anchor = TooltipAnchor::Center;
        Self {
            text,
            placement,
            anchor,
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
