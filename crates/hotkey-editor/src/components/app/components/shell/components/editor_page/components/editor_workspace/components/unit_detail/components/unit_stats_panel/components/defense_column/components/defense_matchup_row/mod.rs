pub mod components;
mod logic;
mod props;
mod style;

use components::defense_matchup::DefenseMatchup;
use dioxus::prelude::*;
use logic::cells;
pub use props::DefenseMatchupRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DefenseMatchupRow);

/// The defender's matchup grid.
#[component]
pub fn DefenseMatchupRow(props: DefenseMatchupRowProps) -> Element {
    let cells = cells(&props);
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                DefenseMatchup { ..cell }
            }
        }
    }
}
