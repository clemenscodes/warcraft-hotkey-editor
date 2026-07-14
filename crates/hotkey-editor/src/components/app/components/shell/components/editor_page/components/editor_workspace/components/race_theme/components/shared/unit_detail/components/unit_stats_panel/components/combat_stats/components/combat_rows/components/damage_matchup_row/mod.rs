pub mod components;
mod data;
mod model;
mod view;

pub use view::DamageMatchupRowView;
mod style;

use components::attack_matchup::AttackMatchup;
use data::DISPLAYED_DEFENSE_TYPES;
use dioxus::prelude::*;
use model::DamageMatchupRowModel;
use style::CLASS;
use tw_macro::assert_component;

/// The attacker's damage matchup grid.
#[component]
pub fn DamageMatchupRow(props: DamageMatchupRowModel) -> Element {
    let attack_type = props.attack_type;
    rsx! {
        div {
            class: CLASS,
            for defense_type in DISPLAYED_DEFENSE_TYPES {
                AttackMatchup {
                    defense_type,
                    attack_type,
                }
            }
        }
    }
}

assert_component!(DamageMatchupRow);
