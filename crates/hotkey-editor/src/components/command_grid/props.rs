use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::CustomKeys;

use crate::model::grid::{DragFollower, DraggingSlot, DropTargetCell, GridLayout, GridSlotId};

#[derive(Props, Clone, PartialEq)]
pub struct CommandGridSectionProps {
    pub heading: &'static str,
    /// The owning unit's race, forwarded to every tile for accent theming. One
    /// race per grid.
    #[props(default = Race::Neutral)]
    pub race: Race,
    pub slot_ids: Rc<[GridSlotId]>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<String, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_cell: Signal<Option<DropTargetCell>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub grid_layout: Signal<GridLayout>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
    #[props(default = false)]
    pub is_research_grid: bool,
    #[props(default = false)]
    pub is_uprooted_grid: bool,
    /// When true, drops onto cells already occupied by another slot are
    /// rejected outright instead of swapping. The off-state position
    /// picker uses this so dragging the toggle's off half can't displace
    /// another ability's on-state on the unit's command card.
    #[props(default = false)]
    pub prevent_swap_on_drop: bool,
    /// When non-empty, only slots whose `as_str()` matches one of these
    /// ids start a drag — other slots render in their cells but are
    /// display-only. Used by the off-state picker to keep the player from
    /// accidentally rearranging the unit's primary command card while
    /// editing one toggle's off position.
    #[props(default)]
    pub restrict_draggable_to: Vec<GridSlotId>,
    /// Unit ID of the host — used to block dragging of morph abilities on
    /// alternate-form units (e.g. Burrowed Crypt Fiend). Empty string
    /// disables the check (off-state picker, build menus without a unit).
    #[props(default)]
    pub host_unit_id: String,
}
