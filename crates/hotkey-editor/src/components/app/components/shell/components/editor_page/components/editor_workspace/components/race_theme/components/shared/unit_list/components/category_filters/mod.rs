pub mod components;
mod data;
mod model;
mod view;

pub use view::CategoryFiltersView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::unit_kind_key;
use components::category_filter::CategoryFilter;
use dioxus::prelude::*;
use style::CLASS;
use super::shared::filter_group_label::FilterGroupLabel;
use tw_macro::assert_component;

use model::CategoryFiltersModel;

#[component]
pub fn CategoryFilters(props: CategoryFiltersModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Categories",
            FilterGroupLabel { text: data::CATEGORIES_LABEL }
            for kind in props.tabs {
                CategoryFilter {
                    key: "{unit_kind_key(kind)}",
                    kind,
                }
            }
        }
    }
}

assert_component!(CategoryFilters);
