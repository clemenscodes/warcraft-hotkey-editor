pub mod components;
mod logic;
mod props;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::armor_row::ArmorRow;
use components::defense_matchup_row::DefenseMatchupRow;
use components::defense_type_row::DefenseTypeRow;
use components::effective_hit_points_row::EffectiveHitPointsRow;
use components::evasion_row::EvasionRow;
use dioxus::prelude::*;
use logic::DefenseFigures;
pub use props::DefenseColumnProps;
use style::{CLASS, ROWS};
use tw_macro::assert_component;
assert_component!(DefenseColumn);

/// The defense column: the defense-type icon beside the armor/defense-type/effective
/// hit points rows, the guarded evasion row, and the defender's matchup grid, laid
/// into the `defense` grid area. Always present; every unit has defense figures. It
/// names its rows directly — each row owns its own look.
#[component]
pub fn DefenseColumn(props: DefenseColumnProps) -> Element {
    let DefenseFigures {
        defense_icon,
        armor,
        defense_type,
        effective_hit_points,
        evasion,
    } = DefenseFigures::from(&props);
    rsx! {
        div {
            class: CLASS,
            StatIconFrame { ..defense_icon }
            div {
                class: ROWS,
                ArmorRow { value: armor }
                DefenseTypeRow { value: defense_type }
                EffectiveHitPointsRow { value: effective_hit_points }
                EvasionRow { value: evasion }
                DefenseMatchupRow { defense_type }
            }
        }
    }
}
