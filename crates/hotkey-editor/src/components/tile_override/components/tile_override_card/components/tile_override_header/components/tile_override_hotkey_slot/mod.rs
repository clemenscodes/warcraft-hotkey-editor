pub mod components;
mod logic;
mod props;

use dioxus::prelude::*;

use crate::components::tile_override::components::tile_override_card::components::override_key_cell::OverrideKeyCell;
use components::tile_override_info_only::TileOverrideInfoOnly;
use logic::TileOverrideHotkeySlotDispatch;

pub use props::TileOverrideHotkeySlotProps;

/// The hotkey / research-hotkey / passive-note slot in the override header. Renders
/// exactly the one that applies, or nothing.
#[component]
pub fn TileOverrideHotkeySlot(props: TileOverrideHotkeySlotProps) -> Element {
    let dispatch = TileOverrideHotkeySlotDispatch::from(&props);
    if let Some(key_cell) = dispatch.key_cell {
        return rsx! {
            OverrideKeyCell { ..key_cell }
        };
    }
    if let Some(info) = dispatch.info {
        return rsx! {
            TileOverrideInfoOnly { ..info }
        };
    }
    rsx! {}
}
