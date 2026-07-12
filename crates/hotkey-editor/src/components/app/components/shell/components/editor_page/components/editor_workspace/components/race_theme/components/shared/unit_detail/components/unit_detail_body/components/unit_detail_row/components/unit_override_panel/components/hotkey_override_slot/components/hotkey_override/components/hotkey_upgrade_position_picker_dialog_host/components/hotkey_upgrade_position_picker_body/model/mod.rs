use super::view::HotkeyUpgradePositionPickerBodyView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The upgraded-form position picker scroll body's inputs: the upgraded-form unit id
/// whose button is draggable, and the command-card slots to lay the grid out from. The
/// grid signals the editor needs are sourced from context by this component's builder.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUpgradePositionPickerBodyModel {
    pub upgrade_unit_id: WarcraftObjectId,
    pub picker_slots: Rc<[GridSlotId]>,
}

impl From<&HotkeyUpgradePositionPickerBodyView> for HotkeyUpgradePositionPickerBodyModel {
    fn from(view: &HotkeyUpgradePositionPickerBodyView) -> Self {
        let HotkeyUpgradePositionPickerBodyView {
            upgrade_unit_id,
            picker_slots,
        } = view.clone();
        Self {
            upgrade_unit_id,
            picker_slots,
        }
    }
}

impl ddd::Model for HotkeyUpgradePositionPickerBodyModel {
    type View = HotkeyUpgradePositionPickerBodyView;
}
