use super::props::SystemHotkeysDialogProps;
use super::state::SystemHotkeysDialogState;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The dialog's model: only the open flag that drives the shell. The active category,
/// editing section, and inventory drag follower live in [`SystemHotkeysDialogState`],
/// which this hook provides to the subtree via context so the dialog threads no UI
/// signals as props.
pub(super) struct SystemHotkeysDialogModel {
    pub(super) open: Signal<bool>,
}

/// Sets up the dialog's UI signals and provides them as [`SystemHotkeysDialogState`].
/// Opens on the Inventory tab, with nothing being edited and no drag in progress. The
/// editors read and write the document through the CustomKeys service, not a signal.
pub(super) fn use_system_hotkeys_dialog(
    props: &SystemHotkeysDialogProps,
) -> SystemHotkeysDialogModel {
    let open = props.system_hotkeys_open;
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    let editing_section = use_signal::<Option<WarcraftObjectId>>(|| None);
    let drag_follower = use_signal::<Option<InventoryDragFollower>>(|| None);
    let state = SystemHotkeysDialogState::new(active_category, editing_section, drag_follower);
    use_context_provider(|| state);
    SystemHotkeysDialogModel { open }
}
