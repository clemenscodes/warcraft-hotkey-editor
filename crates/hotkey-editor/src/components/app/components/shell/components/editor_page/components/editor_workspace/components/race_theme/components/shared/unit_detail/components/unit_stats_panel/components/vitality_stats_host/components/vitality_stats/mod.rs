pub mod components;
mod model;
mod view;

pub use view::VitalityStatsView;
mod style;

use components::hit_points_regen_row::HitPointsRegenRow;
use components::hit_points_row::HitPointsRow;
use components::mana_regen_row::ManaRegenRow;
use components::mana_row::ManaRow;
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
            HitPointsRow {
                value: hit_points,
            }
            HitPointsRegenRow {
                value: hit_points_regen,
            }
            ManaRow {
                value: mana,
            }
            ManaRegenRow {
                value: mana_regen,
            }
        }
    }
}

assert_component!(VitalityStats);
