use dioxus::prelude::*;
use warcraft_keybinds::SystemBindingMap;

use super::props::InventoryGridProps;
use super::{InventoryDragSource, SLOT_FRAME_GOLD};

/// The grid's shaped setup: the inline `--wc3-slot-frame` variable that feeds every
/// slot's border-image, the resolved binding map the cells read, and the drag/drop
/// signals the cells share.
pub(super) struct InventoryGridModel {
    pub(super) frame: String,
    pub(super) binding_map: Memo<SystemBindingMap>,
    pub(super) dragging_source: Signal<Option<InventoryDragSource>>,
    pub(super) drop_target: Signal<Option<String>>,
}

/// Builds the grid's binding map and drag signals and the gold-frame variable.
pub(super) fn use_inventory_grid(props: &InventoryGridProps) -> InventoryGridModel {
    let loaded_keys = props.loaded_keys;
    let dragging_source = use_signal::<Option<InventoryDragSource>>(|| None);
    let drop_target = use_signal::<Option<String>>(|| None);
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    InventoryGridModel {
        frame,
        binding_map,
        dragging_source,
        drop_target,
    }
}
