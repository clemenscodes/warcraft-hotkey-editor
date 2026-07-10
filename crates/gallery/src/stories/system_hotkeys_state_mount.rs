use dioxus::prelude::*;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use hotkey_editor::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::SystemHotkeysDialogState;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// App-specific story decorator: provides the [`SystemHotkeysDialogState`] the
/// system-hotkeys dialog normally supplies to its subtree, so a story can render the
/// dialog's context-reading bodies (`SystemHotkeysDialogBody`, `SystemHotkeysBody`)
/// outside the dialog. It seeds the same defaults the dialog's own hook does — the
/// Inventory tab, nothing being edited, no drag in progress. Mirrors
/// [`super::keys_mount::CustomKeysMount`] for the system-hotkeys UI-state context, and
/// stays with the stories rather than the domain-agnostic gallery framework.
#[component]
pub fn SystemHotkeysStateMount(children: Element) -> Element {
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    let editing_section = use_signal::<Option<WarcraftObjectId>>(|| None);
    let drag_follower = use_signal::<Option<InventoryDragFollower>>(|| None);
    let state = SystemHotkeysDialogState::new(active_category, editing_section, drag_follower);
    use_context_provider(|| state);
    rsx! {
        {children}
    }
}
