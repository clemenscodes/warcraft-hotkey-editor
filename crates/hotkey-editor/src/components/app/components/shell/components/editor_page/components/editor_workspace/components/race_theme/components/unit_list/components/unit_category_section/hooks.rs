use super::components::unit_card::UnitCardProps;
use super::components::unit_category_heading::UnitCategoryHeadingProps;
use super::props::UnitCategorySectionProps;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_keybinds::{UnitCategoryListing, UnitCategoryRequest};

/// The section's shaped view: its heading (with the collapse toggle) and the unit
/// cards to draw when expanded.
pub(super) struct UnitCategorySectionModel {
    pub(super) heading: UnitCategoryHeadingProps,
    pub(super) is_collapsed: bool,
    pub(super) cards: Vec<UnitCardProps>,
}

/// Shapes the heading and queries this category's units. The catalog walk is
/// memoized on race, mode, category, query, search field, and visibility — not
/// on the active selection — so selecting a unit elsewhere in the list does not
/// re-walk the catalog for every section.
pub(super) fn use_unit_category_section(
    props: &UnitCategorySectionProps,
) -> UnitCategorySectionModel {
    let is_collapsed = props.is_collapsed;
    let heading = UnitCategoryHeadingProps::from(props);
    let race = props.race;
    let mode = props.mode;
    let category_kind = props.category_kind;
    let query = props.query.clone();
    let search_field = props.search_field;
    let visibility = props.visibility;
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
            let is_selected = props.active_unit_id == Some(unit_id);
            let display_name = entry.display_name().to_owned();
            let unit_kind = entry.unit_kind();
            let selected_unit_id = props.selected_unit_id;
            let selected_slot = props.selected_slot;
            let active_category = props.active_category;
            UnitCardProps {
                unit_id,
                display_name,
                icon_path,
                unit_kind,
                race,
                is_selected,
                selected_unit_id,
                selected_slot,
                active_category,
            }
        })
        .collect();
    UnitCategorySectionModel {
        heading,
        is_collapsed,
        cards,
    }
}
