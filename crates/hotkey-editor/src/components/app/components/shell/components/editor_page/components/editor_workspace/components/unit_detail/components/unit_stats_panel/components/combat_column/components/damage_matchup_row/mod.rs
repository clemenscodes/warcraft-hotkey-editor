pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::attack_matchup_cell::AttackMatchupCell;
use dioxus::prelude::*;
use logic::cells;
pub use props::DamageMatchupRowProps;
use style::CLASS;
assert_component!(DamageMatchupRow);

/// The attacker's damage matchup grid.
#[component]
pub fn DamageMatchupRow(props: DamageMatchupRowProps) -> Element {
    let cells = cells(&props);
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                AttackMatchupCell { ..cell }
            }
        }
    }
}
