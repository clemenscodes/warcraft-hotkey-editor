pub mod components;
mod model;
mod presentation;
mod state;
mod style;

use crate::components::app::components::shell::components::shared::catalog_visibility_toggle::CatalogVisibilityToggle;
use crate::components::app::components::shell::components::shared::category_scroll::CategoryScroll;
use crate::components::app::components::shell::components::shared::search_field_toggle::SearchFieldToggle;
use crate::components::app::components::shell::components::shared::unit_list_search::UnitListSearch;
use components::mobile_category_tabs::MobileCategoryTabs;
use dioxus::prelude::*;
use model::UnitListModel;
use presentation::use_unit_list;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitList() -> Element {
    let UnitListModel {
        search_value,
        search_placeholder,
        on_input,
        on_keydown,
        mobile_categories,
        groups,
    } = use_unit_list();
    rsx! {
        aside {
            class: CLASS,
            SearchFieldToggle {}
            CatalogVisibilityToggle {}
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
                groups,
            }
        }
    }
}

assert_component!(UnitList);
