pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::defense_matchup_cell::DefenseMatchupCell;
use dioxus::prelude::*;
use logic::cells;
pub use props::DefenseMatchupRowProps;
use style::CLASS;
assert_component!(DefenseMatchupRow);

/// The defender's matchup grid.
#[component]
pub fn DefenseMatchupRow(props: DefenseMatchupRowProps) -> Element {
    let cells = cells(&props);
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                DefenseMatchupCell { ..cell }
            }
        }
    }
}
