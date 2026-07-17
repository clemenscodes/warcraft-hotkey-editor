pub mod components;
mod model;
mod view;

pub use view::RaceChipThemeView;

use components::human_race_chip_theme::HumanRaceChipTheme;
use components::neutral_race_chip_theme::NeutralRaceChipTheme;
use components::nightelf_race_chip_theme::NightelfRaceChipTheme;
use components::orc_race_chip_theme::OrcRaceChipTheme;
use components::undead_race_chip_theme::UndeadRaceChipTheme;
use dioxus::prelude::*;
use model::RaceChipThemeModel;
use tw_macro::assert_component;
use warcraft_api::Race;

/// Publishes the chip's `--race-color`, so the chip below can stay race-agnostic
/// and wear the colour through a plain `var()`. The race is a closed,
/// game-defined set, so it is five thin components and a match, not a lookup.
#[component]
pub fn RaceChipTheme(props: RaceChipThemeModel) -> Element {
    let race = props.race;
    let label = props.label;
    let active = props.active;
    let on_pick = props.on_pick;
    match race {
        Race::Human => rsx! { HumanRaceChipTheme { label, active, on_pick } },
        Race::Orc => rsx! { OrcRaceChipTheme { label, active, on_pick } },
        Race::Nightelf => rsx! { NightelfRaceChipTheme { label, active, on_pick } },
        Race::Undead => rsx! { UndeadRaceChipTheme { label, active, on_pick } },
        Race::Neutral => rsx! { NeutralRaceChipTheme { label, active, on_pick } },
    }
}

assert_component!(RaceChipTheme);
