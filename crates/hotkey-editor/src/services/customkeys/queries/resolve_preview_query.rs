use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use warcraft_keybinds::CascadePlan;
use warcraft_keybinds::CustomKeys;

/// Query: the cascade plan a resolve *would* produce — a read-only preview, no
/// mutation. The resolve page previews the plan through this instead of running
/// `preview_resolve` on the aggregate at render time.
pub struct ResolvePreviewQuery;

impl ResolvePreviewQuery {
    /// Answer the query against a keys snapshot. Pure — the service's
    /// [`ddd::QueryHandler`] impl wraps this with the reactive signal read.
    pub fn answer(&self, custom_keys: Option<&CustomKeys>) -> CascadePlan {
        let default_keys = CustomKeys::default();
        let keys = custom_keys.unwrap_or(&default_keys);
        keys.preview_resolve()
    }
}

impl Layered for ResolvePreviewQuery {
    type Layer = ApplicationLayer;
}

impl Query for ResolvePreviewQuery {
    type Output = CascadePlan;
}

#[cfg(test)]
mod tests {
    use super::ResolvePreviewQuery;
    use crate::services::customkeys::queries::assert_query;

    #[test]
    fn resolve_preview_is_a_query() {
        assert_query::<ResolvePreviewQuery>();
    }

    #[test]
    fn an_empty_document_previews_an_empty_plan() {
        let query = ResolvePreviewQuery;
        let plan = query.answer(None);
        assert_eq!(plan.move_count(), 0);
    }
}
