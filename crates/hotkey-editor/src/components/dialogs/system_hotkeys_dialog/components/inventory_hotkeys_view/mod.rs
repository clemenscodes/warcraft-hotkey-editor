mod props;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::inventory_grid::InventoryGrid;
use crate::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_section::SystemHotkeysSection;
use dioxus::prelude::*;
pub use props::InventoryHotkeysViewProps;
assert_component!(InventoryHotkeysView);

/// The inventory hotkey editor: the six-slot grid, reorderable by drag.
#[component]
pub fn InventoryHotkeysView(props: InventoryHotkeysViewProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let drag_follower = props.drag_follower;
    rsx! {
        SystemHotkeysSection { intro: "Drag a slot onto another to swap their keys.",
            InventoryGrid { loaded_keys, editing_section, drag_follower }
        }
    }
}
