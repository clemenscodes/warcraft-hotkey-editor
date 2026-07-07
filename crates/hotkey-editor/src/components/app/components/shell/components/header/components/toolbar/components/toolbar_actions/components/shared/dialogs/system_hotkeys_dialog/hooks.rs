use super::props::SystemHotkeysDialogProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The dialog's UI state, all held in signals: the open flag that drives the
/// shell, the active category tab, which section is being edited, and the current
/// inventory drag follower. All UI state — the editors read and write the document
/// through the CustomKeys service, not a threaded signal.
pub(super) struct SystemHotkeysDialogModel {
    pub(super) open: Signal<bool>,
    pub(super) active_category: Signal<SystemHotkeysCategory>,
    pub(super) editing_section: Signal<Option<WarcraftObjectId>>,
    pub(super) drag_follower: Signal<Option<InventoryDragFollower>>,
}

/// Sets up the dialog's UI signals. Opens on the Inventory tab, with nothing being
/// edited and no drag in progress.
pub(super) fn use_system_hotkeys_dialog(
    props: &SystemHotkeysDialogProps,
) -> SystemHotkeysDialogModel {
    let open = props.system_hotkeys_open;
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    let editing_section = use_signal::<Option<WarcraftObjectId>>(|| None);
    let drag_follower = use_signal::<Option<InventoryDragFollower>>(|| None);
    SystemHotkeysDialogModel {
        open,
        active_category,
        editing_section,
        drag_follower,
    }
}
