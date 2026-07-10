use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The upgraded-form position picker: a modal command grid where the upgraded-form
/// button can be dragged to a new cell. The shared editor signals the grid needs are
/// sourced from context by the component's hook.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradePositionPickerProps {
    pub upgrade_unit_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub upgrade_position_picker_open: Signal<bool>,
}
