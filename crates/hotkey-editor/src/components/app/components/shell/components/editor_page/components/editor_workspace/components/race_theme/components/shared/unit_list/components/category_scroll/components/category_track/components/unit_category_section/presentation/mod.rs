use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::UnitKind;

/// The heading's inputs: which category it heads, whether it is collapsed, and the
/// collapsed-set signal its toggle flips. The section reads the collapsed set from
/// context and hands these to the heading builder.
pub(super) struct UnitCategoryHeadingInputs {
    pub(super) category_kind: UnitKind,
    pub(super) is_collapsed: bool,
    pub(super) collapsed_categories: Signal<HashSet<UnitKind>>,
}

/// The heading's shaped data: its label, collapsed state, and the toggle handler the
/// section hands down to `UnitCategoryHeading` as named fields.
pub(super) struct CategoryHeadingData {
    pub(super) label: String,
    pub(super) is_collapsed: bool,
    pub(super) on_toggle: EventHandler<MouseEvent>,
}

impl From<UnitCategoryHeadingInputs> for CategoryHeadingData {
    fn from(inputs: UnitCategoryHeadingInputs) -> Self {
        let category_kind = inputs.category_kind;
        let label = category_kind.category_label().to_owned();
        let is_collapsed = inputs.is_collapsed;
        let mut collapsed_categories = inputs.collapsed_categories;
        let on_toggle = EventHandler::new(move |_event: MouseEvent| {
            let mut categories = collapsed_categories.write();
            if categories.contains(&category_kind) {
                categories.remove(&category_kind);
            } else {
                categories.insert(category_kind);
            }
        });
        Self {
            label,
            is_collapsed,
            on_toggle,
        }
    }
}
use super::model::UnitCategorySectionModel;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use warcraft_api::CatalogVisibility;
use warcraft_api::UnitCategoryListing;
use warcraft_api::UnitCategoryRequest;
use warcraft_api::WarcraftObjectId;

/// One unit's shaped catalog data for a card: its id, display name, portrait, and kind.
pub(super) struct UnitCardEntry {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) display_name: String,
    pub(super) icon_path: Option<IconUrl>,
    pub(super) unit_kind: UnitKind,
}

/// The section's shaped view: its heading (with the collapse toggle) and the unit
/// cards to draw when expanded.
pub(super) struct UnitCategorySectionPresentation {
    pub(super) heading: CategoryHeadingData,
    pub(super) is_collapsed: bool,
    pub(super) cards: Vec<UnitCardEntry>,
}

/// Reads the race, mode, search, visibility, and collapsed state from context, shapes
/// the heading, and queries this category's units. The catalog walk is memoized on
/// race, mode, category, query, search field, and visibility — not on the active
/// selection — so selecting a unit elsewhere in the list does not re-walk the catalog
/// for every section.
pub(super) fn use_unit_category_section(
    props: &UnitCategorySectionModel,
) -> UnitCategorySectionPresentation {
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
    let heading = CategoryHeadingData::from(heading_inputs);
    let race = *navigation.active_race().read();
    let mode = *navigation.unit_mode().read();
    let query = navigation.search_query().read().clone();
    let search_field = *editor.search_field().read();
    let show_abilityless_units = *editor.show_abilityless_units().read();
    let expand_variants = *editor.expand_variants().read();
    let visibility = CatalogVisibility {
        include_abilityless: show_abilityless_units,
        expand_variants,
    };
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
            UnitCardEntry {
                unit_id,
                display_name,
                icon_path,
                unit_kind,
            }
        })
        .collect();
    UnitCategorySectionPresentation {
        heading,
        is_collapsed,
        cards,
    }
}
