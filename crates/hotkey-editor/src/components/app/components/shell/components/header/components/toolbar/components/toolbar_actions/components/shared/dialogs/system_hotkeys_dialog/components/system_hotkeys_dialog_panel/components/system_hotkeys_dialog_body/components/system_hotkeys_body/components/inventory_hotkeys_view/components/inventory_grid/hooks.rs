use super::components::inventory_slot::InventorySlotProps;
use super::components::inventory_slot::components::inventory_filled_slot::InventoryFilledSlotProps;
use super::{INVENTORY_COLUMNS, INVENTORY_ROWS, InventoryDragSource, SLOT_FRAME_GOLD};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The grid's shaped setup: the inline `--wc3-slot-frame` variable that feeds every
/// slot's border-image, and the six finished grid positions (filled cell or empty).
pub(super) struct InventoryGridModel {
    pub(super) frame: String,
    pub(super) slots: Vec<InventorySlotProps>,
}

/// Builds the grid's drag signals, gold-frame variable, and the six slot positions
/// with their shared drag/drop state. The drag follower comes from the dialog state
/// context; each filled slot resolves its own binding from the CustomKeys query, so
/// the grid builds no binding map.
pub(super) fn use_inventory_grid() -> InventoryGridModel {
    let dialog_state = use_system_hotkeys_dialog_state();
    let drag_follower = dialog_state.drag_follower();
    let dragging_source = use_signal::<Option<InventoryDragSource>>(|| None);
    let drop_target = use_signal::<Option<WarcraftObjectId>>(|| None);
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    let entries = SystemHotkeysCategory::Inventory.entries();
    let mut slots: Vec<InventorySlotProps> = Vec::new();
    for row in 0..INVENTORY_ROWS {
        for column in 0..INVENTORY_COLUMNS {
            let slot_index = row * INVENTORY_COLUMNS + column;
            let entry_option = entries.get(slot_index).copied();
            let cell = entry_option.map(|entry| {
                let section_key = entry.section_id();
                let section_id = section_key;
                InventoryFilledSlotProps {
                    slot_index,
                    section_id,
                    dragging_source,
                    drop_target,
                    drag_follower,
                }
            });
            let slot = InventorySlotProps { cell };
            slots.push(slot);
        }
    }
    InventoryGridModel { frame, slots }
}
