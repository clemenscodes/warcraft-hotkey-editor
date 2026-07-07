use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::UnitCollisionReport;

/// Query: the per-unit collision report for a given grid layout — each unit's
/// hotkey clashes and same-unit position clashes once the layout is applied. The
/// layout is a query parameter (it lives in a separate aggregate); the keys come
/// from the live aggregate.
pub struct UnitCollisionsQuery {
    layout: GridLayout,
}

impl UnitCollisionsQuery {
    pub fn new(layout: GridLayout) -> Self {
        Self { layout }
    }

    /// Answer the query against a keys snapshot. Pure — the service's
    /// [`ddd::QueryHandler`] impl wraps this with the reactive signal read.
    pub fn answer(&self, custom_keys: Option<&CustomKeys>) -> UnitCollisionReport {
        let default_keys = CustomKeys::default();
        let keys = custom_keys.unwrap_or(&default_keys);
        UnitCollisionReport::compute(keys, self.layout)
    }
}

impl Layered for UnitCollisionsQuery {
    type Layer = ApplicationLayer;
}

impl Query for UnitCollisionsQuery {
    type Output = UnitCollisionReport;
}

#[cfg(test)]
mod tests {
    use super::UnitCollisionsQuery;
    use crate::services::customkeys::queries::assert_query;
    use warcraft_keybinds::GridLayout;

    #[test]
    fn unit_collisions_is_a_query() {
        assert_query::<UnitCollisionsQuery>();
    }

    #[test]
    fn an_empty_document_has_no_unit_collisions() {
        let layout = GridLayout::default();
        let query = UnitCollisionsQuery::new(layout);
        let report = query.answer(None);
        assert!(report.entries().is_empty());
    }
}
