use dioxus::prelude::*;

use super::components::tile_override_header_text::TileOverrideHeaderTextProps;
use super::components::tile_override_hotkey_slot::TileOverrideHotkeySlotProps;

/// The header owns the name/id column and the hotkey slot beside it.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideHeaderProps {
    pub header_text: TileOverrideHeaderTextProps,
    pub hotkey_slot: TileOverrideHotkeySlotProps,
}
