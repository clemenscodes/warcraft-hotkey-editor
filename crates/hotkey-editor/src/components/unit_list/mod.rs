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

#[component]
pub(crate) fn UnitListPanel(
    active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    mut selected_unit_id: Signal<Option<String>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
    mut search_query: Signal<String>,
    collapsed_categories: Signal<HashSet<UnitKind>>,
) -> Element {
    let state = UnitListState::new(
        active_race,
        unit_mode,
        search_query,
        selected_unit_id,
        collapsed_categories,
    );
    let active_category_signal = state.active_category();
    let handle_search_input = move |event: Event<FormData>| search_query.set(event.value());

    rsx! {
        aside {
            class: "group flex flex-col gap-2 overflow-hidden min-w-0 min-h-0 max-[700px]:max-h-[32rem] min-[701px]:max-[1099px]:sticky min-[701px]:max-[1099px]:top-4 min-[701px]:max-[1099px]:max-h-[calc(100dvh-16rem)] [@media(min-width:701px)_and_(max-height:900px)]:max-h-none min-[1100px]:absolute min-[1100px]:top-0 min-[1100px]:left-0 min-[1100px]:w-[var(--main-sidebar-w)] min-[1100px]:h-full",
            "data-active-category": "{unit_kind_data_attr(state.active_kind())}",
            "data-search-active": "{state.search_active()}",
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
                for kind in MOBILE_CATEGORY_ORDER {
                    {
                        let kind_attr = unit_kind_data_attr(kind);
                        let is_active = kind == state.active_kind();
                        rsx! {
                            MobileCategoryTab {
                                key: "{kind_attr}",
                                kind,
                                is_active,
                                active_category: active_category_signal,
                            }
                        }
                    }
                }
            }
            div {
                class: "grow overflow-y-auto overflow-x-hidden pr-1 flex flex-col min-h-0 [scrollbar-width:thin] [scrollbar-color:rgba(255,206,99,0)_transparent] transition-[scrollbar-color] duration-200 group-hover:[scrollbar-color:rgba(255,206,99,0.45)_transparent] hover:[scrollbar-color:rgba(255,206,99,0.45)_transparent] focus-within:[scrollbar-color:rgba(255,206,99,0.45)_transparent]",
                div { class: "flex flex-col gap-2",
                    for category_kind in state.category_kinds().to_owned() {
                        {
                            let category_label = UnitKindHelpers::category_label(category_kind).to_owned();
                            let is_collapsed = state.collapsed_snapshot().contains(&category_kind);
                            let query_for_section = state.query_snapshot().to_owned();
                            let active_unit_id_for_section = state.active_unit_id().map(str::to_owned);
                            rsx! {
                                UnitCategorySection {
                                    key: "{unit_kind_data_attr(category_kind)}",
                                    category_kind,
                                    category_label,
                                    is_collapsed,
                                    collapsed_categories,
                                    race: state.race(),
                                    mode: state.mode(),
                                    query: query_for_section,
                                    active_unit_id: active_unit_id_for_section,
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
    }
}
