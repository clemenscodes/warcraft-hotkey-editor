pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::grid::{Grid, GridTileKind};
use components::grid_heading::GridHeading;
use dioxus::prelude::*;
pub use props::HeadedGridProps;
use style::CLASS;
assert_component!(HeadedGrid);

/// A captioned grid: a heading above the reused `Grid`. Owns `.headed-grid`, which
/// stacks the two. Purely presentational, generic over the [`GridTileKind`] it
/// lays out; it draws the tiles it is handed and has no behavior of its own.
#[component]
pub fn HeadedGrid<B: GridTileKind>(props: HeadedGridProps<B>) -> Element {
    let heading = props.heading;
    let grid = props.grid;
    rsx! {
        div { class: CLASS,
            GridHeading { heading }
            Grid { ..grid }
        }
    }
}
