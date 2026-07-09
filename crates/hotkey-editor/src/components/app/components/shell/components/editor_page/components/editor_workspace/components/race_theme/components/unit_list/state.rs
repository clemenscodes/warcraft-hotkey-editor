use warcraft_api::{UnitKind, WarcraftObjectId};
use warcraft_keybinds::UnitListing;

/// The first search result the Enter key selects: the unit's id and its kind (so the
/// list can both select it and switch to its category in one step).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct FirstResult {
    id: WarcraftObjectId,
    kind: UnitKind,
}

impl FirstResult {
    pub(super) fn id(&self) -> WarcraftObjectId {
        self.id
    }

    pub(super) fn kind(&self) -> UnitKind {
        self.kind
    }
}

/// The list's listing-derived state: the category kinds to render (in display order)
/// and the first search result (the unit Enter selects). Everything else the list
/// draws — the active category, the collapsed set, the selection — is read from
/// context by the sections and cards, so it is not snapshotted here.
pub(super) struct UnitListState {
    category_kinds: Vec<UnitKind>,
    first_result: Option<FirstResult>,
}

impl UnitListState {
    /// Builds the list's derived state from the already-resolved `listing` (the caller
    /// memoizes the catalog walk on its real inputs; this constructor never re-runs it).
    pub(super) fn new(listing: UnitListing) -> Self {
        let category_kinds = listing.category_kinds().to_vec();
        let first_result = listing.first_result().map(|entry| {
            let id = entry.unit_id();
            let kind = entry.unit_kind();
            FirstResult { id, kind }
        });
        Self {
            category_kinds,
            first_result,
        }
    }

    pub(super) fn category_kinds(&self) -> &[UnitKind] {
        &self.category_kinds
    }

    pub(super) fn first_result(&self) -> Option<FirstResult> {
        self.first_result
    }
}
