pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitStatsPanelView;
pub mod stat_icon;
mod style;

use components::attributes_stats_host::AttributesStatsHost;
use components::combat_stats_host::CombatStatsHost;
use components::defense_stats_host::DefenseStatsHost;
use components::vitality_stats_host::VitalityStatsHost;
use dioxus::prelude::*;
use model::UnitStatsPanelModel;
use presentation::{UnitStatsPanelPresentation, use_unit_stats_panel};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitStatsPanel(props: UnitStatsPanelModel) -> Element {
    let UnitStatsPanelPresentation {
        hit_points,
        hit_points_regen,
        mana,
        mana_regen,
        attack,
        armor,
        defense_type,
        effective_hit_points,
        evasion,
        hero,
    } = use_unit_stats_panel(&props);
    rsx! {
        div {
            class: CLASS,
            VitalityStatsHost {
                hit_points,
                hit_points_regen,
                mana,
                mana_regen,
            }
            CombatStatsHost {
                attack,
            }
            DefenseStatsHost {
                armor,
                defense_type,
                effective_hit_points,
                evasion,
            }
            AttributesStatsHost {
                hero,
            }
        }
    }
}

assert_component!(UnitStatsPanel);
