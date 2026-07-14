use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use warcraft_keybinds::CollisionSummary;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;

pub struct CollisionSummaryQuery {
    layout: GridLayout,
}

impl CollisionSummaryQuery {
    pub fn new(layout: GridLayout) -> Self {
        Self { layout }
    }

    pub fn answer(&self, custom_keys: Option<&CustomKeys>) -> CollisionSummary {
        match custom_keys {
            Some(keys) => CollisionSummary::compute(keys, self.layout),
            None => CollisionSummary::default(),
        }
    }
}

impl Layered for CollisionSummaryQuery {
    type Layer = ApplicationLayer;
}

impl Query for CollisionSummaryQuery {
    type Output = CollisionSummary;
}

#[cfg(test)]
mod tests {
    use super::CollisionSummaryQuery;
    use crate::services::customkeys::queries::assert_query;
    use warcraft_keybinds::GridLayout;

    #[test]
    fn collision_summary_is_a_query() {
        assert_query::<CollisionSummaryQuery>();
    }

    #[test]
    fn an_empty_document_summarises_to_no_collisions() {
        let layout = GridLayout::default();
        let query = CollisionSummaryQuery::new(layout);
        let summary = query.answer(None);
        assert_eq!(summary, warcraft_keybinds::CollisionSummary::default());
    }
}
