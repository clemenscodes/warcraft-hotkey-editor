mod props;
mod style;

use super::island_mini_grid::{IslandMiniGrid, IslandMiniGridProps};
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictPositionCellProps;
use style::CLASS;
assert_component!(ConflictPositionCell);

#[component]
pub fn ConflictPositionCell(props: ConflictPositionCellProps) -> Element {
    let is_top = props.is_top;
    let mini_grid = IslandMiniGridProps {
        collision_column: props.collision_column,
        collision_row: props.collision_row,
    };
    rsx! {
        span {
            class: CLASS,
            "data-top": is_top,
            IslandMiniGrid { ..mini_grid }
        }
    }
}
