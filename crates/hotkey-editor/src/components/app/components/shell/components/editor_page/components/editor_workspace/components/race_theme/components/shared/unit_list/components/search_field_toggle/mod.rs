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

/// The Unit/Ability toggle that chooses which field the search box matches against.
/// It reads and writes the search-field selection from editor context, so it needs
/// no props: the buttons it renders are built from the context signal directly.
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
