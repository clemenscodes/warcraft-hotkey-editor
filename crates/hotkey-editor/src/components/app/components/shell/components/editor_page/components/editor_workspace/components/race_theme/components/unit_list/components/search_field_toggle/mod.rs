pub mod components;
mod data;
mod logic;
mod style;

use crate::services::editor_state::context::use_editor_state;
use components::search_field_button::SearchFieldButton;
use dioxus::prelude::*;
use logic::SearchFieldToggleModel;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(SearchFieldToggle);

/// The Unit/Ability toggle that chooses which field the search box matches against.
/// It reads and writes the search-field selection from editor context, so it needs
/// no props: the buttons it renders are built from the context signal directly.
#[component]
pub fn SearchFieldToggle() -> Element {
    let search_field = use_editor_state().search_field();
    let SearchFieldToggleModel {
        unit_button,
        ability_button,
    } = SearchFieldToggleModel::from(search_field);
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Search by",
            SearchFieldButton { ..unit_button }
            SearchFieldButton { ..ability_button }
        }
    }
}
