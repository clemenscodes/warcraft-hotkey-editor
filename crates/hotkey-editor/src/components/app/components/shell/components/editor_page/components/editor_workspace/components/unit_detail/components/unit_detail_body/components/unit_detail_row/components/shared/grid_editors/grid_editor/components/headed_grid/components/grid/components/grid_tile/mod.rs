pub mod components;
mod kind;
mod logic;
mod props;
mod state;

use components::empty_tile::{EmptyTile, EmptyTileProps};
use components::filled_tile::{FilledTile, FilledTileProps};
use dioxus::prelude::*;
pub use kind::PlainTileKind;
pub use logic::TileChrome;
pub use props::GridTileProps;
pub use state::GridTileState;

/// A command-grid slot. A pure dispatcher: from the slot's state it renders the
/// occupied tile (`FilledTile`) or the empty one (`EmptyTile`). The occupancy
/// decision lives in the `TryFrom`/`From` conversions, so the body only guards and
/// renders — no computation.
use tw_macro::assert_component;
assert_component!(GridTile);
#[component]
pub fn GridTile(props: GridTileProps) -> Element {
    if let Ok(filled) = FilledTileProps::try_from(&props) {
        return rsx! {
            FilledTile { ..filled }
        };
    }
    let empty = EmptyTileProps::from(&props);
    rsx! {
        EmptyTile { ..empty }
    }
}
