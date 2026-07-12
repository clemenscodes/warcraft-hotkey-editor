use super::view::HotkeyAltPositionPickerDialogView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The off-state position picker: a modal command grid where the off-state button can
/// be dragged to a new cell. The shared editor signals the grid needs are sourced from
/// context by the component's hook, so only the picker's own identity is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyAltPositionPickerDialogModel {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub hotkey_alt_position_picker_open: Signal<bool>,
}

impl From<&HotkeyAltPositionPickerDialogView> for HotkeyAltPositionPickerDialogModel {
    fn from(view: &HotkeyAltPositionPickerDialogView) -> Self {
        let HotkeyAltPositionPickerDialogView {
            object_id,
            display_name,
            picker_slots,
            hotkey_alt_position_picker_open,
        } = view.clone();
        Self {
            object_id,
            display_name,
            picker_slots,
            hotkey_alt_position_picker_open,
        }
    }
}

impl ddd::Model for HotkeyAltPositionPickerDialogModel {
    type View = HotkeyAltPositionPickerDialogView;
}
