use warcraft_api::UnitListing;
use warcraft_api::{UnitKind, WarcraftObjectId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct UnitListState {
    category_kinds: Vec<UnitKind>,
    first_result: Option<FirstResult>,
}

impl UnitListState {
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
