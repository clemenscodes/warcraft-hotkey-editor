use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use warcraft_keybinds::CrossUnitCollisionReport;
use warcraft_keybinds::CustomKeys;

/// Query: the cross-unit position-collision report — every grid position where
/// abilities from different units contend for the same spot. Answered against the
/// live aggregate so the collisions page never runs the report at render time.
pub struct CrossUnitCollisionsQuery;

impl CrossUnitCollisionsQuery {
    /// Answer the query against a keys snapshot. Pure — the service's
    /// [`ddd::QueryHandler`] impl wraps this with the reactive signal read.
    pub fn answer(&self, custom_keys: Option<&CustomKeys>) -> CrossUnitCollisionReport {
        let default_keys = CustomKeys::default();
        let keys = custom_keys.unwrap_or(&default_keys);
        CrossUnitCollisionReport::compute(keys)
    }
}

impl Layered for CrossUnitCollisionsQuery {
    type Layer = ApplicationLayer;
}

impl Query for CrossUnitCollisionsQuery {
    type Output = CrossUnitCollisionReport;
}

#[cfg(test)]
mod tests {
    use super::CrossUnitCollisionsQuery;
    use crate::services::customkeys::queries::assert_query;

    #[test]
    fn cross_unit_collisions_is_a_query() {
        assert_query::<CrossUnitCollisionsQuery>();
    }

    #[test]
    fn an_empty_document_has_no_cross_unit_collisions() {
        let query = CrossUnitCollisionsQuery;
        let report = query.answer(None);
        assert!(report.position_groups().is_empty());
    }
}
