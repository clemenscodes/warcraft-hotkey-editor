use super::{
    INVENTORY_COLUMNS, INVENTORY_ROWS, InventoryDragSource, InventoryFilledSlotEntry,
    InventorySlotEntry, SLOT_FRAME_GOLD,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The grid's shaped setup: the inline `--wc3-slot-frame` variable that feeds every
/// slot's border-image, and the six finished grid positions (filled cell or empty).
pub(super) struct InventoryGridModel {
    pub(super) frame: String,
    pub(super) entries: Vec<InventorySlotEntry>,
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
    let inventory_entries = SystemHotkeysCategory::Inventory.entries();
    let mut entries: Vec<InventorySlotEntry> = Vec::new();
    for row in 0..INVENTORY_ROWS {
        for column in 0..INVENTORY_COLUMNS {
            let slot_index = row * INVENTORY_COLUMNS + column;
            let entry_option = inventory_entries.get(slot_index).copied();
            let filled = entry_option.map(|entry| {
                let section_id = entry.section_id();
                InventoryFilledSlotEntry {
                    slot_index,
                    section_id,
                    dragging_source,
                    drop_target,
                    drag_follower,
                }
            });
            let slot_entry = InventorySlotEntry { filled };
            entries.push(slot_entry);
        }
    }
    InventoryGridModel { frame, entries }
}
