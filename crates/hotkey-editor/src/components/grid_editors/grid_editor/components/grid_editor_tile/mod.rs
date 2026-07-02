pub mod components;
mod kind;
mod logic;
mod props;
mod style;

use crate::assert_component;
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::{
    GridTile, GridTileProps,
};
use components::tile_badge::{TileBadge, TileBadgeProps};
use dioxus::prelude::*;
pub use kind::EditorTileKind;
use logic::EditorTileChrome;
pub use props::GridEditorTileProps;
use style::CLASS;
assert_component!(GridEditorTile);

/// The interactive command tile: the inert base `GridTile` with the hotkey badge
/// layered on top, wrapped in the element that owns all interaction — focus, drag
/// state, and every event handler. The base tile and the badge are siblings under
/// this wrapper; the wrapper is the drag/click target and the badge's positioning
/// and container context.
#[component]
pub fn GridEditorTile(props: GridEditorTileProps) -> Element {
    let base = GridTileProps::from(&props);
    let badge = TileBadgeProps::from(&props);
    let EditorTileChrome {
        tabindex,
        draggable_attribute,
        dragging_source,
        drag_over,
        onkeydown,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
        onclick,
        ondoubleclick,
    } = EditorTileChrome::from(&props);
    rsx! {
        div {
            class: CLASS,
            tabindex,
            "data-draggable": draggable_attribute,
            "data-dragging-source": dragging_source,
            "data-drag-over": drag_over,
            onkeydown,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            onclick,
            ondoubleclick,
            GridTile { ..base }
            TileBadge { ..badge }
        }
    }
}
