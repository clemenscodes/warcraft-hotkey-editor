mod logic;
mod props;

use crate::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_section::{
    SystemHotkeysSection, SystemHotkeysSectionProps,
};
use dioxus::prelude::*;
pub use props::InventoryHotkeysViewProps;

/// The inventory hotkey editor: the six-slot grid, reorderable by drag.
#[component]
pub fn InventoryHotkeysView(props: InventoryHotkeysViewProps) -> Element {
    rsx! {
        SystemHotkeysSection { ..SystemHotkeysSectionProps::from(&props) }
    }
}
