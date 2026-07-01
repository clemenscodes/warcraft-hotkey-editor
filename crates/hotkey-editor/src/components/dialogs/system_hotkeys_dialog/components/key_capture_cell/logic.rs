use super::hooks::KeyCaptureCellModel;
use crate::components::dialogs::system_key_picker_dialog::SystemKeyPickerDialogProps;

impl From<&KeyCaptureCellModel> for SystemKeyPickerDialogProps {
    fn from(model: &KeyCaptureCellModel) -> Self {
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
