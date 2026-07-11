use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The seam: this host reads the dialog's UI state from context and threads the raw
/// values its two gallery-rendered children need — the breadcrumbs' active category
/// signal and the overlay's drag follower signal. `SystemHotkeysBody` reads context
/// itself, so it needs no props from here.
pub(super) struct SystemHotkeysDialogBodyModel {
    pub(super) active_category: Signal<SystemHotkeysCategory>,
    pub(super) drag_follower: Signal<Option<InventoryDragFollower>>,
}

pub(super) fn use_system_hotkeys_dialog_body() -> SystemHotkeysDialogBodyModel {
    let state = use_system_hotkeys_dialog_state();
    let active_category = state.active_category();
    let drag_follower = state.drag_follower();
    SystemHotkeysDialogBodyModel {
        active_category,
        drag_follower,
    }
}
