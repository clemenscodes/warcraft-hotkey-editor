pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::tile_override_header_text::TileOverrideHeaderText;
use components::tile_override_hotkey_slot::TileOverrideHotkeySlot;
use style::CLASS;
use tw_macro::assert_component;

pub use props::TileOverrideHeaderProps;

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

assert_component!(TileOverrideHeader);
