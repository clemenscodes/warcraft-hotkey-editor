use dioxus::prelude::*;

use super::components::tile_override_id::TileOverrideIdProps;
use super::components::tile_override_name::TileOverrideNameProps;

/// The header text column owns the name heading and the id line beneath it.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideHeaderTextProps {
    pub name: TileOverrideNameProps,
    pub id: TileOverrideIdProps,
}
