pub mod components;
mod model;
mod view;

pub use view::CaptionedEditorGridView;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::GridHeading;
use components::editor_grid::EditorGrid;
use dioxus::prelude::*;
use model::CaptionedEditorGridModel;
use style::CLASS;
use tw_macro::assert_component;

/// A captioned editor grid: the shared `GridHeading` above the interactive
/// `EditorGrid`. Owns `.captioned-editor-grid`, which stacks the two and establishes
/// the query container the tiles size against. Purely presentational — `GridEditor`
/// builds the finished interactive tiles and hands them down; this just draws them.
#[component]
pub fn CaptionedEditorGrid(props: CaptionedEditorGridModel) -> Element {
    let heading = props.heading;
    let tiles = props.tiles;
    rsx! {
        div {
            class: CLASS,
            GridHeading {
                heading,
            }
            EditorGrid {
                tiles,
            }
        }
    }
}

assert_component!(CaptionedEditorGrid);
