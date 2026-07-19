pub mod components;
mod model;
mod view;

pub use view::CombatStatsHostView;
mod style;

use components::combat_stats::CombatStats;
use dioxus::prelude::*;
use model::CombatStatsHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CombatStatsHost(props: CombatStatsHostModel) -> Element {
    let attack = props.attack;
    rsx! {
        div {
            class: CLASS,
            CombatStats {
                attack,
            }
        }
    }
}

assert_component!(CombatStatsHost);
