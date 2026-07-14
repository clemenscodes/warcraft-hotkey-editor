use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use warcraft_keybinds::CascadePlan;
use warcraft_keybinds::CustomKeys;

pub struct ResolvePreviewQuery;

impl ResolvePreviewQuery {
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
