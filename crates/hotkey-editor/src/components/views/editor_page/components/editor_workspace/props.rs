use crate::components::unit_detail::UnitDetailPanelProps;
use crate::components::unit_list::UnitListProps;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::{Race, UnitKind};
use warcraft_database::{SearchField, UnitMode};
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId};

/// The editor workspace's input: the unit list's state and the detail panel's state,
/// plus the active race that tints the whole workspace. The workspace lays the unit
/// list beside (or, on narrow widths, above) the detail panel and hands each child
/// its own props.
#[derive(Props, Clone, PartialEq)]
pub struct EditorWorkspaceProps {
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub search_query: Signal<String>,
    pub search_field: Signal<SearchField>,
    pub show_abilityless_units: Signal<bool>,
    pub expand_variants: Signal<bool>,
    pub collapsed_categories: Signal<HashSet<UnitKind>>,
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

impl From<&EditorWorkspaceProps> for UnitListProps {
    fn from(props: &EditorWorkspaceProps) -> Self {
        Self {
            active_race: props.active_race,
            unit_mode: props.unit_mode,
            selected_unit_id: props.selected_unit_id,
            selected_slot: props.selected_slot,
            search_query: props.search_query,
            search_field: props.search_field,
            show_abilityless_units: props.show_abilityless_units,
            expand_variants: props.expand_variants,
            collapsed_categories: props.collapsed_categories,
        }
    }
}

impl From<&EditorWorkspaceProps> for UnitDetailPanelProps {
    fn from(props: &EditorWorkspaceProps) -> Self {
        Self {
            active_race: props.active_race,
            selected_unit_id: props.selected_unit_id,
            selected_slot: props.selected_slot,
            selected_from_research: props.selected_from_research,
            selected_from_uprooted: props.selected_from_uprooted,
            tier_overrides: props.tier_overrides,
            dragging_slot: props.dragging_slot,
            drop_target_tile: props.drop_target_tile,
            drag_follower: props.drag_follower,
            loaded_keys: props.loaded_keys,
            grid_layout: props.grid_layout,
            update_hotkeys_on_move: props.update_hotkeys_on_move,
            hotkey_assign_request: props.hotkey_assign_request,
        }
    }
}
