mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::tile_face::{
    TileFace, TileFaceProps,
};
use dioxus::prelude::*;
use logic::EditorTileChrome;
pub use props::GridEditorTileProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(GridEditorTile);

/// The interactive command tile: the connected Host that wraps the presentational
/// `TileFace` painter and owns all interaction — focus, drag state, the cursor, and
/// every event handler. The painter draws the tile; this wrapper is the drag/click
/// target and layers the drag-over ring, dragging-source ghost, and focus ring over it.
#[component]
pub fn GridEditorTile(props: GridEditorTileProps) -> Element {
    let face = TileFaceProps::from(&props);
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
            TileFace { ..face }
        }
    }
}
