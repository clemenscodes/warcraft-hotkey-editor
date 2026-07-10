pub mod components;
mod hooks;
mod logic;
mod props;
mod state;
mod style;

use components::catalog_visibility_toggle::CatalogVisibilityToggle;
use components::category_scroll::CategoryScroll;
use components::mobile_category_tabs::MobileCategoryTabs;
use components::search_field_toggle::SearchFieldToggle;
use components::unit_list_search::UnitListSearch;
use dioxus::prelude::*;
use hooks::use_unit_list;
use style::CLASS;
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
/// scrollable category sections. Its root owns only the panel class; the tab row
/// and the scroll region are their own child components, each fed by conversion
/// from the composed hook.
#[component]
pub fn UnitList() -> Element {
    let model = use_unit_list();
    rsx! {
        aside {
            class: CLASS,
            SearchFieldToggle {}
            CatalogVisibilityToggle {}
            UnitListSearch { ..model.search }
            MobileCategoryTabs { ..model.tabs }
            CategoryScroll { ..model.scroll }
        }
    }
}
