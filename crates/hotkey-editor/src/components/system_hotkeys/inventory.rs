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
        div { class: "flex flex-col items-center gap-8 w-full max-[1099px]:gap-[0.85rem]",
            p { class: "m-0 text-[2rem] max-w-[90rem] text-center leading-snug font-friz-quadrata uppercase tracking-[0.1em] text-[rgba(255,206,99,0.75)] [text-shadow:1px_1px_0_#000] max-[1099px]:text-[clamp(11px,3vw,14px)] max-[1099px]:tracking-[0.04em] max-[1099px]:leading-[1.35] max-[1099px]:px-[0.25rem] max-[1099px]:max-w-full",
                "Drag a slot onto another to swap their keys."
            }
            InventoryGrid { loaded_keys, editing_section, drag_follower }
        }
    }
}
