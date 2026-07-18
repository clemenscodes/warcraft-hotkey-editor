pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::SearchDialogBodyView;

use crate::components::app::components::shell::components::shared::category_scroll::CategoryScroll;
use crate::components::app::components::shell::components::shared::unit_list_search::UnitListSearch;
use components::search_config_button::SearchConfigButton;
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
    let filters_open = presentation.filters_open;
    let filters_label = presentation.filters_label;
    let on_toggle_filters = presentation.on_toggle_filters;
    // Search first, filters folded: the search field is the reason the dialog
    // exists, so it sits at the top and reachable at once. The race, mode and
    // display filters live one tap away behind the config button, which carries a
    // count of how many currently narrow the search.
    rsx! {
        div {
            class: CLASS,
            UnitListSearch {
                value: search_value,
                placeholder: search_placeholder,
                on_input,
                on_keydown,
            }
            SearchConfigButton {
                label: filters_label,
                open: filters_open,
                onclick: on_toggle_filters,
            }
            SearchDialogFilters {
                open: filters_open,
            }
            CategoryScroll {
                groups,
            }
        }
    }
}

assert_component!(SearchDialogBody);
