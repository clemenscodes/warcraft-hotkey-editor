mod category;
mod mobile_category_tab;
mod state;
mod unit_card;

use std::collections::HashSet;
use std::time::Duration;

use dioxus::document;
use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{UnitKindHelpers, UnitMode};

use crate::model::grid::GridSlotId;
use crate::services::focus::modality::FocusModality;

use category::UnitCategorySection;
use mobile_category_tab::MobileCategoryTab;
use state::UnitListState;

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

struct MobileTabEntry {
    kind: UnitKind,
    is_active: bool,
}

impl MobileTabEntry {
    fn kind(&self) -> UnitKind {
        self.kind
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

struct CategorySectionEntry {
    kind: UnitKind,
    label: String,
    is_collapsed: bool,
    query: String,
    active_unit_id: Option<String>,
}

impl CategorySectionEntry {
    fn kind(&self) -> UnitKind {
        self.kind
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn is_collapsed(&self) -> bool {
        self.is_collapsed
    }

    fn query(&self) -> String {
        self.query.clone()
    }

    fn active_unit_id(&self) -> Option<String> {
        self.active_unit_id.clone()
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct UnitListPanelProps {
    pub(crate) active_race: Signal<Race>,
    pub(crate) unit_mode: Signal<UnitMode>,
    pub(crate) selected_unit_id: Signal<Option<String>>,
    pub(crate) selected_slot: Signal<Option<GridSlotId>>,
    pub(crate) search_query: Signal<String>,
    pub(crate) collapsed_categories: Signal<HashSet<UnitKind>>,
}

#[component]
pub(crate) fn UnitListPanel(props: UnitListPanelProps) -> Element {
    let active_race = props.active_race;
    let unit_mode = props.unit_mode;
    let mut selected_unit_id = props.selected_unit_id;
    let mut selected_slot = props.selected_slot;
    let mut search_query = props.search_query;
    let collapsed_categories = props.collapsed_categories;
    let mut raw_query = use_signal(|| search_query.read().clone());
    let mut debounce_gen: Signal<u32> = use_signal(|| 0);
    let state = UnitListState::new(
        active_race,
        unit_mode,
        search_query,
        selected_unit_id,
        collapsed_categories,
    );
    let mut active_category_signal = state.active_category();
    let active_kind = state.active_kind();
    let search_active = state.search_active();
    let race = state.race();
    let mode = state.mode();
    let first_result_id = state.first_result_id().map(str::to_owned);
    let first_result_kind = state.first_result_kind();
    let handle_search_keydown = move |event: Event<KeyboardData>| {
        let event_data = event.data();
        let key = event_data.key();
        let key_string = key.to_string();
        match key_string.as_str() {
            "Escape" => {
                let current_raw = raw_query.read().clone();
                if current_raw.is_empty() {
                    let focus_script = "document.body.setAttribute('data-kb-modality', ''); const card = document.querySelector('.unit-card'); if (card) card.focus();";
                    document::eval(focus_script);
                } else {
                    raw_query.set(String::new());
                    search_query.set(String::new());
                    let current_gen: u32 = *debounce_gen.read();
                    let next_gen = current_gen.wrapping_add(1);
                    debounce_gen.set(next_gen);
                }
            }
            "Enter" => {
                if let (Some(unit_id), Some(unit_kind)) =
                    (first_result_id.clone(), first_result_kind)
                {
                    selected_unit_id.set(Some(unit_id));
                    selected_slot.set(None);
                    active_category_signal.set(unit_kind);
                    FocusModality::after_render(".unit-card.selected, .unit-card");
                }
            }
            _ => {}
        }
    };
    let handle_search_input = move |event: Event<FormData>| {
        let value = event.value();
        raw_query.set(value.clone());
        let current_gen: u32 = *debounce_gen.read();
        let next_gen = current_gen.wrapping_add(1);
        debounce_gen.set(next_gen);
        spawn(async move {
            let delay = Duration::from_millis(150);
            gloo_timers::future::sleep(delay).await;
            let gen_now: u32 = *debounce_gen.read();
            if gen_now == next_gen {
                search_query.set(value);
            }
        });
    };
    let mobile_tab_entries: Vec<MobileTabEntry> = MOBILE_CATEGORY_ORDER
        .iter()
        .map(|&kind| MobileTabEntry {
            kind,
            is_active: kind == active_kind,
        })
        .collect();
    let category_section_entries: Vec<CategorySectionEntry> = state
        .category_kinds()
        .iter()
        .map(|&kind| CategorySectionEntry {
            kind,
            label: UnitKindHelpers::category_label(kind).to_owned(),
            is_collapsed: state.collapsed_snapshot().contains(&kind),
            query: state.query_snapshot().to_owned(),
            active_unit_id: state.active_unit_id().map(str::to_owned),
        })
        .collect();

    rsx! {
        aside {
            class: "unit-list",
            "data-active-category": "{unit_kind_data_attr(active_kind)}",
            "data-search-active": search_active,
            div {
                class: "unit-list-search",
                input {
                    r#type: "search",
                    placeholder: "Search units…",
                    value: raw_query,
                    oninput: handle_search_input,
                    onkeydown: handle_search_keydown,
                }
            }
            nav {
                class: "unit-category-tabs",
                role: "tablist",
                aria_label: "Unit categories",
                for tab in mobile_tab_entries {
                    MobileCategoryTab {
                        key: "{unit_kind_data_attr(tab.kind())}",
                        kind: tab.kind(),
                        is_active: tab.is_active(),
                        active_category: active_category_signal,
                    }
                }
            }
            div {
                class: "unit-list-scroll",
                div { class: "unit-list-track",
                    for section in category_section_entries {
                        UnitCategorySection {
                            key: "{unit_kind_data_attr(section.kind())}",
                            category_kind: section.kind(),
                            category_label: section.label(),
                            is_collapsed: section.is_collapsed(),
                            collapsed_categories,
                            race,
                            mode,
                            query: section.query(),
                            active_unit_id: section.active_unit_id(),
                            selected_unit_id,
                            selected_slot,
                            active_category: active_category_signal,
                        }
                    }
                }
            }
        }
    }
}
