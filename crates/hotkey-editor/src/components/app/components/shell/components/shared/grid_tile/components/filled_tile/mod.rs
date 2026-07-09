pub mod components;
mod props;
mod state;
mod style;

use super::super::TileChrome;
use components::ability_fill::{AbilityFill, AbilityFillProps};
use components::command_fill::{CommandFill, CommandFillProps};
use components::selection_ring::{SelectionRing, SelectionRingProps};
use components::tile_icon::{TileIcon, TileIconProps};
use components::tile_label::{TileLabel, TileLabelProps};
use dioxus::prelude::*;
pub use props::FilledTileProps;
pub use state::FilledTileKind;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FilledTile);

/// An occupied command tile. Purely presentational: it draws the ability icon (or its
/// text fallback) over a per-kind background fill, themes its accent from the owning
/// unit's race, and — when selected — mounts the `SelectionRing` whose presence turns
/// the tile's own border gold. It knows nothing of hotkeys, focus, or dragging;
/// `GridEditorTile` layers all interaction on top of this base tile.
#[component]
pub fn FilledTile(props: FilledTileProps) -> Element {
    let ability_fill = AbilityFillProps::from(&props);
    let command_fill = CommandFillProps::from(&props);
    let selection_ring = SelectionRingProps::from(&props);
    let icon = TileIconProps::from(&props);
    let label = TileLabelProps::from(&props);
    let TileChrome { row, column } = props.chrome;
    rsx! {
        div {
            class: CLASS,
            "data-grid-row": row,
            "data-grid-col": column,
            AbilityFill { ..ability_fill }
            CommandFill { ..command_fill }
            SelectionRing { ..selection_ring }
            TileIcon { ..icon }
            TileLabel { ..label }
        }
    }
}
