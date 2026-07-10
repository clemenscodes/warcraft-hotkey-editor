use super::components::key_chip::KeyChipProps;
use super::hooks::KeyCaptureModel;
use crate::components::app::components::shell::components::shared::tooltip::{TooltipAnchor, TooltipPlacement, TooltipProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;

impl From<&KeyCaptureModel> for KeyChipProps {
    fn from(model: &KeyCaptureModel) -> Self {
        let conflict = model.is_conflict;
        let label = model.key_label.clone();
        let onclick = model.on_click;
        let tooltip = TooltipProps::from(model);
        Self {
            conflict,
            label,
            onclick,
            tooltip,
        }
    }
}

impl From<&KeyCaptureModel> for TooltipProps {
    fn from(model: &KeyCaptureModel) -> Self {
        let text = model.conflict_title.clone();
        let placement = TooltipPlacement::Above;
        let anchor = TooltipAnchor::Center;
        Self {
            text,
            placement,
            anchor,
        }
    }
}

impl From<&KeyCaptureModel> for SystemKeyPickerDialogProps {
    fn from(model: &KeyCaptureModel) -> Self {
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
