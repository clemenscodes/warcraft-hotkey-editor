pub mod components;
mod data;
mod presentation;
mod style;

use crate::services::editor_state::context::use_editor_state;
use components::search_field_button::SearchFieldButton;
use dioxus::prelude::*;
use presentation::SearchFieldToggleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchFieldToggle() -> Element {
    let search_field = use_editor_state().search_field();
    let SearchFieldToggleModel {
        unit_is_active,
        ability_is_active,
        select_unit,
        select_ability,
    } = SearchFieldToggleModel::from(search_field);
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Search by",
            SearchFieldButton {
                label: data::UNIT,
                is_active: unit_is_active,
                on_select: select_unit,
            }
            SearchFieldButton {
                label: data::ABILITY,
                is_active: ability_is_active,
                on_select: select_ability,
            }
        }
    }
}

assert_component!(SearchFieldToggle);
