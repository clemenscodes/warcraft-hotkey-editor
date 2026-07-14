use warcraft_api::{CatalogVisibility, SearchField, UnitMode};
use warcraft_api::{Race, WarcraftObjectId};
use warcraft_api::{UnitListing, UnitListingRequest};

pub struct DefaultUnit {
    race: Race,
    mode: UnitMode,
}

impl DefaultUnit {
    pub fn new(race: Race, mode: UnitMode) -> Self {
        Self { race, mode }
    }

    pub fn resolve(&self) -> Option<WarcraftObjectId> {
        let empty_query = String::new();
        let visibility = CatalogVisibility::default();
        let request = UnitListingRequest::new(
            self.race,
            self.mode,
            empty_query,
            SearchField::UnitName,
            visibility,
        );
        let listing = UnitListing::resolve(&request);
        let first_result = listing.first_result();
        first_result.map(|entry| entry.unit_id())
    }
}
