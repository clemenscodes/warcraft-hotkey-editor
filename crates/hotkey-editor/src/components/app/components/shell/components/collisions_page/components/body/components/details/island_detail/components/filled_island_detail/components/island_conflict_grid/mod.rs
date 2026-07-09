pub mod components;
mod props;
mod style;

use components::island_conflict_card::IslandConflictCard;
use dioxus::prelude::*;
pub use props::IslandConflictGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IslandConflictGrid);

/// The scrolling grid of conflict cards for the selected island.
#[component]
pub fn IslandConflictGrid(props: IslandConflictGridProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                IslandConflictCard { ..card }
            }
        }
    }
}
