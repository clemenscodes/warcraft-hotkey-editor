pub mod components;
mod model;
mod presentation;
mod view;

pub use view::PagerCardRaceThemeView;

use components::human_pager_card_theme::HumanPagerCardTheme;
use components::neutral_pager_card_theme::NeutralPagerCardTheme;
use components::nightelf_pager_card_theme::NightelfPagerCardTheme;
use components::orc_pager_card_theme::OrcPagerCardTheme;
use components::undead_pager_card_theme::UndeadPagerCardTheme;
use dioxus::prelude::*;
use model::PagerCardRaceThemeModel;
use presentation::use_pager_card_race_theme;
use warcraft_api::Race;

#[component]
pub fn PagerCardRaceTheme(props: PagerCardRaceThemeModel) -> Element {
    let unit_id = props.unit_id;
    let race = use_pager_card_race_theme(&props);
    match race {
        Race::Human => rsx! {
            HumanPagerCardTheme {
                unit_id,
            }
        },
        Race::Orc => rsx! {
            OrcPagerCardTheme {
                unit_id,
            }
        },
        Race::Undead => rsx! {
            UndeadPagerCardTheme {
                unit_id,
            }
        },
        Race::Nightelf => rsx! {
            NightelfPagerCardTheme {
                unit_id,
            }
        },
        Race::Neutral => rsx! {
            NeutralPagerCardTheme {
                unit_id,
            }
        },
    }
}
