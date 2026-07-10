pub mod components;
mod props;
mod view;

pub use view::EditorGridView;
mod style;

use components::grid_editor_tile::{EditorTile, GridEditorTile};
use dioxus::prelude::*;
use props::EditorGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// The interactive command grid: the three-by-four square of editor tiles. A pure
/// tile renderer that encodes the grid shape (shared with the preview and mini
/// grids via the same utility values) and renders each finished `GridEditorTile`.
/// It owns no behavior — `GridEditor` builds the tiles with their handlers.
#[component]
pub fn EditorGrid(props: EditorGridProps) -> Element {
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            for EditorTile {
                coordinate,
                icon,
                label,
                hotkey,
                badge_state,
                state,
                is_dragging_source,
                is_drag_over,
                is_focusable,
                draggable,
                onkeydown,
                onpointerdown,
                onpointermove,
                onpointerup,
                onpointercancel,
                onlostpointercapture,
                onclick,
                ondoubleclick,
            } in tiles {
                GridEditorTile {
                    coordinate,
                    icon,
                    label,
                    hotkey,
                    badge_state,
                    state,
                    is_dragging_source,
                    is_drag_over,
                    is_focusable,
                    draggable,
                    onkeydown,
                    onpointerdown,
                    onpointermove,
                    onpointerup,
                    onpointercancel,
                    onlostpointercapture,
                    onclick,
                    ondoubleclick,
                }
            }
        }
    }
}

assert_component!(EditorGrid);
