use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use crate::services::drag_state::{CursorPoint, HitTestPoint};
use dioxus::prelude::*;
use std::cell::{Cell, RefCell};
use warcraft_api::{InventorySlots, SystemHotkeysSlot};
use warcraft_keybinds::WarcraftObjectId;
use wasm_bindgen::closure::Closure;

pub(crate) const DRAG_MOVEMENT_THRESHOLD_PIXELS: f64 = 4.0;

#[derive(Clone, Copy)]
pub(crate) struct DragOrigin {
    pub(crate) cursor_horizontal_position: f64,
    pub(crate) cursor_vertical_position: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DragMovePoint {
    pub(crate) client_horizontal: f64,
    pub(crate) client_vertical: f64,
}

impl DragMovePoint {
    pub(crate) fn flush(
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
        let mut slot_index_under: usize = 0;
        let mut previous_sibling_option = cell_under.previous_element_sibling();
        while let Some(previous_sibling) = previous_sibling_option {
            slot_index_under += 1;
            previous_sibling_option = previous_sibling.previous_element_sibling();
        }
        let Some(target_slot) = InventorySlots::ALL.iter().nth(slot_index_under) else {
            if drop_target.read().is_some() {
                drop_target.set(None);
            }
            return;
        };
        let target_id = target_slot.section_id();
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

pub(crate) type DragRafClosure = Closure<dyn FnMut(f64)>;

thread_local! {
    pub(crate) static SUPPRESS_NEXT_CLICK: Cell<bool> = const { Cell::new(false) };
    pub(crate) static DRAG_ORIGIN: Cell<Option<DragOrigin>> = const { Cell::new(None) };
    pub(crate) static DID_DRAG_MOVE: Cell<bool> = const { Cell::new(false) };

    pub(crate) static LATEST_DRAG_MOVE: Cell<Option<DragMovePoint>> = const { Cell::new(None) };

    pub(crate) static DRAG_RAF_HANDLE: Cell<Option<i32>> = const { Cell::new(None) };

    pub(crate) static DRAG_RAF_CLOSURE: RefCell<Option<DragRafClosure>> = const {
        RefCell::new(None)
    };

    pub(crate) static PENDING_INVENTORY_DRAG: RefCell<Option<PendingInventoryDrag>> = const {
        RefCell::new(None)
    };
}

pub(crate) fn cancel_drag_raf() {
    if let Some(handle) = DRAG_RAF_HANDLE.with(|cell| cell.replace(None))
        && let Some(window) = web_sys::window()
    {
        let _ = window.cancel_animation_frame(handle);
    }
    LATEST_DRAG_MOVE.with(|cell| cell.set(None));
    DRAG_RAF_CLOSURE.with(|cell| cell.borrow_mut().take());
}

pub(crate) const SLOT_FRAME_GOLD: Asset =
    asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

#[derive(Clone, PartialEq, Debug)]
pub struct InventoryDragFollower {
    pub(crate) section_id: WarcraftObjectId,
    pub(crate) label: String,
    pub(crate) click_offset_horizontal: f64,
    pub(crate) click_offset_vertical: f64,
    pub(crate) cursor_horizontal_position: f64,
    pub(crate) cursor_vertical_position: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
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

#[derive(Clone, PartialEq)]
pub(crate) struct InventoryFilledSlotEntry {
    pub(crate) slot_index: usize,
    pub(crate) section_id: WarcraftObjectId,
    pub(crate) dragging_source: Signal<Option<InventoryDragSource>>,
    pub(crate) drop_target: Signal<Option<WarcraftObjectId>>,
    pub(crate) drag_follower: Signal<Option<InventoryDragFollower>>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct InventorySlotEntry {
    pub(crate) filled: Option<InventoryFilledSlotEntry>,
}

pub(crate) struct PendingInventoryDrag {
    pub(crate) section_id: WarcraftObjectId,
    pub(crate) label: String,
    pub(crate) click_offset_horizontal: f64,
    pub(crate) click_offset_vertical: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) cell_element: web_sys::Element,
    pub(crate) pointer_id: i32,
}

pub(super) struct InventoryGridModel {
    pub(super) frame: String,
    pub(super) entries: Vec<InventorySlotEntry>,
}

pub(super) fn use_inventory_grid() -> InventoryGridModel {
    let dialog_state = use_system_hotkeys_dialog_state();
    let drag_follower = dialog_state.drag_follower();
    let dragging_source = use_signal::<Option<InventoryDragSource>>(|| None);
    let drop_target = use_signal::<Option<WarcraftObjectId>>(|| None);
    let frame_url = SLOT_FRAME_GOLD;
    let frame = format!("--wc3-slot-frame: url('{frame_url}');");
    let inventory_slots: Vec<SystemHotkeysSlot> = InventorySlots::ALL.iter().collect();
    let mut entries: Vec<InventorySlotEntry> = Vec::new();
    for row in 0..InventorySlots::ROWS {
        for column in 0..InventorySlots::COLUMNS {
            let slot_index = row * InventorySlots::COLUMNS + column;
            let slot_option = inventory_slots.get(slot_index).copied();
            let filled = slot_option.map(|slot| {
                let section_id = slot.section_id();
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
