use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId, InspectorDetail};

/// The override slot's inputs: the inspector detail (absent when no tile is
/// selected) and all the editor signals its override card drives.
#[derive(Props, Clone, PartialEq)]
pub struct UnitTileOverrideProps {
    pub detail: Option<InspectorDetail>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<String, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub active_container_slots: Rc<[GridSlotId]>,
    pub hotkey_assign_request: Signal<bool>,
}
