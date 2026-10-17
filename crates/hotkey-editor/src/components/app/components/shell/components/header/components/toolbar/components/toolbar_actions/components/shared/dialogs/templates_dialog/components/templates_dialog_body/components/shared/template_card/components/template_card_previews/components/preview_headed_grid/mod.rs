pub mod components;
mod model;
mod view;

pub use view::PreviewHeadedGridView;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::GridHeading;
use components::preview_grid::PreviewGrid;
use dioxus::prelude::*;
use model::PreviewHeadedGridModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PreviewHeadedGrid(props: PreviewHeadedGridModel) -> Element {
    let heading = props.heading;
    let tiles = props.tiles;
    rsx! {
        div {
            class: CLASS,
            GridHeading {
                heading,
            }
            PreviewGrid {
                tiles,
            }
        }
    }
}

assert_component!(PreviewHeadedGrid);
