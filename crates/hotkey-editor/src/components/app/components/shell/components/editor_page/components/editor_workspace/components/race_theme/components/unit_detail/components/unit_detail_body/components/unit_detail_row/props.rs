use super::components::unit_command_grids::UnitCommandGridsProps;
use super::components::unit_tile_override::UnitTileOverrideProps;
use dioxus::prelude::*;

/// The grids-and-override row: the command grids beside the override panel.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailRowProps {
    pub grids: UnitCommandGridsProps,
    pub tile_override: UnitTileOverrideProps,
}
