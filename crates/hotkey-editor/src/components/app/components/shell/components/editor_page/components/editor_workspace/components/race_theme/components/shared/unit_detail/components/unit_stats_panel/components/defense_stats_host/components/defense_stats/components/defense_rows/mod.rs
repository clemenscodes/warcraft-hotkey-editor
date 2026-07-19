pub mod components;
mod model;
mod view;

pub use view::DefenseRowsView;
mod style;

use components::armor_row::ArmorRow;
use components::defense_matchup_row::DefenseMatchupRow;
use components::defense_type_row::DefenseTypeRow;
use components::effective_hit_points_row::EffectiveHitPointsRow;
use components::evasion_row::EvasionRow;
use dioxus::prelude::*;
use model::DefenseRowsModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DefenseRows(props: DefenseRowsModel) -> Element {
    let armor = props.armor;
    let defense_type = props.defense_type;
    let effective_hit_points = props.effective_hit_points;
    let evasion = props.evasion;
    rsx! {
        div {
            class: CLASS,
            ArmorRow {
                value: armor,
            }
            DefenseTypeRow {
                value: defense_type,
            }
            EffectiveHitPointsRow {
                value: effective_hit_points,
            }
            EvasionRow {
                value: evasion,
            }
            DefenseMatchupRow {
                defense_type,
            }
        }
    }
}

assert_component!(DefenseRows);
