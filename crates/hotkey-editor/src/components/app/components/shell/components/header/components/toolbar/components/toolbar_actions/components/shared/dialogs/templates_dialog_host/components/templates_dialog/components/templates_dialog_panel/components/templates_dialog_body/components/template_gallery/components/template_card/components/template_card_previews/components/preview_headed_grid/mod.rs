pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::GridHeading;
use components::preview_grid::PreviewGrid;
use dioxus::prelude::*;
use props::PreviewHeadedGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// A captioned template preview: the shared `GridHeading` above the read-only
/// `PreviewGrid`. Owns `.preview-headed-grid`, which stacks the two and establishes
/// the query container the tiles size against. Purely presentational.
#[component]
pub fn PreviewHeadedGrid(props: PreviewHeadedGridProps) -> Element {
    let heading = props.heading;
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            GridHeading { heading }
            PreviewGrid { tiles }
        }
    }
}

assert_component!(PreviewHeadedGrid);
