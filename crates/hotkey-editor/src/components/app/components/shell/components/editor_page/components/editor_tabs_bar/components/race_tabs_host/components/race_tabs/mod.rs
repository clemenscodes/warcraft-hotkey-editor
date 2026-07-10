pub mod components;
mod logic;
mod props;
mod view;

pub use view::RaceTabsView;
mod style;

use components::human_race_tab::HumanRaceTab;
use components::neutral_race_tab::NeutralRaceTab;
use components::nightelf_race_tab::NightelfRaceTab;
use components::orc_race_tab::OrcRaceTab;
use components::undead_race_tab::UndeadRaceTab;
use dioxus::prelude::*;
use logic::RaceTabBindings;
use props::RaceTabBinding;
use props::RaceTabsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The race tabs: one banner tab per supported race. It is the consumer of the active
/// race — it reads it once to mark each tab active and bakes the select handler into
/// each tab's handlers — so only a finished `RaceTabBinding` flows to each per-race
/// wrapper, never the navigation itself. Each wrapper adds only its own colours on top.
#[component]
pub fn RaceTabs(props: RaceTabsProps) -> Element {
    let RaceTabBindings {
        human,
        orc,
        nightelf,
        undead,
        neutral,
    } = RaceTabBindings::build(&props);
    let RaceTabBinding {
        is_active: human_is_active,
        label: human_label,
        onclick: human_onclick,
        onkeydown: human_onkeydown,
    } = human;
    let RaceTabBinding {
        is_active: orc_is_active,
        label: orc_label,
        onclick: orc_onclick,
        onkeydown: orc_onkeydown,
    } = orc;
    let RaceTabBinding {
        is_active: nightelf_is_active,
        label: nightelf_label,
        onclick: nightelf_onclick,
        onkeydown: nightelf_onkeydown,
    } = nightelf;
    let RaceTabBinding {
        is_active: undead_is_active,
        label: undead_label,
        onclick: undead_onclick,
        onkeydown: undead_onkeydown,
    } = undead;
    let RaceTabBinding {
        is_active: neutral_is_active,
        label: neutral_label,
        onclick: neutral_onclick,
        onkeydown: neutral_onkeydown,
    } = neutral;
    rsx! {
        nav {
            class: CLASS,
            HumanRaceTab {
                is_active: human_is_active,
                label: human_label,
                onclick: human_onclick,
                onkeydown: human_onkeydown,
            }
            OrcRaceTab {
                is_active: orc_is_active,
                label: orc_label,
                onclick: orc_onclick,
                onkeydown: orc_onkeydown,
            }
            NightelfRaceTab {
                is_active: nightelf_is_active,
                label: nightelf_label,
                onclick: nightelf_onclick,
                onkeydown: nightelf_onkeydown,
            }
            UndeadRaceTab {
                is_active: undead_is_active,
                label: undead_label,
                onclick: undead_onclick,
                onkeydown: undead_onkeydown,
            }
            NeutralRaceTab {
                is_active: neutral_is_active,
                label: neutral_label,
                onclick: neutral_onclick,
                onkeydown: neutral_onkeydown,
            }
        }
    }
}

assert_component!(RaceTabs);
