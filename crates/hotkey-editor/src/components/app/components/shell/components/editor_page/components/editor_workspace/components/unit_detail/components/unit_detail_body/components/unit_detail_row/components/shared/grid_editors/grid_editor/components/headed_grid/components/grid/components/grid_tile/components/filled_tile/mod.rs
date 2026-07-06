pub mod components;
mod props;
mod state;
mod style;

use super::super::TileChrome;
use components::tile_icon::{TileIcon, TileIconProps};
use components::tile_label::{TileLabel, TileLabelProps};
use dioxus::prelude::*;
pub use props::FilledTileProps;
use tw_macro::assert_component;
assert_component!(FilledTile);

/// An occupied command tile. Purely presentational: it draws the ability icon (or
/// its text fallback), themes its accent from the owning unit's race, and reports
/// its selected state and coordinate. It knows nothing of hotkeys, focus, or
/// dragging — `GridEditorTile` layers all interaction on top of this base tile.
#[component]
pub fn FilledTile(props: FilledTileProps) -> Element {
    let icon = TileIconProps::from(&props);
    let label = TileLabelProps::from(&props);
    let class = style::class(props.state);
    let selected = props.selected;
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
            "data-selected": selected,
            TileIcon { ..icon }
            TileLabel { ..label }
        }
    }
}
