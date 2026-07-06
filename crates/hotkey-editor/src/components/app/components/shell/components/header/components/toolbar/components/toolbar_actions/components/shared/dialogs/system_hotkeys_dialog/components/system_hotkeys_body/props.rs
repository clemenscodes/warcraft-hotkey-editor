use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::{CustomKeys, WarcraftObjectId};

/// What the body needs to render the active category's editor: the current
/// category, the loaded keys, the editing-section signal the editors share, and
/// the drag follower the inventory editor drives.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBodyProps {
    pub active_category: Signal<SystemHotkeysCategory>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
