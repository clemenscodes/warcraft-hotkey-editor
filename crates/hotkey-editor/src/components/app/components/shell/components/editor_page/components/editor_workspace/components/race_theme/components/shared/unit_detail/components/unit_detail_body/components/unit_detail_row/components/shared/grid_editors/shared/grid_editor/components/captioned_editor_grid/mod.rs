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
