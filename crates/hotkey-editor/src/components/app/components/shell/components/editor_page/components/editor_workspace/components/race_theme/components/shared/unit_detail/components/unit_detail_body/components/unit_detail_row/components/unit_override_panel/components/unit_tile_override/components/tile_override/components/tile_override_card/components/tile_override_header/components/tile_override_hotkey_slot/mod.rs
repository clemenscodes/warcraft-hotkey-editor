pub mod components;
mod logic;
mod props;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key::OverrideKey;
use components::tile_override_info_only::TileOverrideInfoOnly;
use logic::TileOverrideHotkeySlotDispatch;
use tw_macro::assert_component;

pub use props::TileOverrideHotkeySlotProps;

/// The hotkey / research-hotkey / passive-note slot in the override header. Renders
/// exactly the one that applies, or nothing.
#[component]
pub fn TileOverrideHotkeySlot(props: TileOverrideHotkeySlotProps) -> Element {
    let dispatch = TileOverrideHotkeySlotDispatch::from(&props);
    if let Some(key_cell) = dispatch.key_cell {
        return rsx! {
            OverrideKey { ..key_cell }
        };
    }
    if let Some(info) = dispatch.info {
        return rsx! {
            TileOverrideInfoOnly { ..info }
        };
    }
    rsx! {}
}

assert_component!(TileOverrideHotkeySlot);
