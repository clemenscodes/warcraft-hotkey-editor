use dioxus::prelude::*;

/// The system-hotkeys dialog's own shell: the open value driving the dialog and the change
/// handler that writes the open signal, plus the dialog title. `WarcraftDialog` derives its
/// own close from `on_open_change`. The body reads its category tab, editing-section, and
/// inventory drag follower from context, so the shell carries only the header data.
pub(super) struct SystemHotkeysDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
}

impl From<&SystemHotkeysDialogPresentation> for SystemHotkeysDialogShell {
    fn from(model: &SystemHotkeysDialogPresentation) -> Self {
        let open = model.open;
        let on_open_change = model.on_open_change;
        let title = String::from("System Hotkeys");
        Self {
            open,
            on_open_change,
            title,
        }
    }
}
use super::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use super::state::SystemHotkeysDialogState;
use crate::services::overlay_state::context::use_overlay_state;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// The dialog's shaped state: the shared open flag and the change handler mirroring the
/// headless dialog's own close. The active category, editing section, and inventory drag
/// follower live in [`SystemHotkeysDialogState`], which this provides to the subtree via
/// context so the dialog threads no UI signals as props.
pub(super) struct SystemHotkeysDialogPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the shared open signal from the overlay context — the one the toolbar cog button
/// and burger drawer flip — and sets up the dialog's UI signals, providing them as
/// [`SystemHotkeysDialogState`]. Opens on the Inventory tab, with nothing being edited and
/// no drag in progress. The editors read and write the document through the CustomKeys
/// service, not a signal.
pub(super) fn use_system_hotkeys_dialog() -> SystemHotkeysDialogPresentation {
    let overlay = use_overlay_state();
    let dialog_open = overlay.system_hotkeys_open();
    let open = *dialog_open.read();
    let mut change_open = dialog_open;
    let on_open_change = Callback::new(move |is_open| change_open.set(is_open));
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    let editing_section = use_signal::<Option<WarcraftObjectId>>(|| None);
    let drag_follower = use_signal::<Option<InventoryDragFollower>>(|| None);
    let state = SystemHotkeysDialogState::new(active_category, editing_section, drag_follower);
    use_context_provider(|| state);
    SystemHotkeysDialogPresentation {
        open,
        on_open_change,
    }
}
