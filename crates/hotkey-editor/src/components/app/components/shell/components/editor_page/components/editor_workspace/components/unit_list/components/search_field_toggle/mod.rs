pub mod components;
mod data;
mod logic;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::search_field_button::SearchFieldButton;
use logic::SearchFieldToggleModel;
use style::CLASS;

pub use props::SearchFieldToggleProps;

assert_component!(SearchFieldToggle);

/// The Unit/Ability toggle that chooses which field the search box matches against.
#[component]
pub fn SearchFieldToggle(props: SearchFieldToggleProps) -> Element {
    let SearchFieldToggleModel {
        unit_button,
        ability_button,
    } = SearchFieldToggleModel::from(&props);
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
