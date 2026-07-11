pub mod components;
mod data;
mod model;
mod view;

pub use view::DefenseMatchupRowView;
mod style;

use components::defense_matchup::DefenseMatchup;
use data::ALL_ATTACK_TYPES;
use dioxus::prelude::*;
use model::DefenseMatchupRowModel;
use style::CLASS;
use tw_macro::assert_component;

/// The defender's matchup grid.
#[component]
pub fn DefenseMatchupRow(props: DefenseMatchupRowModel) -> Element {
    let defense_type = props.defense_type;
    rsx! {
        div {
            class: CLASS,
            for attack_type in ALL_ATTACK_TYPES {
                DefenseMatchup { attack_type, defense_type }
            }
        }
    }
}

assert_component!(DefenseMatchupRow);
