use dioxus::prelude::*;
use warcraft_api::CatalogVisibility;

use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::UnitCatalogService;
use crate::services::unit_catalog::queries::unit_filter_query::UnitFilterQuery;

pub(crate) fn use_unit_catalog() -> UnitCatalogService {
    use_context()
}

pub(crate) fn use_unit_catalog_provider() -> UnitCatalogService {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let active_race = navigation.active_race();
    let unit_modes = navigation.unit_modes();
    let search_query = navigation.search_query();
    let search_field = editor.search_field();
    let show_abilityless_units = editor.show_abilityless_units();
    let expand_variants = editor.expand_variants();
    let filter = use_memo(move || {
        let race = *active_race.read();
        let modes = *unit_modes.read();
        let query = search_query.read().clone();
        let field = *search_field.read();
        let visibility = CatalogVisibility {
            include_abilityless: *show_abilityless_units.read(),
            expand_variants: *expand_variants.read(),
        };
        UnitFilterQuery::new(race, modes, query, field, visibility)
    });
    // The database pass is the expensive part, so it hangs off the filter rather
    // than off the individual signals: an edit that leaves the filter equal (a
    // keystroke the debounce has not committed yet) must not re-scan.
    let listing = use_memo(move || {
        let current_filter = filter.read().clone();
        current_filter.answer()
    });
    let service = UnitCatalogService::new(filter, listing);
    use_context_provider(|| service);
    service
}
