mod inventory_cell;

use std::cell::Cell;

use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

use inventory_cell::InventoryCell;

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
pub(crate) struct InventoryDragFollower {
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
pub(super) struct InventoryDragSource {
    pub(super) section_id: String,
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct InventoryGridProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) editing_section: Signal<Option<String>>,
    pub(crate) drag_follower: Signal<Option<InventoryDragFollower>>,
}

#[component]
pub(crate) fn InventoryGrid(props: InventoryGridProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let drag_follower = props.drag_follower;
    let entries = SystemHotkeysCategory::Inventory.entries();
    let dragging_source = use_signal::<Option<InventoryDragSource>>(|| None);
    let drop_target = use_signal::<Option<String>>(|| None);
    let binding_map = use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    });

    let slot_frame_url = SLOT_FRAME_GOLD;
    let frame_style = format!("--wc3-slot-frame: url('{slot_frame_url}');");
    rsx! {
        div { class: "wc3-inventory-grid", style: frame_style,
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
                                div { class: "wc3-slot empty", "—" }
                            },
                        }
                    }
                }
            }
        }
    }
}
