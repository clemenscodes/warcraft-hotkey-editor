pub mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid_heading::{
    GridHeading, GridHeadingProps,
};
use components::unit_command_grids::UnitCommandGrids;
use components::unit_tile_override::UnitTileOverride;
use dioxus::prelude::*;
pub use props::UnitDetailRowProps;
use style::{CLASS, PANEL};
use tw_macro::assert_component;
assert_component!(UnitDetailRow);

/// The command grids beside the override panel (headed by "Hotkey override"). It owns
/// the override panel column directly.
#[component]
pub fn UnitDetailRow(props: UnitDetailRowProps) -> Element {
    let heading = GridHeadingProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            UnitCommandGrids { ..props.grids }
            aside {
                class: PANEL,
                GridHeading { ..heading }
                UnitTileOverride { ..props.tile_override }
            }
        }
    }
}
