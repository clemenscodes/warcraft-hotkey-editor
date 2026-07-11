use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The published `View` contract mirroring [`UpgradePositionPickerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UpgradePositionPickerView {
    pub upgrade_unit_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub upgrade_position_picker_open: Signal<bool>,
}

impl ddd::View for UpgradePositionPickerView {}
