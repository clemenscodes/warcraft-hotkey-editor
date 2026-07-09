use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::UnitDetailProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::UnitListProps;
use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{Race, UnitKind, WarcraftObjectId};
use warcraft_api::{SearchField, UnitMode};
use warcraft_keybinds::GridSlotId;

/// The race-theme container's input: the active race that picks the `--race-color`
/// this wrapper publishes, plus every signal the unit list and unit detail panel need.
/// The wrapper is a `display:contents` grouping element, so it lays out nothing of its
/// own; it only threads each child its own props and colours the subtree by race.
#[derive(Props, Clone, PartialEq)]
pub struct RaceThemeProps {
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

impl From<&RaceThemeProps> for UnitListProps {
    fn from(props: &RaceThemeProps) -> Self {
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

impl From<&RaceThemeProps> for UnitDetailProps {
    fn from(props: &RaceThemeProps) -> Self {
        Self {
            active_race: props.active_race,
            selected_unit_id: props.selected_unit_id,
        }
    }
}
