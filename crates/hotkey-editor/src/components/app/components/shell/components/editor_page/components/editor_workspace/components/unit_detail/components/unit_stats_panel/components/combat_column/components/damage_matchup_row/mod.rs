pub mod components;
mod logic;
mod props;
mod style;

use components::attack_matchup::AttackMatchup;
use dioxus::prelude::*;
use logic::cells;
pub use props::DamageMatchupRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DamageMatchupRow);

/// The attacker's damage matchup grid.
#[component]
pub fn DamageMatchupRow(props: DamageMatchupRowProps) -> Element {
    let cells = cells(&props);
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                AttackMatchup { ..cell }
            }
        }
    }
}
