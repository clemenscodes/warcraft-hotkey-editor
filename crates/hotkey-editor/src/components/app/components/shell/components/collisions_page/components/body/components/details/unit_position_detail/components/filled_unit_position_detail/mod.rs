pub mod components;
mod props;
mod style;

use components::unit_position_conflict_grid::{
    UnitPositionConflictGrid, UnitPositionConflictGridProps,
};
use components::unit_position_detail_header::{
    UnitPositionDetailHeader, UnitPositionDetailHeaderProps,
};
use dioxus::prelude::*;
pub use props::FilledUnitPositionDetailProps;
use style::CLASS;
use tw_macro::assert_component;

/// The populated position-collision detail pane: the selected unit's header over its
/// position-conflict cards.
#[component]
pub fn FilledUnitPositionDetail(props: FilledUnitPositionDetailProps) -> Element {
    let header = UnitPositionDetailHeaderProps::from(&props);
    let grid = UnitPositionConflictGridProps::from(&props);
    rsx! {
        section {
            class: CLASS,
            UnitPositionDetailHeader { ..header }
            UnitPositionConflictGrid { ..grid }
        }
    }
}

assert_component!(FilledUnitPositionDetail);
