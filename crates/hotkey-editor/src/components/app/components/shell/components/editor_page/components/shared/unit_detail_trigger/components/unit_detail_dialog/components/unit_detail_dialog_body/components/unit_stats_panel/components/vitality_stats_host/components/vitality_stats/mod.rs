pub mod components;
mod model;
mod view;

pub use view::VitalityStatsView;
mod style;

use components::vitality_table::VitalityTable;
use dioxus::prelude::*;
use model::VitalityStatsModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn VitalityStats(props: VitalityStatsModel) -> Element {
    let hit_points = props.hit_points;
    let hit_points_regen = props.hit_points_regen;
    let mana = props.mana;
    let mana_regen = props.mana_regen;
    rsx! {
        div {
            class: CLASS,
            VitalityTable {
                hit_points,
                hit_points_regen,
                mana,
                mana_regen,
            }
        }
    }
}

assert_component!(VitalityStats);
