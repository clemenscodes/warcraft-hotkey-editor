pub mod binding;
pub mod components;
mod model;
mod view;

pub use view::RaceTabBannerView;

use components::human_race_tab::HumanRaceTab;
use components::neutral_race_tab::NeutralRaceTab;
use components::nightelf_race_tab::NightelfRaceTab;
use components::orc_race_tab::OrcRaceTab;
use components::undead_race_tab::UndeadRaceTab;
use dioxus::prelude::*;
use model::RaceTabBannerModel;
use tw_macro::assert_component;
use warcraft_api::Race;

#[component]
pub fn RaceTabBanner(props: RaceTabBannerModel) -> Element {
    let RaceTabBannerModel {
        race,
        is_active,
        label,
        onclick,
        onkeydown,
    } = props;
    match race {
        Race::Human => rsx! {
            HumanRaceTab {
                is_active,
                label,
                onclick,
                onkeydown,
            }
        },
        Race::Orc => rsx! {
            OrcRaceTab {
                is_active,
                label,
                onclick,
                onkeydown,
            }
        },
        Race::Nightelf => rsx! {
            NightelfRaceTab {
                is_active,
                label,
                onclick,
                onkeydown,
            }
        },
        Race::Undead => rsx! {
            UndeadRaceTab {
                is_active,
                label,
                onclick,
                onkeydown,
            }
        },
        Race::Neutral => rsx! {
            NeutralRaceTab {
                is_active,
                label,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(RaceTabBanner);
