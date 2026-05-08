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

#[component]
pub(super) fn UnitCategorySection(
    category_kind: UnitKind,
    category_label: String,
    is_collapsed: bool,
    mut collapsed_categories: Signal<HashSet<UnitKind>>,
    race: Race,
    mode: UnitMode,
    query: String,
    active_unit_id: Option<String>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
    mut active_category: Signal<UnitKind>,
) -> Element {
    let heading_class = UnitCategoryHeadingClass::compute(is_collapsed);
    let kind_attr = unit_kind_data_attr(category_kind);
    let captured_kind = category_kind;
    let query_str = query.as_str();
    let query_option = Some(query_str);
    let category_option = Some(category_kind);
    let entries = UnitCatalog::entries_for(race, mode, category_option, query_option);
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
            "{category_label}"
        }
        if !is_collapsed {
            for entry in entries {
                {
                    let entry_object = entry.warcraft_object();
                    let display_name = entry_object.names().first().copied().unwrap_or("(unnamed)").to_owned();
                    let icon_path = entry_object.icons().first().copied().map(IconUrl::from_database_path);
                    let entry_unit_id = entry.unit_id().to_owned();
                    let entry_kind = entry.unit_kind();
                    let is_selected = active_unit_id.as_deref() == Some(entry.unit_id());
                    rsx! {
                        UnitCard {
                            key: "{entry_unit_id}",
                            unit_id: entry_unit_id,
                            display_name,
                            icon_path,
                            unit_kind: entry_kind,
                            race,
                            is_selected,
                            selected_unit_id,
                            selected_slot,
                            active_category,
                        }
                    }
                }
            }
        }
    }
}
