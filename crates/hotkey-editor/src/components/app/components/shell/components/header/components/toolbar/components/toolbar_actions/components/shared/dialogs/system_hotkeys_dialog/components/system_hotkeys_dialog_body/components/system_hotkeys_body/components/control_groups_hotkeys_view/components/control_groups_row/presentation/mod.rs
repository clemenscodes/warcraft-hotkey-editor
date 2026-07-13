use dioxus::prelude::*;
use warcraft_api::ControlGroupSlots;
use warcraft_keybinds::WarcraftObjectId;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

/// A finished control-group slot descriptor: its caption and the section id whose
/// binding the slot resolves. Plain domain values the row hands to each `SlotButton`.
#[derive(Clone, PartialEq)]
pub(super) struct SlotButtonEntry {
    pub(super) slot_label: String,
    pub(super) section_id: WarcraftObjectId,
}

/// The row's shaped setup: the gold-frame variable the container reads and the ten
/// finished control-group slots.
pub(super) struct ControlGroupsRowModel {
    pub(super) frame: String,
    pub(super) slots: Vec<SlotButtonEntry>,
}

/// Builds the gold-frame variable and the ten slot buttons. Each slot resolves its
/// own binding from the CustomKeys query and its editing section from the dialog
/// state context, so the row builds no binding map.
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
