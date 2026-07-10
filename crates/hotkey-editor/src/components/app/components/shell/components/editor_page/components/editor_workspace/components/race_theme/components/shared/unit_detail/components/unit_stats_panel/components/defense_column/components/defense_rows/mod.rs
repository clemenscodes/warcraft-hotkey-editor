pub mod components;
mod props;
mod style;

use components::armor_row::ArmorRow;
use components::defense_matchup_row::DefenseMatchupRow;
use components::defense_type_row::DefenseTypeRow;
use components::effective_hit_points_row::EffectiveHitPointsRow;
use components::evasion_row::EvasionRow;
use dioxus::prelude::*;
use props::DefenseRowsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The defense column's stat rows stacked beside its icon: the armor, defense-type, and
/// effective-hit-points rows, the guarded evasion row, and the defender's matchup grid.
/// It names its rows directly — each row owns its own look.
#[component]
pub fn DefenseRows(props: DefenseRowsProps) -> Element {
    let armor = props.armor;
    let defense_type = props.defense_type;
    let effective_hit_points = props.effective_hit_points;
    let evasion = props.evasion;
    rsx! {
        div {
            class: CLASS,
            ArmorRow { value: armor }
            DefenseTypeRow { value: defense_type }
            EffectiveHitPointsRow { value: effective_hit_points }
            EvasionRow { value: evasion }
            DefenseMatchupRow { defense_type }
        }
    }
}

assert_component!(DefenseRows);
