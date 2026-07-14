pub mod components;
mod model;
mod view;

pub use view::UnitPositionConflictGridView;
mod style;

use components::unit_position_conflict_card::UnitPositionConflictCard;
use dioxus::prelude::*;
use model::UnitPositionConflictGridModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitPositionConflictGrid(props: UnitPositionConflictGridModel) -> Element {
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
