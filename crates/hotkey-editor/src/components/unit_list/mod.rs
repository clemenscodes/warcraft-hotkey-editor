mod category;
mod mobile_category_tab;
mod state;
mod unit_card;

use std::collections::HashSet;

use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};
use warcraft_database::{UnitKindHelpers, UnitMode};

use crate::model::grid::GridSlotId;

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
    let selected_unit_id = props.selected_unit_id;
    let selected_slot = props.selected_slot;
    let mut search_query = props.search_query;
    let collapsed_categories = props.collapsed_categories;
    let state = UnitListState::new(
        active_race,
        unit_mode,
        search_query,
        selected_unit_id,
        collapsed_categories,
    );
    let active_category_signal = state.active_category();
    let active_kind = state.active_kind();
    let search_active = state.search_active();
    let race = state.race();
    let mode = state.mode();
    let handle_search_input = move |event: Event<FormData>| search_query.set(event.value());
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
            class: "group flex flex-col gap-2 overflow-hidden min-w-0 min-h-0 max-[700px]:max-h-[32rem] min-[701px]:max-[1099px]:sticky min-[701px]:max-[1099px]:top-4 min-[701px]:max-[1099px]:max-h-[calc(100dvh-16rem)] [@media(min-width:701px)_and_(max-height:900px)]:max-h-none min-[1100px]:absolute min-[1100px]:top-0 min-[1100px]:left-0 min-[1100px]:w-[var(--main-sidebar-w)] min-[1100px]:h-full",
            "data-active-category": "{unit_kind_data_attr(active_kind)}",
            "data-search-active": search_active,
            div {
                class: "shrink-0 flex items-center gap-2 p-2 min-w-0 bg-[rgba(13,31,61,0.85)] border border-[#1f3d63] rounded-[6px]",
                input {
                    class: "flex-1 min-w-0 w-full bg-[rgba(8,18,35,0.7)] border border-warcraft-blue rounded text-white py-3 px-4 font-[inherit] text-[1.4rem] focus:outline-none focus:border-warcraft-gold focus:shadow-[0_0_6px_rgba(255,206,99,0.4)]",
                    r#type: "search",
                    placeholder: "Search units…",
                    value: search_query,
                    oninput: handle_search_input,
                }
            }
            nav {
                class: "hidden",
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
                class: "grow overflow-y-auto overflow-x-hidden pr-1 flex flex-col min-h-0 [scrollbar-width:thin] [scrollbar-color:rgba(255,206,99,0)_transparent] transition-[scrollbar-color] duration-200 group-hover:[scrollbar-color:rgba(255,206,99,0.45)_transparent] hover:[scrollbar-color:rgba(255,206,99,0.45)_transparent] focus-within:[scrollbar-color:rgba(255,206,99,0.45)_transparent]",
                div { class: "flex flex-col gap-2",
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
