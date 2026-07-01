use super::super::{InventoryDragFollower, InventoryDragSource};
use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

/// One inventory slot: its index and section, the loaded keys it edits, the shared
/// editing/drag signals the grid coordinates, and the resolved binding map.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryCellProps {
    pub slot_index: usize,
    pub section_id: String,
    pub default_hotkey: u32,
    pub default_modifier: SystemKeybindModifier,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<String>>,
    pub dragging_source: Signal<Option<InventoryDragSource>>,
    pub drop_target: Signal<Option<String>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
    pub binding_map: ReadSignal<SystemBindingMap>,
}
