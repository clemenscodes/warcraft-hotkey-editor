pub mod components;
mod data;
mod kinds;
mod logic;
mod props;

use super::shared::stat_column::{StatColumn, StatColumnKind};
use super::shared::stat_icon_frame::StatIconFrame;
use super::shared::stat_row::StatRow;
use super::shared::stat_rows::StatRows;
use components::defense_matchup_row::DefenseMatchupRow;
use components::evasion_row::EvasionRow;
use dioxus::prelude::*;
use logic::DefenseRows;
pub use props::DefenseColumnProps;

/// The defense column: the defense-type icon beside the armor/defense-type/effective
/// hit points rows, the guarded evasion row, and the defender's matchup grid. Always
/// present; every unit has defense figures.
#[component]
pub fn DefenseColumn(props: DefenseColumnProps) -> Element {
    let DefenseRows {
        defense_icon,
        armor_row,
        defense_type_row,
        effective_hit_points_row,
        evasion,
        defense_type,
    } = DefenseRows::from(&props);
    rsx! {
        StatColumn {
            kind: StatColumnKind::Defense,
            with_icon: true,
            StatIconFrame { ..defense_icon }
            StatRows {
                StatRow { ..armor_row }
                StatRow { ..defense_type_row }
                StatRow { ..effective_hit_points_row }
                EvasionRow { evasion }
                DefenseMatchupRow { defense_type }
            }
        }
    }
}
