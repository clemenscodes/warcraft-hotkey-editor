mod props;
mod state;
mod style;

use super::super::TileChrome;
use dioxus::prelude::*;
pub use props::EmptyTileProps;
use tw_macro::assert_component;
assert_component!(EmptyTile);

/// An empty command slot. Purely presentational: it draws its state look, its race
/// accent, and its coordinate attributes. It knows nothing of hotkeys, focus, or
/// dragging — `GridEditorTile` layers all interaction on top of this base tile.
#[component]
pub fn EmptyTile(props: EmptyTileProps) -> Element {
    let class = style::class(props.state);
    let drop_target = props.drop_target;
    let TileChrome {
        race_attribute,
        row,
        column,
    } = props.chrome;
    rsx! {
        div {
            class,
            "data-race": race_attribute,
            "data-grid-row": row,
            "data-grid-col": column,
            "data-drop-target": drop_target,
        }
    }
}
