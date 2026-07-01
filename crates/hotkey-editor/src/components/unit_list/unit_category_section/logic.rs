use super::super::unit_card::UnitCardProps;
use super::super::unit_kind_data_attr;
use super::props::UnitCategorySectionProps;
use super::unit_category_heading::UnitCategoryHeadingProps;
use crate::model::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_database::UnitCatalog;

/// The section's shaped view: its heading (with the collapse toggle) and the unit
/// cards to draw when expanded.
pub(super) struct UnitCategorySectionModel {
    pub(super) heading: UnitCategoryHeadingProps,
    pub(super) is_collapsed: bool,
    pub(super) cards: Vec<UnitCardProps>,
}

impl From<&UnitCategorySectionProps> for UnitCategorySectionModel {
    fn from(props: &UnitCategorySectionProps) -> Self {
        let is_collapsed = props.is_collapsed;
        let heading = UnitCategoryHeadingProps::from(props);
        let cards = unit_cards(props);
        Self {
            heading,
            is_collapsed,
            cards,
        }
    }
}

impl From<&UnitCategorySectionProps> for UnitCategoryHeadingProps {
    fn from(props: &UnitCategorySectionProps) -> Self {
        let label = props.category_label.clone();
        let kind_attr = unit_kind_data_attr(props.category_kind);
        let is_collapsed = props.is_collapsed;
        let category_kind = props.category_kind;
        let mut collapsed_categories = props.collapsed_categories;
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
            kind_attr,
            is_collapsed,
            on_toggle,
        }
    }
}

/// Queries the catalog for this category and adapts each entry into a finished
/// unit card, marking the active one as selected.
fn unit_cards(props: &UnitCategorySectionProps) -> Vec<UnitCardProps> {
    let search_active = !props.query.is_empty();
    let query_option = Some(props.query.as_str());
    let race_option = if search_active {
        None
    } else {
        Some(props.race)
    };
    let mode_option = if search_active {
        None
    } else {
        Some(props.mode)
    };
    let category_option = Some(props.category_kind);
    let entries = UnitCatalog::entries_for(
        race_option,
        mode_option,
        category_option,
        query_option,
        props.search_field,
        props.visibility,
    );
    entries
        .into_iter()
        .map(|entry| {
            let entry_object = entry.warcraft_object();
            let display_name = entry_object
                .names()
                .first()
                .copied()
                .unwrap_or("(unnamed)")
                .to_owned();
            let icon_path = entry_object
                .icons()
                .first()
                .copied()
                .map(IconUrl::from_database_path);
            let unit_id = entry.unit_id().to_owned();
            let unit_kind = entry.unit_kind();
            let is_selected = props.active_unit_id.as_deref() == Some(entry.unit_id());
            let race = props.race;
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
        .collect()
}
