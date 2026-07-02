use super::components::editor_tabs_bar::EditorTabsBarProps;
use super::components::editor_workspace::EditorWorkspaceProps;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::{Race, UnitKind};
use warcraft_database::{SearchField, UnitMode};
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId};

/// The editor view's input: all of the editor navigation and selection state. It is
/// split between the top tab bar and the workspace by conversion.
#[derive(Props, Clone, PartialEq)]
pub struct EditorPageProps {
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

impl From<&EditorPageProps> for EditorTabsBarProps {
    fn from(props: &EditorPageProps) -> Self {
        Self {
            unit_mode: props.unit_mode,
            active_race: props.active_race,
            selected_unit_id: props.selected_unit_id,
            selected_slot: props.selected_slot,
        }
    }
}

impl From<&EditorPageProps> for EditorWorkspaceProps {
    fn from(props: &EditorPageProps) -> Self {
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
