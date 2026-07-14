pub mod components;
mod model;
mod presentation;
mod view;

pub use view::DefenseStatsView;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::defense_rows::DefenseRows;
use dioxus::prelude::*;
use model::DefenseStatsModel;
use presentation::DefenseFigures;
use style::CLASS;
use tw_macro::assert_component;

/// The defense column: the defense-type icon beside its stat rows and the defender's
/// matchup grid, laid into the `defense` grid area. Always present; every unit has
/// defense figures.
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
            StatIconFrame {
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
