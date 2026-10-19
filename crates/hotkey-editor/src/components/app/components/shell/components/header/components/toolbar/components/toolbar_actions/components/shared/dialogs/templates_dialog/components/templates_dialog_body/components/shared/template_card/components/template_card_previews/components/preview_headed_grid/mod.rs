pub mod components;
mod model;
mod view;

pub use view::PreviewHeadedGridView;
mod style;

use components::preview_grid::PreviewGrid;
use components::preview_grid_heading::PreviewGridHeading;
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
            PreviewGridHeading {
                heading,
            }
            PreviewGrid {
                tiles,
            }
        }
    }
}

assert_component!(PreviewHeadedGrid);
