pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::grid::Grid;
use components::grid_heading::GridHeading;
use dioxus::prelude::*;
pub use props::HeadedGridProps;
use style::CLASS;
assert_component!(HeadedGrid);

/// A captioned grid: a heading above the reused `Grid`. Owns `.headed-grid`,
/// which stacks the two. Purely presentational, no behavior; it draws the tiles
/// it is handed. The `GridEditor` and the templates preview both compose it.
#[component]
pub fn HeadedGrid(props: HeadedGridProps) -> Element {
    let heading = props.heading;
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            GridHeading { heading }
            Grid { tiles }
        }
    }
}
