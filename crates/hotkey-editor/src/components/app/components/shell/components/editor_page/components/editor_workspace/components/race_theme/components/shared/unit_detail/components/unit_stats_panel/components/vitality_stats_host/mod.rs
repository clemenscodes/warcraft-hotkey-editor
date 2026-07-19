pub mod components;
mod model;
mod view;

pub use view::VitalityStatsHostView;
mod style;

use components::vitality_stats::VitalityStats;
use dioxus::prelude::*;
use model::VitalityStatsHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn VitalityStatsHost(props: VitalityStatsHostModel) -> Element {
    let hit_points = props.hit_points;
    let hit_points_regen = props.hit_points_regen;
    let mana = props.mana;
    let mana_regen = props.mana_regen;
    rsx! {
        div {
            class: CLASS,
            VitalityStats {
                hit_points,
                hit_points_regen,
                mana,
                mana_regen,
            }
        }
    }
}

assert_component!(VitalityStatsHost);
