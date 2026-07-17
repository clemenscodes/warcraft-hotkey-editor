use warcraft_api::{CatalogVisibility, SearchField, UnitModeSelection};
use warcraft_api::{Race, WarcraftObjectId};
use warcraft_api::{UnitListing, UnitListingRequest};

/// The inputs to "which unit should a fresh browse of this race land on?".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefaultUnitRequest {
    race: Race,
    modes: UnitModeSelection,
}

impl DefaultUnitRequest {
    pub fn new(race: Race, modes: UnitModeSelection) -> Self {
        Self { race, modes }
    }
}

/// The unit a race/mode switch lands on: the first result of a curated browse.
///
/// It deliberately asks under [`CatalogVisibility::default`] rather than the
/// user's own toggles, so switching race always lands somewhere sensible instead
/// of on whatever a widened catalog happens to sort first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct DefaultUnit {
    unit_id: Option<WarcraftObjectId>,
}

impl DefaultUnit {
    pub fn unit_id(&self) -> Option<WarcraftObjectId> {
        self.unit_id
    }
}

impl From<&DefaultUnitRequest> for DefaultUnit {
    fn from(request: &DefaultUnitRequest) -> Self {
        let empty_query = String::new();
        let visibility = CatalogVisibility::default();
        let listing_request = UnitListingRequest::new(
            request.race,
            request.modes,
            empty_query,
            SearchField::UnitName,
            visibility,
        );
        let listing = UnitListing::from(&listing_request);
        let first_result = listing.first_result();
        let unit_id = first_result.map(|entry| entry.unit_id());
        Self { unit_id }
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultUnit;
    use super::DefaultUnitRequest;
    use warcraft_api::{Race, UnitMode, UnitModeSelection};

    #[test]
    fn every_race_has_a_default_melee_unit() {
        let melee = UnitModeSelection::only(UnitMode::Melee);
        for race in [
            Race::Human,
            Race::Orc,
            Race::Nightelf,
            Race::Undead,
            Race::Neutral,
        ] {
            let request = DefaultUnitRequest::new(race, melee);
            let default_unit = DefaultUnit::from(&request);
            assert!(
                default_unit.unit_id().is_some(),
                "{race:?} must have a unit to land on when it is selected"
            );
        }
    }

    #[test]
    fn a_selection_admitting_no_mode_lands_nowhere() {
        let neither = UnitModeSelection {
            melee: false,
            campaign: false,
        };
        let request = DefaultUnitRequest::new(Race::Human, neither);
        let default_unit = DefaultUnit::from(&request);
        assert!(
            default_unit.unit_id().is_none(),
            "a browse that lists nothing has nothing to land on"
        );
    }
}
