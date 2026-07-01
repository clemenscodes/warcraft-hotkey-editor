mod logic;
mod props;

use super::tile_override_empty::{TileOverrideEmpty, TileOverrideEmptyProps};
use crate::components::tile_override::{TileOverride, TileOverrideProps};
use dioxus::prelude::*;
pub use props::UnitTileOverrideProps;

/// The override slot: the override card for the selected tile, or the empty prompt
/// when nothing is selected. A pure dispatcher with no class of its own.
#[component]
pub fn UnitTileOverride(props: UnitTileOverrideProps) -> Element {
    if props.detail.is_none() {
        let empty = TileOverrideEmptyProps {
            message: "Select a tile in the grid to override its hotkey.".to_string(),
        };
        return rsx! {
            TileOverrideEmpty { ..empty }
        };
    }
    let tile = TileOverrideProps::from(&props);
    rsx! {
        TileOverride { ..tile }
    }
}
