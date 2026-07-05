pub mod components;
mod hooks;
mod props;
mod style;

use crate::assert_component;
pub use components::inventory_cell::InventoryCell;
use components::inventory_slot::InventorySlot;
use dioxus::prelude::*;
use hooks::use_inventory_grid;
pub use props::InventoryGridProps;
use std::cell::{Cell, RefCell};
use style::CLASS;
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
    pub(super) section_id: String,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InventoryDragSource {
    pub(crate) section_id: String,
}
assert_component!(InventoryGrid);

/// The two-by-three inventory slot grid. Each filled slot is an editable,
/// draggable `InventoryCell`; the grid seeds the gold-frame CSS variable and holds
/// the drag/drop signals its cells share.
#[component]
pub fn InventoryGrid(props: InventoryGridProps) -> Element {
    let model = use_inventory_grid(&props);
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
