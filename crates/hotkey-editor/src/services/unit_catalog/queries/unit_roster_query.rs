use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use warcraft_api::CatalogVisibility;
use warcraft_api::UnitRoster;
use warcraft_api::UnitRosterRequest;

/// The whole game, as one ordered list: every unit of every race, in canonical
/// race order (Human, Orc, Nightelf, Undead, Neutral) and melee before campaign,
/// each unit once.
///
/// Unlike [`UnitFilterQuery`](super::unit_filter_query::UnitFilterQuery) it is
/// narrowed by nothing but the visibility toggles — the mobile pager walks the
/// entire roster rather than one race's filtered listing, so a swipe carries
/// from the last unit of one race into the first of the next. The order is a
/// domain decision behind [`UnitRoster`] (ARCHITECTURE R3); this type only
/// carries the visibility inputs across the wall.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitRosterQuery {
    visibility: CatalogVisibility,
}

impl UnitRosterQuery {
    pub fn new(visibility: CatalogVisibility) -> Self {
        Self { visibility }
    }

    /// The full ordered roster from one database pass.
    pub fn answer(&self) -> UnitRoster {
        let request = UnitRosterRequest::from(self);
        UnitRoster::from(&request)
    }

    pub fn visibility(&self) -> CatalogVisibility {
        self.visibility
    }
}

impl From<&UnitRosterQuery> for UnitRosterRequest {
    fn from(query: &UnitRosterQuery) -> Self {
        Self::new(query.visibility)
    }
}

impl Layered for UnitRosterQuery {
    type Layer = ApplicationLayer;
}

impl Query for UnitRosterQuery {
    type Output = UnitRoster;
}

#[cfg(test)]
mod tests {
    use super::UnitRosterQuery;
    use crate::services::unit_catalog::queries::assert_query;
    use warcraft_api::CatalogVisibility;

    #[test]
    fn unit_roster_is_a_query() {
        assert_query::<UnitRosterQuery>();
    }

    #[test]
    fn the_roster_lists_the_whole_game() {
        let visibility = CatalogVisibility::default();
        let query = UnitRosterQuery::new(visibility);
        let roster = query.answer();
        assert!(
            !roster.entries().is_empty(),
            "the roster walks every unit of every race, so it is never empty"
        );
    }
}
