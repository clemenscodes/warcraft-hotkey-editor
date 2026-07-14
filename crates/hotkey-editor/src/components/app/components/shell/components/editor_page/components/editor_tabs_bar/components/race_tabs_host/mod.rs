pub mod components;
mod presentation;
mod style;

use components::race_tabs::RaceTabs;
use dioxus::prelude::*;
use presentation::{RaceTabsInputs, use_race_tabs_host};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceTabsHost() -> Element {
    let RaceTabsInputs {
        active_race,
        on_select,
    } = use_race_tabs_host();
    rsx! {
        div {
            class: CLASS,
            RaceTabs {
                active_race,
                on_select,
            }
        }
    }
}

assert_component!(RaceTabsHost);
