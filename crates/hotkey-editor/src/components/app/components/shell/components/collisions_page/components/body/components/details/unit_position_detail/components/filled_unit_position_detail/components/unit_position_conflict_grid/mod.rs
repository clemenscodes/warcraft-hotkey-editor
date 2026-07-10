pub mod components;
mod props;
mod view;

pub use view::UnitPositionConflictGridView;
mod style;

use components::unit_position_conflict_card::UnitPositionConflictCard;
use dioxus::prelude::*;
use props::UnitPositionConflictGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// The scrolling grid of position-collision cards for the selected unit.
#[component]
pub fn UnitPositionConflictGrid(props: UnitPositionConflictGridProps) -> Element {
    let conflicts = props.conflicts;
    let unit_id = props.unit_id;
    rsx! {
        div {
            class: CLASS,
            for conflict in conflicts {
                UnitPositionConflictCard {
                    conflict,
                    unit_id,
                }
            }
        }
    }
}

assert_component!(UnitPositionConflictGrid);
