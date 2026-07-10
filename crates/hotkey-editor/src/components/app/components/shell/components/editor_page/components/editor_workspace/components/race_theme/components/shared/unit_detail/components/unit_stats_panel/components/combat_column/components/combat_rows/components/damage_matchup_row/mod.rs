pub mod components;
mod data;
mod props;
mod style;

use components::attack_matchup::AttackMatchup;
use data::DISPLAYED_DEFENSE_TYPES;
use dioxus::prelude::*;
use props::DamageMatchupRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// The attacker's damage matchup grid.
#[component]
pub fn DamageMatchupRow(props: DamageMatchupRowProps) -> Element {
    let attack_type = props.attack_type;
    rsx! {
        div {
            class: CLASS,
            for defense_type in DISPLAYED_DEFENSE_TYPES {
                AttackMatchup { defense_type, attack_type }
            }
        }
    }
}

assert_component!(DamageMatchupRow);
