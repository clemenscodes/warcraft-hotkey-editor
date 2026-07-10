use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The seam: this host reads the dialog's UI state from context and hands the signals
/// down to the active category's gallery-rendered editor. The body picks which editor
/// to render from the active category; the editors resolve their bindings from the
/// CustomKeys query, so no loaded keys are threaded here.
pub(super) struct SystemHotkeysBodyModel {
    pub(super) active_category: Signal<SystemHotkeysCategory>,
    pub(super) editing_section: Signal<Option<WarcraftObjectId>>,
    pub(super) drag_follower: Signal<Option<InventoryDragFollower>>,
}

pub(super) fn use_system_hotkeys_body() -> SystemHotkeysBodyModel {
    let state = use_system_hotkeys_dialog_state();
    let active_category = state.active_category();
    let editing_section = state.editing_section();
    let drag_follower = state.drag_follower();
    SystemHotkeysBodyModel {
        active_category,
        editing_section,
        drag_follower,
    }
}
