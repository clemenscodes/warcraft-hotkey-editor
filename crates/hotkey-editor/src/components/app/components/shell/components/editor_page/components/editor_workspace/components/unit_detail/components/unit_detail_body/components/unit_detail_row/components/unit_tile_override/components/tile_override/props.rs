use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::InspectorDetail;

use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};

/// The per-tile override editor: the panel that edits the selected ability's hotkey,
/// off-state, upgraded form, and command-card position.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideProps {
    pub detail: InspectorDetail,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<std::collections::HashMap<String, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub active_container_slots: Rc<[GridSlotId]>,
    pub hotkey_assign_request: Signal<bool>,
}
