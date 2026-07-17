pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::SearchDialogBodyView;

use crate::components::app::components::shell::components::shared::category_scroll::CategoryScroll;
use crate::components::app::components::shell::components::shared::unit_list_search::UnitListSearch;
use components::search_dialog_filters::SearchDialogFilters;
use dioxus::prelude::*;
use model::SearchDialogBodyModel;
use presentation::use_search_dialog_body;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchDialogBody(props: SearchDialogBodyModel) -> Element {
    let SearchDialogBodyModel {} = props;
    let presentation = use_search_dialog_body();
    let search_value = presentation.search_value;
    let on_input = presentation.on_input;
    let on_keydown = presentation.on_keydown;
    let search_placeholder = presentation.search_placeholder;
    let groups = presentation.groups;
    // Configure, then search, then read: the filters narrow what a search even
    // covers, so asking for the words first and the scope afterwards is backwards.
    // The search field's own scope (unit name xor ability) rides inside the search
    // component, because it is part of the question being typed, not a filter.
    rsx! {
        div {
            class: CLASS,
            SearchDialogFilters {}
            UnitListSearch {
                value: search_value,
                placeholder: search_placeholder,
                on_input,
                on_keydown,
            }
            CategoryScroll {
                groups,
            }
        }
    }
}

assert_component!(SearchDialogBody);
