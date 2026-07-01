use super::components::inventory_slot::InventorySlotProps;
use super::inventory_cell::InventoryCellProps;
use super::props::InventoryGridProps;
use super::{INVENTORY_COLUMNS, INVENTORY_ROWS, InventoryDragSource, SLOT_FRAME_GOLD};
use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::SystemBindingMap;

/// The grid's shaped setup: the inline `--wc3-slot-frame` variable that feeds every
/// slot's border-image, and the six finished grid positions (filled cell or empty).
pub(super) struct InventoryGridModel {
    pub(super) frame: String,
    pub(super) slots: Vec<InventorySlotProps>,
}

/// Builds the grid's binding map, drag signals, gold-frame variable, and the six
/// slot positions with their shared drag/drop state.
pub(super) fn use_inventory_grid(props: &InventoryGridProps) -> InventoryGridModel {
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let drag_follower = props.drag_follower;
    let dragging_source = use_signal::<Option<InventoryDragSource>>(|| None);
    let drop_target = use_signal::<Option<String>>(|| None);
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    let binding_map_signal: ReadSignal<SystemBindingMap> = binding_map.into();
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    let entries = SystemHotkeysCategory::Inventory.entries();
    let mut slots: Vec<InventorySlotProps> = Vec::new();
    for row in 0..INVENTORY_ROWS {
        for column in 0..INVENTORY_COLUMNS {
            let slot_index = row * INVENTORY_COLUMNS + column;
            let entry_option = entries.get(slot_index).copied();
            let cell = entry_option.map(|entry| {
                let section_id = entry.section_id().to_string();
                let default_hotkey = entry.default_hotkey();
                let default_modifier = entry.default_modifier();
                InventoryCellProps {
                    slot_index,
                    section_id,
                    default_hotkey,
                    default_modifier,
                    loaded_keys,
                    editing_section,
                    dragging_source,
                    drop_target,
                    drag_follower,
                    binding_map: binding_map_signal,
                }
            });
            let slot = InventorySlotProps { cell };
            slots.push(slot);
        }
    }
    InventoryGridModel { frame, slots }
}
