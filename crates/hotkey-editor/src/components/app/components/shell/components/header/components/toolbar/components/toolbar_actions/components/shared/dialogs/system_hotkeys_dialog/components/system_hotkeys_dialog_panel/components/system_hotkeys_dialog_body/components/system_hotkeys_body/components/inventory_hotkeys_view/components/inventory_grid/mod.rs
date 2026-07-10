pub mod components;
mod hooks;
mod style;

use crate::services::editor_state::{CursorPoint, HitTestPoint};
use components::inventory_slot::InventorySlot;
use dioxus::prelude::*;
use hooks::use_inventory_grid;
use std::cell::{Cell, RefCell};
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;
use wasm_bindgen::closure::Closure;

pub(super) const DRAG_MOVEMENT_THRESHOLD_PIXELS: f64 = 4.0;
pub(super) const INVENTORY_COLUMNS: usize = 2;
pub(super) const INVENTORY_ROWS: usize = 3;

#[derive(Clone, Copy)]
pub(super) struct DragOrigin {
    pub(super) cursor_horizontal_position: f64,
    pub(super) cursor_vertical_position: f64,
}

/// Latest pointer coords awaiting an animation-frame flush.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct DragMovePoint {
    pub(super) client_horizontal: f64,
    pub(super) client_vertical: f64,
}

impl DragMovePoint {
    /// Applies this pending pointer position: moves the drag follower to it and
    /// hit-tests the element underneath to pick (or clear) the current drop target.
    pub(super) fn flush(
        self,
        mut drop_target: Signal<Option<WarcraftObjectId>>,
        mut drag_follower: Signal<Option<InventoryDragFollower>>,
        section_id: WarcraftObjectId,
    ) {
        let cursor_horizontal_position = self.client_horizontal;
        let cursor_vertical_position = self.client_vertical;
        let current_follower_option = drag_follower.read().clone();
        if let Some(mut current_follower) = current_follower_option {
            current_follower.cursor_horizontal_position = cursor_horizontal_position;
            current_follower.cursor_vertical_position = cursor_vertical_position;
            drag_follower.set(Some(current_follower));
        }
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let cursor_point = CursorPoint::new(cursor_horizontal_position, cursor_vertical_position);
        let hit_test_point = HitTestPoint::from(cursor_point);
        let hit_test_horizontal = hit_test_point.horizontal_position();
        let hit_test_vertical = hit_test_point.vertical_position();
        let elem_under_option = document.element_from_point(hit_test_horizontal, hit_test_vertical);
        let cell_under_option = elem_under_option
            .and_then(|elem| elem.closest(".inventory-filled-slot").ok().flatten());
        let Some(cell_under) = cell_under_option else {
            if drop_target.read().is_some() {
                drop_target.set(None);
            }
            return;
        };
        // The target slot's section is its row-major position among the grid's slots:
        // counting the cell's preceding siblings gives its slot index, which maps to the
        // inventory section rendered there (the same order the grid builds them). No DOM
        // attribute is read.
        let mut slot_index_under: usize = 0;
        let mut previous_sibling_option = cell_under.previous_element_sibling();
        while let Some(previous_sibling) = previous_sibling_option {
            slot_index_under += 1;
            previous_sibling_option = previous_sibling.previous_element_sibling();
        }
        let inventory_entries = SystemHotkeysCategory::Inventory.entries();
        let Some(target_entry) = inventory_entries.get(slot_index_under) else {
            if drop_target.read().is_some() {
                drop_target.set(None);
            }
            return;
        };
        let target_id = target_entry.section_id();
        if target_id == section_id {
            if drop_target.read().is_some() {
                drop_target.set(None);
            }
            return;
        }
        let needs_update = *drop_target.read() != Some(target_id);
        if needs_update {
            drop_target.set(Some(target_id));
        }
    }
}

pub(super) type DragRafClosure = Closure<dyn FnMut(f64)>;

thread_local! {
    pub(super) static SUPPRESS_NEXT_CLICK: Cell<bool> = const { Cell::new(false) };
    pub(super) static DRAG_ORIGIN: Cell<Option<DragOrigin>> = const { Cell::new(None) };
    pub(super) static DID_DRAG_MOVE: Cell<bool> = const { Cell::new(false) };

    /// Latest pointer coords awaiting an animation-frame flush (client x, y).
    pub(super) static LATEST_DRAG_MOVE: Cell<Option<DragMovePoint>> = const { Cell::new(None) };

    /// Handle of the pending requestAnimationFrame, so it can be cancelled.
    pub(super) static DRAG_RAF_HANDLE: Cell<Option<i32>> = const { Cell::new(None) };

    /// The rAF callback closure, kept alive while a frame is pending.
    pub(super) static DRAG_RAF_CLOSURE: RefCell<Option<DragRafClosure>> = const {
        RefCell::new(None)
    };

    /// A begun-but-not-yet-promoted drag, awaiting the first past-threshold move.
    pub(super) static PENDING_INVENTORY_DRAG: RefCell<Option<PendingInventoryDrag>> = const {
        RefCell::new(None)
    };
}

pub(super) struct InventoryDragRaf;

impl InventoryDragRaf {
    /// Cancels any pending animation frame and drops its state, so a stale
    /// frame cannot fire after the drag has ended.
    pub(super) fn cancel() {
        if let Some(handle) = DRAG_RAF_HANDLE.with(|cell| cell.replace(None))
            && let Some(window) = web_sys::window()
        {
            let _ = window.cancel_animation_frame(handle);
        }
        LATEST_DRAG_MOVE.with(|cell| cell.set(None));
        DRAG_RAF_CLOSURE.with(|cell| cell.borrow_mut().take());
    }
}

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

#[derive(Clone, PartialEq, Debug)]
pub struct InventoryDragFollower {
    pub(super) section_id: WarcraftObjectId,
    pub(super) label: String,
    pub(super) click_offset_horizontal: f64,
    pub(super) click_offset_vertical: f64,
    pub(super) cursor_horizontal_position: f64,
    pub(super) cursor_vertical_position: f64,
    pub(super) width: f64,
    pub(super) height: f64,
}

impl InventoryDragFollower {
    pub(crate) fn left(&self) -> f64 {
        self.cursor_horizontal_position - self.click_offset_horizontal
    }

    pub(crate) fn top(&self) -> f64 {
        self.cursor_vertical_position - self.click_offset_vertical
    }

    pub(crate) fn width(&self) -> f64 {
        self.width
    }

    pub(crate) fn height(&self) -> f64 {
        self.height
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InventoryDragSource {
    pub(crate) section_id: WarcraftObjectId,
}

/// A drag that has begun (pointer down on a slot) but not yet crossed the movement
/// threshold, so it is still ambiguous with a click. Held here until the first
/// past-threshold move promotes it into a real drag — capturing the pointer and
/// mounting the follower. Deferring the state change is what keeps a plain click from
/// unmounting the slot content under the cursor: mutating the drag signals on
/// pointerdown detaches the mousedown target, which makes the browser drop the click and
/// swallow the click-to-edit gesture.
pub(super) struct PendingInventoryDrag {
    pub(super) section_id: WarcraftObjectId,
    pub(super) label: String,
    pub(super) click_offset_horizontal: f64,
    pub(super) click_offset_vertical: f64,
    pub(super) width: f64,
    pub(super) height: f64,
    pub(super) cell_element: web_sys::Element,
    pub(super) pointer_id: i32,
}
assert_component!(InventoryGrid);

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
            for slot in model.slots {
                InventorySlot { ..slot }
            }
        }
    }
}
