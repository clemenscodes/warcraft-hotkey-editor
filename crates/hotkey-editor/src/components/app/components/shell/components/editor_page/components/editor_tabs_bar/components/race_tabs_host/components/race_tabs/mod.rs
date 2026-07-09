pub mod components;
mod logic;
mod props;
mod style;

use crate::services::focus::context::use_focus_coordinator;
use components::human_race_tab::HumanRaceTab;
use components::neutral_race_tab::NeutralRaceTab;
use components::nightelf_race_tab::NightelfRaceTab;
use components::orc_race_tab::OrcRaceTab;
use components::undead_race_tab::UndeadRaceTab;
use dioxus::prelude::*;
use logic::RaceTabBindings;
pub use props::{RaceTabBinding, RaceTabsProps};
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RaceTabs);

/// The race tabs: one banner tab per supported race. It is the consumer of the active
/// race — it reads it once to mark each tab active and bakes the select handler into
/// each tab's handlers — so only a finished `RaceTabBinding` flows to each per-race
/// wrapper, never the navigation itself. Each wrapper adds only its own colours on top.
#[component]
pub fn RaceTabs(props: RaceTabsProps) -> Element {
    let focus = use_focus_coordinator();
    let RaceTabBindings {
        human,
        orc,
        nightelf,
        undead,
        neutral,
    } = RaceTabBindings::build(&props, focus);
    rsx! {
        nav {
            class: CLASS,
            HumanRaceTab { ..human }
            OrcRaceTab { ..orc }
            NightelfRaceTab { ..nightelf }
            UndeadRaceTab { ..undead }
            NeutralRaceTab { ..neutral }
        }
    }
}
