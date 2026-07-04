use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_api::Race;
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId};

/// Everything the unit detail panel reads: the active race and selection, the
/// drag state, the loaded keys, and the grid layout.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailPanelProps {
    pub active_race: Signal<Race>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<String, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
}
