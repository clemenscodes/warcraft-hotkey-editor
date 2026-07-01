use crate::components::dialogs::system_hotkeys_dialog::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// What the inventory editor needs: the loaded keys it edits, the shared
/// editing-section signal, and the drag follower its grid drives.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryHotkeysViewProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<String>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
