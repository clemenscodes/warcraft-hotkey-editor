pub mod components;
mod props;

use dioxus::prelude::*;

use crate::components::tile_override::components::tile_override_card::components::override_key_cell::OverrideKeyCell;
use components::tile_override_info_only::TileOverrideInfoOnly;

pub use props::TileOverrideHotkeySlotProps;

/// The hotkey / research-hotkey / passive-note slot in the override header. Renders
/// exactly the one that applies, or nothing.
#[component]
pub fn TileOverrideHotkeySlot(props: TileOverrideHotkeySlotProps) -> Element {
    if props.show_hotkey_field {
        let title = String::from("Hotkey");
        return rsx! {
            OverrideKeyCell {
                label: props.hotkey_label,
                is_editing: props.hotkey_is_editing,
                is_special: props.hotkey_is_special,
                title,
                on_activate: props.on_hotkey_activate,
            }
        };
    }
    if props.show_research_field {
        let title = String::from("Research hotkey");
        return rsx! {
            OverrideKeyCell {
                label: props.research_label,
                is_editing: props.research_is_editing,
                is_special: props.research_is_special,
                title,
                on_activate: props.on_research_activate,
            }
        };
    }
    if props.is_info_only {
        return rsx! {
            TileOverrideInfoOnly { text: "Passive racial ability" }
        };
    }
    rsx! {}
}
