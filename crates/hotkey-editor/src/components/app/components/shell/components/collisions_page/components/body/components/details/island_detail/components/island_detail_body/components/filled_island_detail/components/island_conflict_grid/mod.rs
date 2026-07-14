pub mod components;
mod model;
mod view;

pub use view::IslandConflictGridView;
mod style;

use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
use model::IslandConflictGridModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandConflictGrid(props: IslandConflictGridModel) -> Element {
    let conflicts = props.conflicts;
    rsx! {
        div {
            class: CLASS,
            for conflict in conflicts {
                IslandConflictCard {
                    conflict,
                }
            }
        }
    }
}

assert_component!(IslandConflictGrid);
