mod presentation;
mod style;

use crate::components::app::components::shell::components::shared::race_tab_banner::RaceTabBanner;
use dioxus::prelude::*;
use presentation::use_mobile_race_nav_items;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileRaceNav() -> Element {
    let race_items = use_mobile_race_nav_items();
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Races",
            for item in race_items {
                RaceTabBanner {
                    key: "{item.key}",
                    race: item.race,
                    is_active: item.is_active,
                    label: item.label,
                    onclick: item.onclick,
                    onkeydown: item.onkeydown,
                }
            }
        }
    }
}

assert_component!(MobileRaceNav);
