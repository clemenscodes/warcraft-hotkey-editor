use std::collections::HashSet;

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{UnitCatalog, UnitMode};

pub(super) struct UnitListState {
    active_category: Signal<UnitKind>,
    race: Race,
    mode: UnitMode,
    query_snapshot: String,
    search_active: bool,
    active_kind: UnitKind,
    active_unit_id: Option<String>,
    collapsed_snapshot: HashSet<UnitKind>,
    category_kinds: Vec<UnitKind>,
}

impl UnitListState {
    pub(super) fn new(
        active_race: Signal<Race>,
        unit_mode: Signal<UnitMode>,
        search_query: Signal<String>,
        selected_unit_id: Signal<Option<String>>,
        collapsed_categories: Signal<HashSet<UnitKind>>,
    ) -> Self {
        let active_category = use_signal::<UnitKind>(|| UnitKind::Soldier);
        let race = *active_race.read();
        let mode = *unit_mode.read();
        let query_snapshot = search_query.read().clone();
        let search_active = !query_snapshot.is_empty();
        let active_kind = *active_category.read();
        let active_unit_id = selected_unit_id.read().clone();
        let collapsed_snapshot = collapsed_categories.read().clone();
        let query_str = query_snapshot.as_str();
        let query_option = Some(query_str);
        let all_entries = UnitCatalog::entries_for(race, mode, None, query_option);
        let mut seen: Vec<UnitKind> = Vec::new();
        for entry in all_entries {
            let entry_kind = entry.unit_kind();
            if !seen.contains(&entry_kind) {
                seen.push(entry_kind);
            }
        }
        Self {
            active_category,
            race,
            mode,
            query_snapshot,
            search_active,
            active_kind,
            active_unit_id,
            collapsed_snapshot,
            category_kinds: seen,
        }
    }

    pub(super) fn active_category(&self) -> Signal<UnitKind> {
        self.active_category
    }

    pub(super) fn race(&self) -> Race {
        self.race
    }

    pub(super) fn mode(&self) -> UnitMode {
        self.mode
    }

    pub(super) fn query_snapshot(&self) -> &str {
        &self.query_snapshot
    }

    pub(super) fn search_active(&self) -> bool {
        self.search_active
    }

    pub(super) fn active_kind(&self) -> UnitKind {
        self.active_kind
    }

    pub(super) fn active_unit_id(&self) -> Option<&str> {
        self.active_unit_id.as_deref()
    }

    pub(super) fn collapsed_snapshot(&self) -> &HashSet<UnitKind> {
        &self.collapsed_snapshot
    }

    pub(super) fn category_kinds(&self) -> &[UnitKind] {
        &self.category_kinds
    }
}
