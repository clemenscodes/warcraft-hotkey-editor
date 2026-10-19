use dioxus::prelude::*;
use warcraft_api::ControlGroupSlots;
use warcraft_keybinds::WarcraftObjectId;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

#[derive(Clone, PartialEq)]
pub(super) struct SlotButtonEntry {
    pub(super) slot_label: String,
    pub(super) section_id: WarcraftObjectId,
}

pub(super) struct ControlGroupsRowModel {
    pub(super) frame: String,
    pub(super) slots: Vec<SlotButtonEntry>,
}

pub(super) fn use_control_groups_row() -> ControlGroupsRowModel {
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    let slots = ControlGroupSlots::ALL
        .iter()
        .map(|slot| {
            let slot_label = slot.label().to_string();
            let section_id = slot.section_id();
            SlotButtonEntry {
                slot_label,
                section_id,
            }
        })
        .collect();
    ControlGroupsRowModel { frame, slots }
}
