pub mod components;
mod model;
mod view;

pub use view::VitalityTableView;
mod style;

use components::hit_points_row::HitPointsRow;
use components::mana_row::ManaRow;
use components::regen_condition_note::RegenConditionNote;
use components::vitality_heading_row::VitalityHeadingRow;
use dioxus::prelude::*;
use model::VitalityTableModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn VitalityTable(props: VitalityTableModel) -> Element {
    let hit_points = props.hit_points;
    let hit_points_regen = props.hit_points_regen;
    let mana = props.mana;
    let mana_regen = props.mana_regen;
    rsx! {
        div {
            class: CLASS,
            VitalityHeadingRow {}
            HitPointsRow {
                value: hit_points,
                regen: hit_points_regen,
            }
            ManaRow {
                value: mana,
                regen: mana_regen,
            }
            RegenConditionNote {
                regen: hit_points_regen,
            }
        }
    }
}

assert_component!(VitalityTable);
