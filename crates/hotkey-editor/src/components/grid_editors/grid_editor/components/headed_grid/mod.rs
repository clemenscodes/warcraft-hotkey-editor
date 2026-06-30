pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::grid::Grid;
use components::grid_heading::GridHeading;
use style::HEADED_GRID_STYLE_SHEETS;

pub use props::HeadedGridProps;

/// A captioned grid: a heading above the reused `Grid`. Owns `.headed-grid`,
/// which stacks the two. Purely presentational, no behavior; it draws the tiles
/// it is handed. The `GridEditor` and the templates preview both compose it.
#[component]
pub fn HeadedGrid(props: HeadedGridProps) -> Element {
    let heading = props.heading;
    let tiles = props.tiles;
    rsx! {
        document::Stylesheet { href: HEADED_GRID_STYLE_SHEETS }
        div {
            class: "headed-grid",
            GridHeading { heading }
            Grid { tiles }
        }
    }
}
