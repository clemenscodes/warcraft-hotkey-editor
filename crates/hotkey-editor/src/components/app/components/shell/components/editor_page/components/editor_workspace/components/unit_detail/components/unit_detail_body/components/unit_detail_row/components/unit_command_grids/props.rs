use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::Race;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;

/// The unit's four command grids (command card, build menu, uprooted, research) and
/// all the shared editor state they need.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCommandGridsProps {
    pub unit_id: String,
    pub race: Race,
    pub command_card_slots: Rc<[GridSlotId]>,
    pub build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub research_menu_slots: Option<Rc<[GridSlotId]>>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<String, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub grid_layout: Signal<GridLayout>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
}
