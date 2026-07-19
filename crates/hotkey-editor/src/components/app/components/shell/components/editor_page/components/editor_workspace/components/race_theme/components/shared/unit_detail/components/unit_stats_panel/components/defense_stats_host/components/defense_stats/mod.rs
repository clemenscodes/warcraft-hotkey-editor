pub mod components;
mod model;
mod presentation;
mod view;

pub use view::DefenseStatsView;
mod style;

use super::super::super::shared::stat_icon_frame_host::StatIconFrameHost;
use components::defense_rows::DefenseRows;
use dioxus::prelude::*;
use model::DefenseStatsModel;
use presentation::DefenseFigures;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DefenseStats(props: DefenseStatsModel) -> Element {
    let DefenseFigures {
        icon_src,
        icon_alt,
        armor,
        defense_type,
        effective_hit_points,
        evasion,
    } = DefenseFigures::from(&props);
    rsx! {
        div {
            class: CLASS,
            StatIconFrameHost {
                src: icon_src,
                alt: icon_alt,
            }
            DefenseRows {
                armor,
                defense_type,
                effective_hit_points,
                evasion,
            }
        }
    }
}

assert_component!(DefenseStats);
