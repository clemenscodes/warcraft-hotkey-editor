use dioxus::prelude::*;
use warcraft_api::CatalogVisibility;

use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::UnitCatalogService;
use crate::services::unit_catalog::queries::unit_filter_query::UnitFilterQuery;
use crate::services::unit_catalog::queries::unit_roster_query::UnitRosterQuery;

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
    let search_race_scope = editor.search_race_scope();
    let filter = use_memo(move || {
        let race = *active_race.read();
        let modes = *unit_modes.read();
        let query = search_query.read().clone();
        let field = *search_field.read();
        let visibility = CatalogVisibility {
            include_abilityless: *show_abilityless_units.read(),
            expand_variants: *expand_variants.read(),
        };
        let scope = search_race_scope.read().clone();
        UnitFilterQuery::new(race, modes, query, field, visibility, scope)
    });
    // The database pass is the expensive part, so it hangs off the filter rather
    // than off the individual signals: an edit that leaves the filter equal (a
    // keystroke the debounce has not committed yet) must not re-scan.
    let listing = use_memo(move || {
        let current_filter = filter.read().clone();
        current_filter.answer()
    });
    // The roster is the whole game in canonical order, narrowed by nothing but
    // the visibility toggles, so it does not re-scan when the race or mode
    // changes — only when a visibility toggle does. The mobile pager walks it.
    let roster = use_memo(move || {
        let visibility = CatalogVisibility {
            include_abilityless: *show_abilityless_units.read(),
            expand_variants: *expand_variants.read(),
        };
        let query = UnitRosterQuery::new(visibility);
        query.answer()
    });
    let service = UnitCatalogService::new(filter, listing, roster);
    use_context_provider(|| service);
    service
}
