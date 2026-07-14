use super::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

#[derive(Clone, Copy, PartialEq)]
pub struct SystemHotkeysDialogState {
    active_category: Signal<SystemHotkeysCategory>,
    editing_section: Signal<Option<WarcraftObjectId>>,
    drag_follower: Signal<Option<InventoryDragFollower>>,
}

impl SystemHotkeysDialogState {
    pub fn new(
        active_category: Signal<SystemHotkeysCategory>,
        editing_section: Signal<Option<WarcraftObjectId>>,
        drag_follower: Signal<Option<InventoryDragFollower>>,
    ) -> Self {
        Self {
            active_category,
            editing_section,
            drag_follower,
        }
    }

    pub(crate) fn active_category(&self) -> Signal<SystemHotkeysCategory> {
        self.active_category
    }

    pub(crate) fn editing_section(&self) -> Signal<Option<WarcraftObjectId>> {
        self.editing_section
    }

    pub(crate) fn drag_follower(&self) -> Signal<Option<InventoryDragFollower>> {
        self.drag_follower
    }
}

pub(crate) fn use_system_hotkeys_dialog_state() -> SystemHotkeysDialogState {
    use_context()
}
