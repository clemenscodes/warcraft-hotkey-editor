use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::RaceThemeProps;
use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{Race, UnitKind, WarcraftObjectId};
use warcraft_api::{SearchField, UnitMode};
use warcraft_keybinds::GridSlotId;

/// The editor workspace's input: the unit list's state, plus the active race and
/// selected unit the detail panel needs. The workspace lays the unit list beside (or,
/// on narrow widths, above) the detail panel and hands each child its own props. The
/// editor signals the detail panel's grids and override card drive are sourced from
/// context by those children, not threaded through here.
#[derive(Props, Clone, PartialEq)]
pub struct EditorWorkspaceProps {
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<WarcraftObjectId>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub search_query: Signal<String>,
    pub search_field: Signal<SearchField>,
    pub show_abilityless_units: Signal<bool>,
    pub expand_variants: Signal<bool>,
    pub collapsed_categories: Signal<HashSet<UnitKind>>,
}

impl From<&EditorWorkspaceProps> for RaceThemeProps {
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
