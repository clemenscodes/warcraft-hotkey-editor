pub mod components;
mod logic;
mod props;
mod style;

use components::inventory_grid::{InventoryGrid, InventoryGridProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::{SystemHotkeysSectionIntro, SystemHotkeysSectionIntroProps};
use dioxus::prelude::*;
pub use props::InventoryHotkeysViewProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(InventoryHotkeysView);

/// The inventory hotkey editor: the intro caption above the six-slot grid,
/// reorderable by drag.
#[component]
pub fn InventoryHotkeysView(props: InventoryHotkeysViewProps) -> Element {
    let intro = SystemHotkeysSectionIntroProps::from(&props);
    let grid = InventoryGridProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro { ..intro }
            InventoryGrid { ..grid }
        }
    }
}
