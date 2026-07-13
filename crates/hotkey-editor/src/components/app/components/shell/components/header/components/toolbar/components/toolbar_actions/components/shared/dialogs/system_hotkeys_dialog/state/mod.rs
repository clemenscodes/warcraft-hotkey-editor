use super::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The system-hotkeys dialog's own UI state, held in signals and provided once by the
/// dialog: the active category tab, which section is being edited, and the current
/// inventory drag follower. Each field is a `Signal`, so a reader subscribes only to
/// the slice it touches; the whole struct is `Copy`, so a handler captures it cheaply.
/// The dialog subtree reads this from context instead of threading the signals as
/// props. The `open` flag stays a prop on the dialog (the trigger owns it), and the
/// editors read and write the document through the CustomKeys service, not through
/// this state.
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

/// Access the [`SystemHotkeysDialogState`] provided by the dialog. Call from a
/// component or hook body within the dialog subtree (it is a hook). Colocated with the
/// type, mirroring `services/editor_state/context.rs`.
pub(crate) fn use_system_hotkeys_dialog_state() -> SystemHotkeysDialogState {
    use_context()
}
