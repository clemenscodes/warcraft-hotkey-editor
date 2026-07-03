pub mod components;
mod data;
mod logic;
mod props;

use crate::components::tile_override::{TileOverride, TileOverrideProps};
use components::tile_override_empty::{TileOverrideEmpty, TileOverrideEmptyProps};
use dioxus::prelude::*;
pub use props::UnitTileOverrideProps;

/// The override slot: the override card for the selected tile, or the empty prompt
/// when nothing is selected. A pure dispatcher with no class of its own.
#[component]
pub fn UnitTileOverride(props: UnitTileOverrideProps) -> Element {
    if props.detail.is_none() {
        let empty = TileOverrideEmptyProps {
            message: data::EMPTY_PROMPT.to_string(),
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
