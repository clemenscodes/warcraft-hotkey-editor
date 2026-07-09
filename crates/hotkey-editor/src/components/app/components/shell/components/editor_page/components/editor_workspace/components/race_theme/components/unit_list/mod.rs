pub mod components;
mod hooks;
mod logic;
mod props;
mod state;
mod style;

use components::catalog_visibility_toggle::CatalogVisibilityToggle;
use components::mobile_category_tab::MobileCategoryTab;
use components::search_field_toggle::SearchFieldToggle;
use components::unit_category_section::UnitCategorySection;
use components::unit_list_search::UnitListSearch;
use dioxus::prelude::*;
use hooks::use_unit_list;
use style::{CLASS, SCROLL, TABS, TRACK};
use tw_macro::assert_component;
use warcraft_api::UnitKind;
assert_component!(UnitList);

/// The stable `data-unit-kind` attribute for a category, shared by the mobile
/// tabs, the category sections, and the list's own active-category attribute.
pub(super) fn unit_kind_data_attr(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Hero => "hero",
        UnitKind::Soldier => "soldier",
        UnitKind::Worker => "worker",
        UnitKind::Building => "building",
    }
}

/// The unit sidebar: search, catalog toggles, mobile category tabs, and the
/// scrollable category sections. It owns its tab row, scroll region, and section
/// track directly. Every child is fed by conversion from the composed hook.
#[component]
pub fn UnitList() -> Element {
    let model = use_unit_list();
    rsx! {
        aside {
            class: CLASS,
            "data-active-category": model.active_category_attr,
            "data-search-active": model.search_active,
            SearchFieldToggle {}
            CatalogVisibilityToggle {}
            UnitListSearch { ..model.search }
            nav {
                class: TABS,
                role: "tablist",
                aria_label: "Unit categories",
                for tab in model.mobile_tabs {
                    MobileCategoryTab { key: "{unit_kind_data_attr(tab.kind)}", ..tab }
                }
            }
            div {
                class: SCROLL,
                div {
                    class: TRACK,
                    for section in model.sections {
                        UnitCategorySection {
                            key: "{unit_kind_data_attr(section.category_kind)}",
                            ..section
                        }
                    }
                }
            }
        }
    }
}
