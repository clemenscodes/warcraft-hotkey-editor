use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use super::components::upgrade_position_picker::UpgradePositionPickerProps;

/// Guards the upgraded-form position picker: it only exists when the ability has an
/// upgraded form, so its unit id is optional here.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideUpgradePickerProps {
    pub upgrade_unit_id: Option<WarcraftObjectId>,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub upgrade_position_picker_open: Signal<bool>,
}

impl From<&TileOverrideUpgradePickerProps> for UpgradePositionPickerProps {
    /// Only called after the body guards that `upgrade_unit_id` is present and the
    /// picker is visible, so the unwrap holds.
    fn from(props: &TileOverrideUpgradePickerProps) -> Self {
        let upgrade_unit_id = props
            .upgrade_unit_id
            .expect("guarded to Some before conversion");
        let display_name = props.display_name.clone();
        let picker_slots = props.picker_slots.clone();
        let upgrade_position_picker_open = props.upgrade_position_picker_open;
        Self {
            upgrade_unit_id,
            display_name,
            picker_slots,
            upgrade_position_picker_open,
        }
    }
}
