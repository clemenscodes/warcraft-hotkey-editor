pub mod components;
mod style;

use crate::services::navigation::context::use_view_navigation;
use components::human_race_theme::HumanRaceTheme;
use components::neutral_race_theme::NeutralRaceTheme;
use components::nightelf_race_theme::NightelfRaceTheme;
use components::orc_race_theme::OrcRaceTheme;
use components::undead_race_theme::UndeadRaceTheme;
use components::unit_detail::UnitDetail;
use components::unit_list::UnitList;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::Race;

#[component]
pub fn RaceTheme() -> Element {
    let race = *use_view_navigation().active_race().read();
    // The unit list and the detail stay mounted across a race switch; only the
    // race's marker changes, and the theme's style reads it to recolour them.
    rsx! {
        div {
            class: CLASS,
            match race {
                Race::Human => rsx! {
                    HumanRaceTheme {}
                },
                Race::Orc => rsx! {
                    OrcRaceTheme {}
                },
                Race::Undead => rsx! {
                    UndeadRaceTheme {}
                },
                Race::Nightelf => rsx! {
                    NightelfRaceTheme {}
                },
                Race::Neutral => rsx! {
                    NeutralRaceTheme {}
                },
            }
            UnitList {}
            UnitDetail {}
        }
    }
}

assert_component!(RaceTheme);
