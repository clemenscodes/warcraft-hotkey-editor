use super::components::unit_card::UnitCardProps;
use super::components::unit_category_heading::UnitCategoryHeadingProps;
use super::logic::UnitCategoryHeadingInputs;
use super::props::UnitCategorySectionProps;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::CatalogVisibility;
use warcraft_keybinds::{UnitCategoryListing, UnitCategoryRequest};

/// The section's shaped view: its heading (with the collapse toggle) and the unit
/// cards to draw when expanded.
pub(super) struct UnitCategorySectionModel {
    pub(super) heading: UnitCategoryHeadingProps,
    pub(super) is_collapsed: bool,
    pub(super) cards: Vec<UnitCardProps>,
}

/// Reads the race, mode, search, visibility, and collapsed state from context, shapes
/// the heading, and queries this category's units. The catalog walk is memoized on
/// race, mode, category, query, search field, and visibility — not on the active
/// selection — so selecting a unit elsewhere in the list does not re-walk the catalog
/// for every section.
pub(super) fn use_unit_category_section(
    props: &UnitCategorySectionProps,
) -> UnitCategorySectionModel {
    let category_kind = props.category_kind;
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let collapsed_categories = editor.collapsed_categories();
    let is_collapsed = collapsed_categories.read().contains(&category_kind);
    let heading_inputs = UnitCategoryHeadingInputs {
        category_kind,
        is_collapsed,
        collapsed_categories,
    };
    let heading = UnitCategoryHeadingProps::from(heading_inputs);
    let race = *navigation.active_race().read();
    let mode = *navigation.unit_mode().read();
    let query = navigation.search_query().read().clone();
    let search_field = *editor.search_field().read();
    let show_abilityless_units = *editor.show_abilityless_units().read();
    let expand_variants = *editor.expand_variants().read();
    let visibility = CatalogVisibility::new(show_abilityless_units, expand_variants);
    let category_listing_memo = use_memo(use_reactive!(|(
        race,
        mode,
        category_kind,
        query,
        search_field,
        visibility,
    )| {
        let category_request =
            UnitCategoryRequest::new(race, mode, category_kind, query, search_field, visibility);
        UnitCategoryListing::resolve(&category_request)
    }));
    let category_listing = category_listing_memo();
    let entries = category_listing.into_entries();
    let cards = entries
        .into_iter()
        .map(|entry| {
            let icon_path = entry.icon_database_path().map(IconUrl::from_database_path);
            let unit_id = entry.unit_id();
            let display_name = entry.display_name().to_owned();
            let unit_kind = entry.unit_kind();
            UnitCardProps {
                unit_id,
                display_name,
                icon_path,
                unit_kind,
            }
        })
        .collect();
    UnitCategorySectionModel {
        heading,
        is_collapsed,
        cards,
    }
}
