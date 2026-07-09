use warcraft_api::{Race, WarcraftObjectId};
use warcraft_database::{CatalogVisibility, SearchField, UnitMode};
use warcraft_keybinds::{UnitListing, UnitListingRequest};

/// The default unit to select for a race-and-mode context — the first unit the domain's
/// browse yields for that race and mode. This is a domain decision ("which unit is the
/// default here"), so it is resolved through a `warcraft_keybinds` browse
/// (`UnitListing`), never by the renderer reaching into the game database directly. It
/// is the single home for that decision: switching race and switching mode both ask it
/// which unit to land on.
pub struct DefaultUnit {
    race: Race,
    mode: UnitMode,
}

impl DefaultUnit {
    pub fn new(race: Race, mode: UnitMode) -> Self {
        Self { race, mode }
    }

    /// The default unit's id, or `None` when the race-and-mode context has no units.
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
