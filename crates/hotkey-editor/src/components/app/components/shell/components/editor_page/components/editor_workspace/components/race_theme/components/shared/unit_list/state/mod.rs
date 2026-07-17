use warcraft_api::UnitCatalogGroup;
use warcraft_api::UnitCatalogListing;
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

/// The listing shaped for the aside: the groups it renders and the entry the
/// search box jumps to on Enter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct UnitListState {
    groups: Vec<UnitCatalogGroup>,
    first_result: Option<FirstResult>,
}

impl UnitListState {
    pub(super) fn groups(&self) -> &[UnitCatalogGroup] {
        &self.groups
    }

    pub(super) fn first_result(&self) -> Option<FirstResult> {
        self.first_result
    }
}

impl From<UnitCatalogListing> for UnitListState {
    fn from(listing: UnitCatalogListing) -> Self {
        let first_result = listing.first_result().map(|entry| {
            let id = entry.unit_id();
            let kind = entry.unit_kind();
            FirstResult { id, kind }
        });
        let groups = listing.into_groups();
        Self {
            groups,
            first_result,
        }
    }
}
