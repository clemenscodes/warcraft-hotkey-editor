mod style;

use super::catalog_visibility_toggle::CatalogVisibilityToggle;
use super::category_filters::CategoryFilters;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::UnitKind;

#[component]
pub fn UnitControls(tabs: Vec<UnitKind>) -> Element {
    rsx! {
        div {
            class: CLASS,
            CatalogVisibilityToggle {}
            CategoryFilters {
                tabs,
            }
        }
    }
}

assert_component!(UnitControls);

