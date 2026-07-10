pub mod components;
mod props;
mod style;

use components::unit_position_conflict_grid::UnitPositionConflictGrid;
use components::unit_position_detail_header::UnitPositionDetailHeader;
use dioxus::prelude::*;
use props::FilledUnitPositionDetailProps;
use style::CLASS;
use tw_macro::assert_component;

/// The populated position-collision detail pane: the selected unit's header over its
/// position-conflict cards.
#[component]
pub fn FilledUnitPositionDetail(props: FilledUnitPositionDetailProps) -> Element {
    let unit_view = props.unit_view;
    let unit = unit_view.unit().clone();
    let count = unit_view.collision_count();
    let unit_id = unit.unit_id();
    let conflicts = unit_view.conflicts().to_vec();
    rsx! {
        section {
            class: CLASS,
            UnitPositionDetailHeader {
                unit,
                count,
            }
            UnitPositionConflictGrid {
                conflicts,
                unit_id,
            }
        }
    }
}

assert_component!(FilledUnitPositionDetail);
