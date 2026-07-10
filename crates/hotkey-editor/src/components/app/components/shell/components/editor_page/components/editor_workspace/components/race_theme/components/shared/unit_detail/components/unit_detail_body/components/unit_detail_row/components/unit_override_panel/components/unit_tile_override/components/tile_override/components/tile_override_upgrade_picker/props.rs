use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// Guards the upgraded-form position picker: it only exists when the ability has an
/// upgraded form, so its unit id is optional here.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideUpgradePickerProps {
    pub upgrade_unit_id: Option<WarcraftObjectId>,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub upgrade_position_picker_open: Signal<bool>,
}
