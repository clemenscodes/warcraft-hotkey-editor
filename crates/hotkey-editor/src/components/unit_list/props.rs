use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{SearchField, UnitMode};
use warcraft_keybinds::GridSlotId;

/// Everything the unit list reads and writes: the current race and mode, the
/// selection it drives, and the search and catalog-visibility state it owns.
#[derive(Props, Clone, PartialEq)]
pub struct UnitListProps {
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub search_query: Signal<String>,
    pub search_field: Signal<SearchField>,
    pub show_abilityless_units: Signal<bool>,
    pub expand_variants: Signal<bool>,
    pub collapsed_categories: Signal<HashSet<UnitKind>>,
}
