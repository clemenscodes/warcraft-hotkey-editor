use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use warcraft_api::CatalogVisibility;
use warcraft_api::Race;
use warcraft_api::RaceSelection;
use warcraft_api::SearchField;
use warcraft_api::UnitCatalogListing;
use warcraft_api::UnitListingRequest;
use warcraft_api::UnitModeSelection;

/// Every axis the unit list can be narrowed by, in one value.
///
/// Each filter control in the navigation contributes exactly one field, so the
/// question "what is currently being listed?" has a single answer rather than
/// five signals a caller has to remember to read together. It is the query, not
/// a bag handed to one: [`answer`](Self::answer) turns it into the listing.
///
/// The renderer does not decide what the axes mean. Whether a search drops the
/// race and the mode, how fuzzy matches are suppressed, how the results sort —
/// all of that stays behind [`UnitCatalogListing`] in the domain (ARCHITECTURE
/// R3). This type only carries the inputs across the wall.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitFilterQuery {
    race: Race,
    modes: UnitModeSelection,
    search_query: String,
    search_field: SearchField,
    visibility: CatalogVisibility,
    search_race_scope: RaceSelection,
}

impl UnitFilterQuery {
    pub fn new(
        race: Race,
        modes: UnitModeSelection,
        search_query: String,
        search_field: SearchField,
        visibility: CatalogVisibility,
        search_race_scope: RaceSelection,
    ) -> Self {
        Self {
            race,
            modes,
            search_query,
            search_field,
            visibility,
            search_race_scope,
        }
    }

    /// The catalog this filter describes: every category with its units, and the
    /// first result, from one database pass.
    pub fn answer(&self) -> UnitCatalogListing {
        let request = UnitListingRequest::from(self);
        UnitCatalogListing::from(&request)
    }

    /// Whether a search is narrowing the list. While one is, the domain ignores
    /// the race and the mode — a hit may belong to any of either — so the
    /// controls for those axes have nothing to act on and say so.
    pub fn is_searching(&self) -> bool {
        !self.search_query.trim().is_empty()
    }

    pub fn race(&self) -> Race {
        self.race
    }

    pub fn modes(&self) -> UnitModeSelection {
        self.modes
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn search_field(&self) -> SearchField {
        self.search_field
    }

    pub fn visibility(&self) -> CatalogVisibility {
        self.visibility
    }
}

impl From<&UnitFilterQuery> for UnitListingRequest {
    fn from(filter: &UnitFilterQuery) -> Self {
        let search_query = filter.search_query.clone();
        let request = Self::new(
            filter.race,
            filter.modes,
            search_query,
            filter.search_field,
            filter.visibility,
        );
        request.searching_within(filter.search_race_scope.clone())
    }
}

impl Layered for UnitFilterQuery {
    type Layer = ApplicationLayer;
}

impl Query for UnitFilterQuery {
    type Output = UnitCatalogListing;
}

#[cfg(test)]
mod tests {
    use super::UnitFilterQuery;
    use crate::services::unit_catalog::queries::assert_query;
    use warcraft_api::CatalogVisibility;
    use warcraft_api::Race;
    use warcraft_api::RaceSelection;
    use warcraft_api::SearchField;
    use warcraft_api::UnitMode;
    use warcraft_api::UnitModeSelection;

    fn human_melee(search_query: &str) -> UnitFilterQuery {
        let modes = UnitModeSelection::only(UnitMode::Melee);
        let owned_query = search_query.to_owned();
        UnitFilterQuery::new(
            Race::Human,
            modes,
            owned_query,
            SearchField::UnitName,
            CatalogVisibility::default(),
            RaceSelection::All,
        )
    }

    #[test]
    fn unit_filter_is_a_query() {
        assert_query::<UnitFilterQuery>();
    }

    #[test]
    fn an_unfiltered_browse_answers_with_grouped_units() {
        let filter = human_melee("");
        let catalog = filter.answer();
        assert!(!catalog.groups().is_empty(), "a human melee browse groups");
        for group in catalog.groups() {
            assert!(!group.entries().is_empty(), "a group is never empty");
        }
    }

    #[test]
    fn an_empty_query_is_not_searching() {
        let filter = human_melee("");
        assert!(!filter.is_searching());
    }

    #[test]
    fn a_whitespace_query_is_not_searching() {
        let filter = human_melee("   ");
        assert!(
            !filter.is_searching(),
            "whitespace alone matches everything, so treating it as a search would \
             silently drop the race and the mode while looking like an idle browse"
        );
    }

    #[test]
    fn a_real_query_is_searching() {
        let filter = human_melee("foot");
        assert!(filter.is_searching());
    }

    #[test]
    fn selecting_both_modes_answers_with_more_than_melee_alone() {
        let melee = human_melee("");
        let melee_count: usize = melee
            .answer()
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        let both_modes = UnitModeSelection::both();
        let both = UnitFilterQuery::new(
            Race::Human,
            both_modes,
            String::new(),
            SearchField::UnitName,
            CatalogVisibility::default(),
            RaceSelection::All,
        );
        let both_count: usize = both
            .answer()
            .groups()
            .iter()
            .map(|group| group.entries().len())
            .sum();
        assert!(
            both_count > melee_count,
            "the two modes are independent, so selecting both must widen the list"
        );
    }
}
