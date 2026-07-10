use super::components::inventory_drag_overlay::InventoryDragOverlayProps;
use super::components::system_hotkeys_breadcrumbs::SystemHotkeysBreadcrumbsProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;

/// The seam: this host reads the dialog's UI state from context and shapes the props
/// its two gallery-rendered children need — the breadcrumbs' active category and the
/// overlay's drag follower. `SystemHotkeysBody` reads context itself, so it needs no
/// props from here.
pub(super) struct SystemHotkeysDialogBodyModel {
    pub(super) breadcrumbs: SystemHotkeysBreadcrumbsProps,
    pub(super) overlay: InventoryDragOverlayProps,
}

pub(super) fn use_system_hotkeys_dialog_body() -> SystemHotkeysDialogBodyModel {
    let state = use_system_hotkeys_dialog_state();
    let active_category = state.active_category();
    let drag_follower = state.drag_follower();
    let breadcrumbs = SystemHotkeysBreadcrumbsProps { active_category };
    let overlay = InventoryDragOverlayProps { drag_follower };
    SystemHotkeysDialogBodyModel {
        breadcrumbs,
        overlay,
    }
}
