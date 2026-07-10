pub mod components;
mod props;
mod style;

use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
use props::IslandConflictGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling grid of conflict cards for the selected island.
#[component]
pub fn IslandConflictGrid(props: IslandConflictGridProps) -> Element {
    let conflicts = props.conflicts;
    rsx! {
        div {
            class: CLASS,
            for conflict in conflicts {
                IslandConflictCard { conflict }
            }
        }
    }
}

assert_component!(IslandConflictGrid);
