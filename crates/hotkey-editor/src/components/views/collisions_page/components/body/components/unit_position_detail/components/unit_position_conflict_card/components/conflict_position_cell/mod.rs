mod props;
mod style;

use crate::assert_component;
use crate::components::views::collisions_page::components::body::components::mini_grid::{
    MiniGrid, MiniGridProps,
};
use dioxus::prelude::*;
pub use props::ConflictPositionCellProps;
use style::CLASS;
assert_component!(ConflictPositionCell);

#[component]
pub fn ConflictPositionCell(props: ConflictPositionCellProps) -> Element {
    let is_top = props.is_top;
    let mini_grid = MiniGridProps {
        collision_column: props.collision_column,
        collision_row: props.collision_row,
    };
    rsx! {
        span {
            class: CLASS,
            "data-top": is_top,
            MiniGrid { ..mini_grid }
        }
    }
}
