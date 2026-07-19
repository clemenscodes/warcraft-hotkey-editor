pub mod components;
mod model;
mod view;

pub use view::DefenseStatsHostView;
mod style;

use components::defense_stats::DefenseStats;
use dioxus::prelude::*;
use model::DefenseStatsHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DefenseStatsHost(props: DefenseStatsHostModel) -> Element {
    let armor = props.armor;
    let defense_type = props.defense_type;
    let effective_hit_points = props.effective_hit_points;
    let evasion = props.evasion;
    rsx! {
        div {
            class: CLASS,
            DefenseStats {
                armor,
                defense_type,
                effective_hit_points,
                evasion,
            }
        }
    }
}

assert_component!(DefenseStatsHost);
