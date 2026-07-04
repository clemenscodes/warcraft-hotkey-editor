use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
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
