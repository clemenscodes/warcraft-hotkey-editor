pub mod components;
mod style;

use components::also_include_group::AlsoIncludeGroup;
use components::find_units_by_group::FindUnitsByGroup;
use components::mode_group::ModeGroup;
use components::races_group::RacesGroup;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchDialogFilters() -> Element {
    rsx! {
        div {
            class: CLASS,
            FindUnitsByGroup {}
            ModeGroup {}
            AlsoIncludeGroup {}
            RacesGroup {}
        }
    }
}

assert_component!(SearchDialogFilters);
