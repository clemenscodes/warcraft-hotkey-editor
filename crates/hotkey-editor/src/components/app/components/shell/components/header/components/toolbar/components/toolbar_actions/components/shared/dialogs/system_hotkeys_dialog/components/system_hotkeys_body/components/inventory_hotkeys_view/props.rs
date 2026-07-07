use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// What the inventory editor needs: the shared editing-section signal and the drag
/// follower its grid drives. Its cells resolve their bindings from the CustomKeys
/// query.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryHotkeysViewProps {
    pub editing_section: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
