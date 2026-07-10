pub mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::GridHeadingProps;
use components::unit_command_grids::UnitCommandGrids;
use components::unit_override_panel::{UnitOverridePanel, UnitOverridePanelProps};
use dioxus::prelude::*;
pub use props::UnitDetailRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitDetailRow);

/// The command grids beside the override panel (headed by "Hotkey override"). It owns
/// the override panel column directly.
#[component]
pub fn UnitDetailRow(props: UnitDetailRowProps) -> Element {
    let heading = GridHeadingProps::from(&props);
    let tile_override = props.tile_override.clone();
    let panel = UnitOverridePanelProps {
        heading,
        tile_override,
    };
    rsx! {
        div {
            class: CLASS,
            UnitCommandGrids { ..props.grids }
            UnitOverridePanel { ..panel }
        }
    }
}
