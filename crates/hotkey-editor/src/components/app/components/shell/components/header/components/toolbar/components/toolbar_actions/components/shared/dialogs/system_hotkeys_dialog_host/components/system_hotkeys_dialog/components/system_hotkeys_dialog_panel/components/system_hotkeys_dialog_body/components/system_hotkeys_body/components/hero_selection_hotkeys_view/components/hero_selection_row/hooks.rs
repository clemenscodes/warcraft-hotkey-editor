use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

/// A finished hero-selection slot descriptor: its caption and the section id whose
/// binding the slot resolves. Plain domain values the row hands to each `SlotButton`.
#[derive(Clone, PartialEq)]
pub(super) struct SlotButtonEntry {
    pub(super) slot_label: String,
    pub(super) section_id: WarcraftObjectId,
}

/// The row's shaped setup: the gold-frame variable the container reads and the
/// three finished hero-selection slots.
pub(super) struct HeroSelectionRowModel {
    pub(super) frame: String,
    pub(super) slots: Vec<SlotButtonEntry>,
}

/// Builds the gold-frame variable and the three slot buttons. Each slot resolves
/// its own binding from the CustomKeys query and its editing section from the dialog
/// state context, so the row builds no binding map.
pub(super) fn use_hero_selection_row() -> HeroSelectionRowModel {
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    let entries = SystemHotkeysCategory::HeroSelection.entries();
    let slots = entries
        .iter()
        .enumerate()
        .map(|(slot_index, entry)| {
            let slot_label = format!("Hero {}", slot_index + 1);
            let section_key = entry.section_id();
            let section_id = section_key;
            SlotButtonEntry {
                slot_label,
                section_id,
            }
        })
        .collect();
    HeroSelectionRowModel { frame, slots }
}
