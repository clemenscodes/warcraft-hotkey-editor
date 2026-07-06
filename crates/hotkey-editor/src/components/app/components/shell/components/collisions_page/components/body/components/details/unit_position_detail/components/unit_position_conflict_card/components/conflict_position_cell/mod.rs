mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::{
    MiniGrid, MiniGridProps,
};
use dioxus::prelude::*;
pub use props::ConflictPositionCellProps;
use style::CLASS;
assert_component!(ConflictPositionCell);

#[component]
pub fn ConflictPositionCell(props: ConflictPositionCellProps) -> Element {
    let is_top = props.is_top;
    let coordinate = props.coordinate;
    let mini_grid = MiniGridProps { coordinate };
    rsx! {
        span {
            class: CLASS,
            "data-top": is_top,
            MiniGrid { ..mini_grid }
        }
    }
}
