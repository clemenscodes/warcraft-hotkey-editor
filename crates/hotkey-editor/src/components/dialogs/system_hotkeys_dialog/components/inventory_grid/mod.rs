pub mod components;
mod hooks;
mod inventory_cell;
mod props;
mod style;

use std::cell::Cell;

use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;

use crate::assert_component;
use components::inventory_empty_slot::InventoryEmptySlot;
use hooks::use_inventory_grid;
use style::CLASS;

pub use inventory_cell::InventoryCell;
pub use props::InventoryGridProps;

pub(super) const DRAG_MOVEMENT_THRESHOLD_PIXELS: f64 = 4.0;
const INVENTORY_COLUMNS: usize = 2;
const INVENTORY_ROWS: usize = 3;

#[derive(Clone, Copy)]
pub(super) struct DragOrigin {
    pub(super) cursor_horizontal_position: f64,
    pub(super) cursor_vertical_position: f64,
}

thread_local! {
    pub(super) static SUPPRESS_NEXT_CLICK: Cell<bool> = const { Cell::new(false) };
    pub(super) static DRAG_ORIGIN: Cell<Option<DragOrigin>> = const { Cell::new(None) };
    pub(super) static DID_DRAG_MOVE: Cell<bool> = const { Cell::new(false) };
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
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let drag_follower = props.drag_follower;
    let dragging_source = model.dragging_source;
    let drop_target = model.drop_target;
    let binding_map = model.binding_map;
    let entries = SystemHotkeysCategory::Inventory.entries();
    rsx! {
        div {
            class: CLASS,
            style: model.frame,
            for row in 0..INVENTORY_ROWS {
                for column in 0..INVENTORY_COLUMNS {
                    {
                        let slot_index = row * INVENTORY_COLUMNS + column;
                        let entry_option = entries.get(slot_index).copied();
                        match entry_option {
                            Some(entry) => rsx! {
                                InventoryCell {
                                    slot_index,
                                    section_id: entry.section_id().to_string(),
                                    default_hotkey: entry.default_hotkey(),
                                    default_modifier: entry.default_modifier(),
                                    loaded_keys,
                                    editing_section,
                                    dragging_source,
                                    drop_target,
                                    drag_follower,
                                    binding_map,
                                }
                            },
                            None => rsx! {
                                InventoryEmptySlot {}
                            },
                        }
                    }
                }
            }
        }
    }
}
