pub mod components;
mod model;
mod style;
mod view;

pub use view::SearchDialogFiltersView;

use components::also_include_group::AlsoIncludeGroup;
use components::find_units_by_group::FindUnitsByGroup;
use components::mode_group::ModeGroup;
use components::races_group::RacesGroup;
use dioxus::prelude::*;
use model::SearchDialogFiltersModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchDialogFilters(props: SearchDialogFiltersModel) -> Element {
    let leaving = props.leaving;
    rsx! {
        div {
            class: CLASS,
            aria_hidden: leaving,
            FindUnitsByGroup {}
            ModeGroup {}
            AlsoIncludeGroup {}
            RacesGroup {}
        }
    }
}

assert_component!(SearchDialogFilters);
