pub mod unit_category_heading;

use super::unit_card::UnitCard;
use super::unit_kind_data_attr;
use crate::model::icons::IconUrl;
use dioxus::prelude::*;
use std::collections::HashSet;
use unit_category_heading::UnitCategoryHeading;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{CatalogVisibility, SearchField, UnitCatalog, UnitMode};
use warcraft_keybinds::GridSlotId;

struct UnitCardEntry {
    key: String,
    unit_id: String,
    display_name: String,
    icon_path: Option<IconUrl>,
    unit_kind: UnitKind,
    is_selected: bool,
}

impl UnitCardEntry {
    fn key(&self) -> &str {
        &self.key
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct UnitCategorySectionProps {
    pub category_kind: UnitKind,
    pub category_label: String,
    pub is_collapsed: bool,
    pub collapsed_categories: Signal<HashSet<UnitKind>>,
    pub race: Race,
    pub mode: UnitMode,
    pub query: String,
    pub search_field: SearchField,
    pub visibility: CatalogVisibility,
    pub active_unit_id: Option<String>,
    pub selected_unit_id: Signal<Option<String>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub active_category: Signal<UnitKind>,
}

#[component]
pub fn UnitCategorySection(props: UnitCategorySectionProps) -> Element {
    let category_kind = props.category_kind;
    let category_label = props.category_label;
    let is_collapsed = props.is_collapsed;
    let mut collapsed_categories = props.collapsed_categories;
    let race = props.race;
    let mode = props.mode;
    let query = props.query;
    let search_field = props.search_field;
    let visibility = props.visibility;
    let active_unit_id = props.active_unit_id;
    let selected_unit_id = props.selected_unit_id;
    let selected_slot = props.selected_slot;
    let active_category = props.active_category;
    let kind_attr = unit_kind_data_attr(category_kind);
    let captured_kind = category_kind;
    let query_str = query.as_str();
    let query_option = Some(query_str);
    let search_active = !query.is_empty();
    let race_option = if search_active { None } else { Some(race) };
    let mode_option = if search_active { None } else { Some(mode) };
    let category_option = Some(category_kind);
    let entries = UnitCatalog::entries_for(
        race_option,
        mode_option,
        category_option,
        query_option,
        search_field,
        visibility,
    );
    let unit_card_entries: Vec<UnitCardEntry> = entries
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
            let key = unit_id.clone();
            let unit_kind = entry.unit_kind();
            let is_selected = active_unit_id.as_deref() == Some(entry.unit_id());
            UnitCardEntry {
                key,
                unit_id,
                display_name,
                icon_path,
                unit_kind,
                is_selected,
            }
        })
        .collect();
    let toggle_collapse = move |_| {
        let mut categories = collapsed_categories.write();
        if categories.contains(&captured_kind) {
            categories.remove(&captured_kind);
        } else {
            categories.insert(captured_kind);
        }
    };
    let toggle_handler = EventHandler::new(toggle_collapse);
    rsx! {
        UnitCategoryHeading {
            label: category_label,
            kind_attr,
            is_collapsed,
            on_toggle: toggle_handler,
        }
        if !is_collapsed {
            for card_entry in unit_card_entries {
                UnitCard {
                    key: "{card_entry.key()}",
                    unit_kind: card_entry.unit_kind,
                    is_selected: card_entry.is_selected,
                    unit_id: card_entry.unit_id,
                    display_name: card_entry.display_name,
                    icon_path: card_entry.icon_path,
                    race,
                    selected_unit_id,
                    selected_slot,
                    active_category,
                }
            }
        }
    }
}
