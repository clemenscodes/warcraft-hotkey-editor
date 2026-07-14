pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CombatStatsView;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::combat_rows::CombatRows;
use dioxus::prelude::*;
use model::CombatStatsModel;
use presentation::CombatFigures;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CombatStats(props: CombatStatsModel) -> Element {
    let Some(attack) = props.attack else {
        return rsx! {};
    };
    let CombatFigures {
        icon_src,
        icon_alt,
        damage,
        range,
        speed,
        damage_per_second,
        attack_type,
    } = CombatFigures::from(&attack);
    rsx! {
        div {
            class: CLASS,
            StatIconFrame {
                src: icon_src,
                alt: icon_alt,
            }
            CombatRows {
                damage,
                range,
                speed,
                damage_per_second,
                attack_type,
            }
        }
    }
}

assert_component!(CombatStats);
