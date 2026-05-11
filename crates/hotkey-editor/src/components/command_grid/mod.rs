mod drag_state;
mod grid_cell;
mod grid_tile;
mod tile_class;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_database::BuildingTraits;
use warcraft_keybinds::CustomKeys;

use crate::model::grid::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};
use crate::services::customkeys::positions::Positions;

use grid_tile::GridTile;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct CommandGridSectionProps {
    pub(crate) heading: &'static str,
    pub(crate) slot_ids: Rc<[GridSlotId]>,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) selected_slot: Signal<Option<GridSlotId>>,
    pub(crate) selected_from_research: Signal<bool>,
    pub(crate) selected_from_uprooted: Signal<bool>,
    pub(crate) tier_overrides: Signal<HashMap<String, usize>>,
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) grid_layout: Signal<GridLayout>,
    #[props(default = false)]
    pub(crate) is_research_grid: bool,
    #[props(default = false)]
    pub(crate) is_uprooted_grid: bool,
    /// When true, drops onto cells already occupied by another slot are
    /// rejected outright instead of swapping. The off-state position
    /// picker uses this so dragging the toggle's off half can't displace
    /// another ability's on-state on the unit's command card.
    #[props(default = false)]
    pub(crate) prevent_swap_on_drop: bool,
    /// When non-empty, only slots whose `as_str()` matches one of these
    /// ids start a drag — other slots render in their cells but are
    /// display-only. Used by the off-state picker to keep the player from
    /// accidentally rearranging the unit's primary command card while
    /// editing one toggle's off position.
    #[props(default)]
    pub(crate) restrict_draggable_to: Vec<GridSlotId>,
    /// Unit ID of the host — used to block dragging of morph abilities on
    /// alternate-form units (e.g. Burrowed Crypt Fiend). Empty string
    /// disables the check (off-state picker, build menus without a unit).
    #[props(default)]
    pub(crate) host_unit_id: String,
}

#[component]
pub(crate) fn CommandGridSection(props: CommandGridSectionProps) -> Element {
    let read_guard = props.loaded_keys.read();
    let custom_keys_option = read_guard.as_ref();
    let is_research_grid = props.is_research_grid;
    let is_uprooted_grid = props.is_uprooted_grid;
    let keys_signal = props.loaded_keys;
    let select_slot = props.selected_slot;
    let select_from_research = props.selected_from_research;
    let select_from_uprooted = props.selected_from_uprooted;
    let tier_overrides = props.tier_overrides;
    let dragging_slot = props.dragging_slot;
    let drop_target_cell = props.drop_target_cell;
    let drag_follower = props.drag_follower;
    let grid_layout = props.grid_layout;
    let slot_ids = props.slot_ids.clone();
    let heading_text = props.heading;
    let prevent_swap_on_drop = props.prevent_swap_on_drop;
    let restrict_draggable_to: Rc<[GridSlotId]> = props.restrict_draggable_to.clone().into();
    let host_unit_id = props.host_unit_id.clone();
    let host_is_alt_form =
        !host_unit_id.is_empty() && BuildingTraits::unit_starts_in_toggle_alt_state(&host_unit_id);

    let conflicting_hotkeys: Rc<HashSet<String>> = {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for row in 0..COMMAND_GRID_ROWS {
            for column in 0..COMMAND_GRID_COLUMNS {
                let cell_with_slot = Positions::cell_for_position(
                    &slot_ids,
                    custom_keys_option,
                    is_research_grid,
                    column,
                    row,
                );
                let letter = cell_with_slot.as_ref().and_then(|occupant| {
                    let cell = occupant.cell();
                    let token = if is_research_grid {
                        cell.binding_research_hotkey()
                            .or_else(|| cell.binding_hotkey())
                    } else {
                        cell.binding_hotkey()
                    };
                    token.map(|token| token.display_label())
                });
                if let Some(letter_label) = letter {
                    *counts.entry(letter_label).or_insert(0) += 1;
                }
            }
        }
        let conflict_set: HashSet<String> = counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(key, _)| key)
            .collect();
        Rc::new(conflict_set)
    };

    rsx! {
        div { class: "command-section",
            h3 { class: "command-section-heading", {heading_text} }
            div { class: "grid-tiles",
                for row in 0..COMMAND_GRID_ROWS {
                    for column in 0..COMMAND_GRID_COLUMNS {
                        GridTile {
                            column,
                            row,
                            heading: heading_text,
                            slot_ids: slot_ids.clone(),
                            loaded_keys: keys_signal,
                            selected_slot: select_slot,
                            selected_from_research: select_from_research,
                            selected_from_uprooted: select_from_uprooted,
                            tier_overrides,
                            dragging_slot,
                            drop_target_cell,
                            drag_follower,
                            grid_layout,
                            conflicting_hotkeys: conflicting_hotkeys.clone(),
                            is_research_grid,
                            is_uprooted_grid,
                            prevent_swap_on_drop,
                            restrict_draggable_to: restrict_draggable_to.clone(),
                            host_unit_id: host_unit_id.clone(),
                            host_is_alt_form,
                        }
                    }
                }
            }
        }
    }
}
