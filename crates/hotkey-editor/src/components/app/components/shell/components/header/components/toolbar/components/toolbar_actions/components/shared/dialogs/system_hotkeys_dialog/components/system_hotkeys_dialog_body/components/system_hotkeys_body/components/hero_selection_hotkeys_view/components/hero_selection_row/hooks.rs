use super::props::HeroSelectionRowProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::slot_button::SlotButtonProps;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

/// The row's shaped setup: the gold-frame variable the container reads and the
/// three finished hero-selection slots.
pub(super) struct HeroSelectionRowModel {
    pub(super) frame: String,
    pub(super) slots: Vec<SlotButtonProps>,
}

/// Builds the gold-frame variable and the three slot buttons. Each slot resolves
/// its own binding from the CustomKeys query, so the row builds no binding map.
pub(super) fn use_hero_selection_row(props: &HeroSelectionRowProps) -> HeroSelectionRowModel {
    let editing_section = props.editing_section;
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
            SlotButtonProps {
                slot_label,
                section_id,
                editing_section,
                compact: false,
            }
        })
        .collect();
    HeroSelectionRowModel { frame, slots }
}
