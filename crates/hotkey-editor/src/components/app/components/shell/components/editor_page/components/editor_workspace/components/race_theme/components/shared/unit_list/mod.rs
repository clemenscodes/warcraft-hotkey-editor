pub mod components;
mod model;
mod presentation;
mod state;
mod style;

use components::catalog_visibility_toggle::CatalogVisibilityToggle;
use components::category_scroll::CategoryScroll;
use components::mobile_category_tabs::MobileCategoryTabs;
use components::search_field_toggle::SearchFieldToggle;
use components::unit_list_search::UnitListSearch;
use dioxus::prelude::*;
use model::UnitListModel;
use presentation::use_unit_list;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::UnitKind;

/// The stable per-category key string used to identify a category's row in the
/// mobile tabs and the category sections (a Dioxus list `key`).
pub(super) fn unit_kind_key(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Hero => "hero",
        UnitKind::Soldier => "soldier",
        UnitKind::Worker => "worker",
        UnitKind::Building => "building",
    }
}

/// The unit sidebar: search, catalog toggles, mobile category tabs, and the
/// scrollable category sections. Its root owns only the panel class; the tab row
/// and the scroll region are their own child components, each fed by conversion
/// from the composed hook.
#[component]
pub fn UnitList() -> Element {
    let UnitListModel {
        search_value,
        search_placeholder,
        on_input,
        on_keydown,
        mobile_categories,
        category_kinds,
    } = use_unit_list();
    rsx! {
        aside {
            class: CLASS,
            SearchFieldToggle {
            


            }
            CatalogVisibilityToggle {
            


            }
            UnitListSearch {
                value: search_value,
                placeholder: search_placeholder,
                on_input,
                on_keydown,
            }
            MobileCategoryTabs {
                tabs: mobile_categories,
            }
            CategoryScroll {
                sections: category_kinds,
            }
        }
    }
}

assert_component!(UnitList);
