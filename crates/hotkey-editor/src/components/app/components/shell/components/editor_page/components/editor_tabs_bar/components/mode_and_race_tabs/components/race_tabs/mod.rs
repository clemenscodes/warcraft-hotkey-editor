pub mod components;
mod props;
mod style;

use components::human_race_tab::HumanRaceTab;
use components::neutral_race_tab::NeutralRaceTab;
use components::nightelf_race_tab::NightelfRaceTab;
use components::orc_race_tab::OrcRaceTab;
use components::undead_race_tab::UndeadRaceTab;
use dioxus::prelude::*;
pub use props::RaceTabsProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RaceTabs);

/// The race tabs: one banner tab per supported race, in supported-race order. Each tab
/// is a thin per-race wrapper that publishes its race's banner variables and forwards
/// the shared navigation; this component only lays them out in the nav row and hands the
/// same navigation bundle to every tab.
#[component]
pub fn RaceTabs(props: RaceTabsProps) -> Element {
    rsx! {
        nav {
            class: CLASS,
            HumanRaceTab { ..props }
            OrcRaceTab { ..props }
            NightelfRaceTab { ..props }
            UndeadRaceTab { ..props }
            NeutralRaceTab { ..props }
        }
    }
}
