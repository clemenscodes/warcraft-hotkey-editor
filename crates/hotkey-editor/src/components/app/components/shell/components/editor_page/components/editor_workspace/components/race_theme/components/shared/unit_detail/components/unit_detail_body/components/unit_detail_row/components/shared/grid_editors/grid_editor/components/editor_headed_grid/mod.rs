pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::GridHeading;
use components::editor_grid::{EditorGrid, EditorGridProps};
use dioxus::prelude::*;
pub use props::EditorHeadedGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EditorHeadedGrid);

/// A captioned editor grid: the shared `GridHeading` above the interactive
/// `EditorGrid`. Owns `.editor-headed-grid`, which stacks the two and establishes
/// the query container the tiles size against. Purely presentational — `GridEditor`
/// builds the finished interactive tiles and hands them down; this just draws them.
#[component]
pub fn EditorHeadedGrid(props: EditorHeadedGridProps) -> Element {
    let heading = props.heading;
    let grid = EditorGridProps::from(&props);
    rsx! {
        div { class: CLASS,
            GridHeading { heading }
            EditorGrid { ..grid }
        }
    }
}
