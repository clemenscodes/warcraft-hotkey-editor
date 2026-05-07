use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::system_hotkeys::inventory_grid::{InventoryDragFollower, InventoryGrid};

#[component]
pub(crate) fn InventoryHotkeysView(
    loaded_keys: Signal<Option<CustomKeys>>,
    editing_section: Signal<Option<String>>,
    drag_follower: Signal<Option<InventoryDragFollower>>,
) -> Element {
    rsx! {
        div { class: "wc3-stage",
            p { class: "wc3-stage-hint",
                "Drag a slot onto another to swap their keys."
            }
            InventoryGrid { loaded_keys, editing_section, drag_follower }
        }
    }
}
