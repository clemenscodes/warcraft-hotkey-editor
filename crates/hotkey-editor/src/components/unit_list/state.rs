use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{CatalogVisibility, SearchField, UnitMode};
use warcraft_keybinds::{UnitListing, UnitListingRequest};

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
    first_result_id: Option<String>,
    first_result_kind: Option<UnitKind>,
}

impl UnitListState {
    pub(super) fn new(
        active_race: Signal<Race>,
        unit_mode: Signal<UnitMode>,
        search_query: Signal<String>,
        search_field: SearchField,
        selected_unit_id: Signal<Option<String>>,
        collapsed_categories: Signal<HashSet<UnitKind>>,
        visibility: CatalogVisibility,
    ) -> Self {
        let active_category = use_signal::<UnitKind>(|| UnitKind::Soldier);
        let race = *active_race.read();
        let mode = *unit_mode.read();
        let query_snapshot = search_query.read().clone();
        let search_active = !query_snapshot.is_empty();
        let active_kind = *active_category.read();
        let active_unit_id = selected_unit_id.read().clone();
        let collapsed_snapshot = collapsed_categories.read().clone();
        let listing_query = query_snapshot.clone();
        let listing_request =
            UnitListingRequest::new(race, mode, listing_query, search_field, visibility);
        let listing = UnitListing::resolve(&listing_request);
        let category_kinds = listing.category_kinds().to_vec();
        let first_result = listing.first_result();
        let first_result_id = first_result.map(|entry| entry.unit_id().to_owned());
        let first_result_kind = first_result.map(|entry| entry.unit_kind());
        Self {
            active_category,
            race,
            mode,
            query_snapshot,
            search_active,
            active_kind,
            active_unit_id,
            collapsed_snapshot,
            category_kinds,
            first_result_id,
            first_result_kind,
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

    pub(super) fn first_result_id(&self) -> Option<&str> {
        self.first_result_id.as_deref()
    }

    pub(super) fn first_result_kind(&self) -> Option<UnitKind> {
        self.first_result_kind
    }
}
