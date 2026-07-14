pub mod components;
mod presentation;
mod style;

use components::inventory_grid::InventoryGrid;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::SystemHotkeysSectionIntro;
use dioxus::prelude::*;
use presentation::{use_inventory_hotkeys_view, InventoryHotkeysViewModel};
use style::CLASS;
use tw_macro::assert_component;

/// The inventory hotkey editor: the domain-supplied intro caption above the six-slot
/// grid, reorderable by drag. It reads the editing section and drag follower from the
/// dialog state context through its descendants, so it threads nothing.
#[component]
pub fn InventoryHotkeysView() -> Element {
    let InventoryHotkeysViewModel { caption } = use_inventory_hotkeys_view();
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro {
                text: caption,
            }
            InventoryGrid {
            


            }
        }
    }
}

assert_component!(InventoryHotkeysView);
