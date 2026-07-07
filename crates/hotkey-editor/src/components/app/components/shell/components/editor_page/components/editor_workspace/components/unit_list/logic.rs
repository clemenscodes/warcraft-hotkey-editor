use warcraft_api::Race;
use warcraft_database::{CatalogVisibility, SearchField, UnitMode};
use warcraft_keybinds::{UnitListing, UnitListingRequest};

/// The inputs the memoized catalog walk reads — race, mode, committed query,
/// search field, and catalog visibility. It orchestrates the domain call
/// [`UnitListing::resolve`]; the walk itself lives in `warcraft-keybinds`.
#[derive(Clone, PartialEq, Debug)]
pub(super) struct CatalogListingInputs {
    pub(super) race: Race,
    pub(super) mode: UnitMode,
    pub(super) query: String,
    pub(super) search_field: SearchField,
    pub(super) visibility: CatalogVisibility,
}

impl CatalogListingInputs {
    /// Consume these inputs into the domain [`UnitListing`]. A consuming `into_*`
    /// conversion rather than `From`/`Into`, since the output is the foreign domain
    /// type (the orphan rule forbids a `From` impl in the renderer crate).
    pub(super) fn into_listing(self) -> UnitListing {
        let Self {
            race,
            mode,
            query,
            search_field,
            visibility,
        } = self;
        let request = UnitListingRequest::new(race, mode, query, search_field, visibility);
        UnitListing::resolve(&request)
    }
}
