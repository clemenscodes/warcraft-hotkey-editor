pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::tile_override_header_text::TileOverrideHeaderText;
use components::tile_override_hotkey_slot::TileOverrideHotkeySlot;
use style::CLASS;

pub use props::TileOverrideHeaderProps;

assert_component!(TileOverrideHeader);

/// The header row of the override panel: the name/id column and the hotkey slot.
#[component]
pub fn TileOverrideHeader(props: TileOverrideHeaderProps) -> Element {
    rsx! {
        div { class: CLASS,
            TileOverrideHeaderText { ..props.header_text }
            TileOverrideHotkeySlot { ..props.hotkey_slot }
        }
    }
}
