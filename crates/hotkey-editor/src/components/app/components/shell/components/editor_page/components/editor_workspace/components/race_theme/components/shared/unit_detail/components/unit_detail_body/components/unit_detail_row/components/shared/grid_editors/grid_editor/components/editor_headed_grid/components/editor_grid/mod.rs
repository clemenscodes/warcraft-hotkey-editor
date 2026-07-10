pub mod components;
mod props;
mod style;

use components::grid_editor_tile::GridEditorTile;
use dioxus::prelude::*;
pub use props::EditorGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EditorGrid);

/// The interactive command grid: the three-by-four square of editor tiles. A pure
/// tile renderer that encodes the grid shape (shared with the preview and mini
/// grids via the same utility values) and renders each finished `GridEditorTile`.
/// It owns no behavior — `GridEditor` builds the tiles with their handlers.
#[component]
pub fn EditorGrid(props: EditorGridProps) -> Element {
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            for tile in tiles {
                GridEditorTile { ..tile }
            }
        }
    }
}
