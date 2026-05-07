use std::collections::HashSet;

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{UnitCatalog, UnitKindHelpers, UnitMode};

use crate::grid_slot::GridSlotId;

mod category;
mod unit_card;

use category::UnitCategorySection;

const MOBILE_CATEGORY_ORDER: [UnitKind; 4] = [
    UnitKind::Hero,
    UnitKind::Soldier,
    UnitKind::Worker,
    UnitKind::Building,
];

pub(super) fn unit_kind_data_attr(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Hero => "hero",
        UnitKind::Soldier => "soldier",
        UnitKind::Worker => "worker",
        UnitKind::Building => "building",
    }
}

#[component]
pub(crate) fn UnitListPanel(
    active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
    mut search_query: Signal<String>,
    collapsed_categories: Signal<HashSet<UnitKind>>,
) -> Element {
    let mut active_category = use_signal::<UnitKind>(|| UnitKind::Soldier);

    let race = *active_race.read();
    let mode = *unit_mode.read();
    let query_snapshot = search_query.read().clone();
    let search_active = !query_snapshot.is_empty();
    let active_kind = *active_category.read();
    let active_unit_id = selected_unit_id.read().clone();
    let collapsed_snapshot = collapsed_categories.read().clone();

    let category_kinds = {
        let all_entries = UnitCatalog::entries_for(race, mode, None, Some(query_snapshot.as_str()));
        let mut seen: Vec<UnitKind> = Vec::new();
        for entry in all_entries {
            let entry_kind = entry.unit_kind();
            if !seen.contains(&entry_kind) {
                seen.push(entry_kind);
            }
        }
        seen
    };

    rsx! {
        aside {
            class: "unit-list flex flex-col gap-2 overflow-hidden min-w-0 min-h-0",
            "data-active-category": "{unit_kind_data_attr(active_kind)}",
            "data-search-active": "{search_active}",
            div { class: "unit-list-search shrink-0 flex items-center gap-2 p-2 min-w-0",
                input {
                    r#type: "search",
                    placeholder: "Search units…",
                    value: "{search_query}",
                    oninput: move |event| search_query.set(event.value()),
                }
            }
            nav {
                class: "unit-category-tabs",
                role: "tablist",
                aria_label: "Unit categories",
                for kind in MOBILE_CATEGORY_ORDER {
                    {
                        let label = UnitKindHelpers::category_label(kind);
                        let is_active = kind == active_kind;
                        let class_name = if is_active {
                            "unit-category-tab active"
                        } else {
                            "unit-category-tab"
                        };
                        rsx! {
                            button {
                                key: "{unit_kind_data_attr(kind)}",
                                class: "{class_name}",
                                role: "tab",
                                r#type: "button",
                                aria_selected: "{is_active}",
                                "data-unit-kind": "{unit_kind_data_attr(kind)}",
                                onclick: move |_| active_category.set(kind),
                                "{label}"
                            }
                        }
                    }
                }
            }
            div { class: "unit-list-scroll grow overflow-y-auto overflow-x-hidden pr-1 flex flex-col min-h-0",
                div { class: "flex flex-col gap-2",
                    for category_kind in category_kinds {
                        {
                            let category_label = UnitKindHelpers::category_label(category_kind).to_owned();
                            let is_collapsed = collapsed_snapshot.contains(&category_kind);
                            let query_for_section = query_snapshot.clone();
                            let active_unit_id_for_section = active_unit_id.clone();
                            rsx! {
                                UnitCategorySection {
                                    key: "{unit_kind_data_attr(category_kind)}",
                                    category_kind,
                                    category_label,
                                    is_collapsed,
                                    collapsed_categories,
                                    race,
                                    mode,
                                    query: query_for_section,
                                    active_unit_id: active_unit_id_for_section,
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
    }
}
