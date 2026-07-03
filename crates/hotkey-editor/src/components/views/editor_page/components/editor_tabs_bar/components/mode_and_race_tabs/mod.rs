pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::mode_tabs::{ModeTabs, ModeTabsProps};
use components::race_tabs::{RaceTabs, RaceTabsProps};
use dioxus::prelude::*;
pub use props::ModeAndRaceTabsProps;
use style::CLASS;
assert_component!(ModeAndRaceTabs);

/// The unit-navigation band: the mode column (Melee / Campaign) beside the race
/// tabs. Both children are fed by conversion from this component's props.
#[component]
pub fn ModeAndRaceTabs(props: ModeAndRaceTabsProps) -> Element {
    let mode_tabs = ModeTabsProps::from(&props);
    let race_tabs = RaceTabsProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            ModeTabs { ..mode_tabs }
            RaceTabs { ..race_tabs }
        }
    }
}
