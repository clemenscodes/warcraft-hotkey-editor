//! The unit catalog: one place that turns the live filter controls into the
//! listing they describe.
//!
//! The filter axes are spread across two owners — the race, the modes and the
//! search text are navigation (they belong in the URL), while the search field
//! and the visibility toggles are editor state. Every consumer used to gather
//! those five itself and build its own request, so the list and each of its
//! category sections each ran their own database pass on every keystroke. This
//! service gathers them once into a [`UnitFilterQuery`] and answers it once, and
//! the components below it take the result as props and only render.

pub mod context;
pub mod queries;

use dioxus::prelude::*;
use queries::unit_filter_query::UnitFilterQuery;
use warcraft_api::UnitCatalogListing;

/// The live unit catalog, resolved from the current filter.
#[derive(Clone, Copy, PartialEq)]
pub struct UnitCatalogService {
    filter: Memo<UnitFilterQuery>,
    listing: Memo<UnitCatalogListing>,
}

impl UnitCatalogService {
    pub(crate) fn new(filter: Memo<UnitFilterQuery>, listing: Memo<UnitCatalogListing>) -> Self {
        Self { filter, listing }
    }

    /// What is currently being listed.
    pub fn filter(&self) -> UnitFilterQuery {
        self.filter.read().clone()
    }

    /// The catalog the filter describes: every category with its units.
    pub fn listing(&self) -> UnitCatalogListing {
        self.listing.read().clone()
    }
}
