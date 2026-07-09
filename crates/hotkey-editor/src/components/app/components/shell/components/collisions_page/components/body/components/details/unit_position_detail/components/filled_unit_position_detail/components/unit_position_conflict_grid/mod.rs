pub mod components;
mod props;
mod style;

use components::unit_position_conflict_card::UnitPositionConflictCard;
use dioxus::prelude::*;
pub use props::UnitPositionConflictGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitPositionConflictGrid);

/// The scrolling grid of position-collision cards for the selected unit.
#[component]
pub fn UnitPositionConflictGrid(props: UnitPositionConflictGridProps) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                UnitPositionConflictCard { ..card }
            }
        }
    }
}
