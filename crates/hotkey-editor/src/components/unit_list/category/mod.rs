mod state;

use std::collections::HashSet;

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{UnitCatalog, UnitMode};

use crate::model::grid::GridSlotId;
use crate::model::icons::IconUrl;

use super::unit_card::UnitCard;
use super::unit_kind_data_attr;
use state::UnitCategoryHeadingClass;

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
pub(super) struct UnitCategorySectionProps {
    pub(super) category_kind: UnitKind,
    pub(super) category_label: String,
    pub(super) is_collapsed: bool,
    pub(super) collapsed_categories: Signal<HashSet<UnitKind>>,
    pub(super) race: Race,
    pub(super) mode: UnitMode,
    pub(super) query: String,
    pub(super) active_unit_id: Option<String>,
    pub(super) selected_unit_id: Signal<Option<String>>,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) active_category: Signal<UnitKind>,
}

#[component]
pub(super) fn UnitCategorySection(props: UnitCategorySectionProps) -> Element {
    let category_kind = props.category_kind;
    let category_label = props.category_label;
    let is_collapsed = props.is_collapsed;
    let mut collapsed_categories = props.collapsed_categories;
    let race = props.race;
    let mode = props.mode;
    let query = props.query;
    let active_unit_id = props.active_unit_id;
    let selected_unit_id = props.selected_unit_id;
    let selected_slot = props.selected_slot;
    let active_category = props.active_category;
    let heading_class = UnitCategoryHeadingClass::compute(is_collapsed);
    let kind_attr = unit_kind_data_attr(category_kind);
    let captured_kind = category_kind;
    let query_str = query.as_str();
    let query_option = Some(query_str);
    let category_option = Some(category_kind);
    let entries = UnitCatalog::entries_for(race, mode, category_option, query_option);
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

    rsx! {
        button {
            class: heading_class,
            "data-unit-kind": kind_attr,
            onclick: toggle_collapse,
            span { class: "text-[0.9rem] inline-flex w-[0.8rem] shrink-0",
                if is_collapsed { "\u{25b6}" } else { "\u{25bc}" }
            }
            {category_label}
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
