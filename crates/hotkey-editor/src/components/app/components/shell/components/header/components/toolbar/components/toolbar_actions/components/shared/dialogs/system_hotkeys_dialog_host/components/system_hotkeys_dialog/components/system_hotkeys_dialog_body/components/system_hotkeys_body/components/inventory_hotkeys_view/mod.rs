pub mod components;
mod style;

use components::inventory_grid::InventoryGrid;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::SystemHotkeysSectionIntro;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The inventory hotkey editor: the intro caption above the six-slot grid,
/// reorderable by drag. It reads the editing section and drag follower from the
/// dialog state context through its descendants, so it threads nothing.
#[component]
pub fn InventoryHotkeysView() -> Element {
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro { text: "Drag a slot onto another to swap their keys." }
            InventoryGrid {}
        }
    }
}

assert_component!(InventoryHotkeysView);
