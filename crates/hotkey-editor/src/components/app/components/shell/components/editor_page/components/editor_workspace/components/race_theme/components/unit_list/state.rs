use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{UnitKind, WarcraftObjectId};
use warcraft_keybinds::UnitListing;

pub(super) struct UnitListState {
    active_category: Signal<UnitKind>,
    query_snapshot: String,
    search_active: bool,
    active_kind: UnitKind,
    active_unit_id: Option<WarcraftObjectId>,
    collapsed_snapshot: HashSet<UnitKind>,
    category_kinds: Vec<UnitKind>,
    first_result_id: Option<WarcraftObjectId>,
    first_result_kind: Option<UnitKind>,
}

impl UnitListState {
    /// Builds the list's derived state from the already-resolved `listing` (the
    /// caller memoizes the catalog walk on its real inputs; this constructor never
    /// re-runs it).
    pub(super) fn new(
        query_snapshot: String,
        selected_unit_id: Signal<Option<WarcraftObjectId>>,
        collapsed_categories: Signal<HashSet<UnitKind>>,
        listing: UnitListing,
    ) -> Self {
        let active_category = use_signal::<UnitKind>(|| UnitKind::Soldier);
        let search_active = !query_snapshot.is_empty();
        let active_kind = *active_category.read();
        let active_unit_id = *selected_unit_id.read();
        let collapsed_snapshot = collapsed_categories.read().clone();
        let category_kinds = listing.category_kinds().to_vec();
        let first_result = listing.first_result();
        let first_result_id = first_result.map(|entry| entry.unit_id());
        let first_result_kind = first_result.map(|entry| entry.unit_kind());
        Self {
            active_category,
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

    pub(super) fn query_snapshot(&self) -> &str {
        &self.query_snapshot
    }

    pub(super) fn search_active(&self) -> bool {
        self.search_active
    }

    pub(super) fn active_kind(&self) -> UnitKind {
        self.active_kind
    }

    pub(super) fn active_unit_id(&self) -> Option<WarcraftObjectId> {
        self.active_unit_id
    }

    pub(super) fn collapsed_snapshot(&self) -> &HashSet<UnitKind> {
        &self.collapsed_snapshot
    }

    pub(super) fn category_kinds(&self) -> &[UnitKind] {
        &self.category_kinds
    }

    pub(super) fn first_result_id(&self) -> Option<WarcraftObjectId> {
        self.first_result_id
    }

    pub(super) fn first_result_kind(&self) -> Option<UnitKind> {
        self.first_result_kind
    }
}
