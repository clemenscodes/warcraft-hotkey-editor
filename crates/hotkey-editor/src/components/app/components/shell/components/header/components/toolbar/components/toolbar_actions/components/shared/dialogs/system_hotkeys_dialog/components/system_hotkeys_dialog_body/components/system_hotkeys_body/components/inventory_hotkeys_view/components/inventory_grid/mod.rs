pub mod components;
mod presentation;
mod style;

use components::inventory_slot::InventorySlot;
use dioxus::prelude::*;
use presentation::use_inventory_grid;
use style::CLASS;
use tw_macro::assert_component;

pub(crate) use presentation::{
    DID_DRAG_MOVE, DRAG_MOVEMENT_THRESHOLD_PIXELS, DRAG_ORIGIN, DRAG_RAF_CLOSURE, DRAG_RAF_HANDLE,
    DragMovePoint, DragOrigin, DragRafClosure, InventoryFilledSlotEntry, InventorySlotEntry,
    LATEST_DRAG_MOVE, PENDING_INVENTORY_DRAG, PendingInventoryDrag, SUPPRESS_NEXT_CLICK,
    cancel_drag_raf,
};
pub use presentation::{InventoryDragFollower, InventoryDragSource};

/// The two-by-three inventory slot grid. Each filled slot is an editable,
/// draggable `InventoryFilledSlot`; the grid seeds the gold-frame CSS variable and holds
/// the drag/drop signals its cells share.
#[component]
pub fn InventoryGrid() -> Element {
    let model = use_inventory_grid();
    rsx! {
        div {
            class: CLASS,
            style: model.frame,
            for InventorySlotEntry { filled } in model.entries {
                InventorySlot {
                    filled,
                }
            }
        }
    }
}

assert_component!(InventoryGrid);
