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
    pub(super) fn resolve(self) -> UnitListing {
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
