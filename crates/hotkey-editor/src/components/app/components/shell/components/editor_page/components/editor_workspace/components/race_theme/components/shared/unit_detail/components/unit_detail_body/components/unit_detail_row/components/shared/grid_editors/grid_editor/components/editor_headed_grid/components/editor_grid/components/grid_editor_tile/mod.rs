mod components;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::tile_face::{
    TileFace, TileFaceProps,
};
use components::draggable_marker::{DraggableMarker, DraggableMarkerProps};
use dioxus::prelude::*;
use logic::EditorTileChrome;
pub use props::GridEditorTileProps;
use style::CLASS;
use tw_macro::assert_component;

/// The interactive command tile: the connected Host that wraps the presentational
/// `TileFace` painter and owns all interaction — focus, the cursor, and every event
/// handler. The painter draws the tile and, from the drag flags this Host forwards into
/// it, mounts the dragging-source ghost and drag-over ring itself; this wrapper mounts
/// the `DraggableMarker` (the grab-cursor and off-state-picker signal that replaced the
/// `data-draggable` attribute) and layers the focus ring over the tile.
#[component]
pub fn GridEditorTile(props: GridEditorTileProps) -> Element {
    let face = TileFaceProps::from(&props);
    let draggable_marker = DraggableMarkerProps::from(&props);
    let EditorTileChrome {
        tabindex,
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
            onkeydown,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            onclick,
            ondoubleclick,
            TileFace { ..face }
            DraggableMarker { ..draggable_marker }
        }
    }
}

assert_component!(GridEditorTile);
