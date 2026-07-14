use crate::services::drag_state::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::{GridLayout, GridSlotId};

#[derive(Props, Clone, PartialEq)]
pub struct GridEditorView {
    pub heading: &'static str,
    pub slot_ids: Rc<[GridSlotId]>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<WarcraftObjectId, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub grid_layout: Signal<GridLayout>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
    #[props(default = false)]
    pub prevent_swap_on_drop: bool,
    #[props(default)]
    pub restrict_draggable_to: Vec<GridSlotId>,
    #[props(default)]
    pub host_unit_id: WarcraftObjectId,
}

impl ddd::View for GridEditorView {}
