pub mod components;
mod logic;
mod props;
mod style;

use components::tile_override_panel::TileOverridePanel;
use components::unit_command_grids::UnitCommandGrids;
use components::unit_tile_override::UnitTileOverride;
use crate::assert_component;
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid_heading::{
    GridHeading, GridHeadingProps,
};
use dioxus::prelude::*;
pub use props::UnitDetailRowProps;
use style::CLASS;
assert_component!(UnitDetailRow);

/// The command grids beside the override panel (headed by "Hotkey override").
#[component]
pub fn UnitDetailRow(props: UnitDetailRowProps) -> Element {
    let heading = GridHeadingProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            UnitCommandGrids { ..props.grids }
            TileOverridePanel {
                GridHeading { ..heading }
                UnitTileOverride { ..props.tile_override }
            }
        }
    }
}
