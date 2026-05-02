use std::collections::HashSet;

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};

use crate::domain::grid_slot::GridSlotId;
use crate::domain::icons::IconUrl;
use crate::domain::unit_catalog::UnitCatalog;
use crate::domain::unit_kind::UnitKindHelpers;
use crate::domain::unit_mode::UnitMode;
use crate::focus::modality::FocusModality;

const MOBILE_CATEGORY_ORDER: [UnitKind; 4] = [
    UnitKind::Hero,
    UnitKind::Soldier,
    UnitKind::Worker,
    UnitKind::Building,
];

fn unit_kind_data_attr(kind: UnitKind) -> &'static str {
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
    let units = UnitCatalog::entries_for(race, mode, None, Some(query_snapshot.as_str()));
    let search_active = !query_snapshot.is_empty();
    let active_kind = *active_category.read();

    let active_unit_id = selected_unit_id.read().clone();

    rsx! {
        aside {
            class: "unit-list",
            "data-active-category": "{unit_kind_data_attr(active_kind)}",
            "data-search-active": "{search_active}",
            div { class: "unit-list-search",
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
            div { class: "unit-list-scroll",
                div { class: "unit-list-track",
                    {
                        let collapsed_snapshot = collapsed_categories.read().clone();
                        let mut current_kind: Option<UnitKind> = None;
                        let mut output: Vec<Element> = Vec::new();
                        for entry in units {
                            if Some(entry.unit_kind) != current_kind {
                                current_kind = Some(entry.unit_kind);
                                let category_label = UnitKindHelpers::category_label(entry.unit_kind);
                                let is_collapsed = collapsed_snapshot.contains(&entry.unit_kind);
                                let heading_class = if is_collapsed {
                                    "unit-category-heading collapsed"
                                } else {
                                    "unit-category-heading"
                                };
                                let captured_kind = entry.unit_kind;
                                let mut categories_signal = collapsed_categories;
                                output.push(rsx! {
                                    button {
                                        class: "{heading_class}",
                                        "data-unit-kind": "{unit_kind_data_attr(captured_kind)}",
                                        onclick: move |_| {
                                            let mut categories = categories_signal.write();
                                            if categories.contains(&captured_kind) {
                                                categories.remove(&captured_kind);
                                            } else {
                                                categories.insert(captured_kind);
                                            }
                                        },
                                        span { class: "category-chevron",
                                            if is_collapsed { "\u{25b6}" } else { "\u{25bc}" }
                                        }
                                        "{category_label}"
                                    }
                                });
                            }
                            if collapsed_snapshot.contains(&entry.unit_kind) {
                                continue;
                            }
                            let display_name = entry.warcraft_object.names().first().copied().unwrap_or("(unnamed)");
                            let icon_path = entry.warcraft_object.icons().first().copied().map(IconUrl::from_database_path);
                            let unit_id_for_click = entry.unit_id.clone();
                            let is_selected = active_unit_id.as_deref() == Some(entry.unit_id.as_str());
                            let class_name = if is_selected { "unit-card selected" } else { "unit-card" };
                            let unit_id_label = entry.unit_id.clone();
                            let unit_id_for_keydown = entry.unit_id.clone();
                            let card_kind = entry.unit_kind;
                            output.push(rsx! {
                                button {
                                    class: "{class_name}",
                                    "data-unit-kind": "{unit_kind_data_attr(card_kind)}",
                                    onclick: move |_| {
                                        selected_unit_id.set(Some(unit_id_for_click.clone()));
                                        selected_slot.set(None);
                                        active_category.set(card_kind);
                                    },
                                    onkeydown: move |event| {
                                        let key_value = event.data().key().to_string();
                                        if key_value == " " || key_value == "Enter" {
                                            event.prevent_default();
                                            selected_unit_id.set(Some(unit_id_for_keydown.clone()));
                                            selected_slot.set(None);
                                            active_category.set(card_kind);
                                            FocusModality::after_render(".grid-tile.has-ability.selected, .grid-tile.has-ability");
                                        }
                                    },
                                    if let Some(source) = icon_path {
                                        img { class: "unit-card-icon", src: "{source}", alt: "{display_name}", loading: "lazy", decoding: "async" }
                                    } else {
                                        div { class: "unit-card-icon", }
                                    }
                                    div { class: "unit-card-text",
                                        span { class: "unit-card-name", "{display_name}" }
                                        code { class: "unit-card-id", "{unit_id_label}" }
                                    }
                                }
                            });
                        }
                        rsx! { {output.into_iter()} }
                    }
                }
            }
        }
    }
}
