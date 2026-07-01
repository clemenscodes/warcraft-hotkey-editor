pub mod components;
mod hooks;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use crate::assert_component;
use components::inventory_drag_overlay::InventoryDragOverlay;
use components::system_hotkeys_body::SystemHotkeysBody;
use components::system_hotkeys_breadcrumbs::SystemHotkeysBreadcrumbs;
use hooks::use_system_hotkeys_dialog;

pub use props::SystemHotkeysDialogProps;

assert_component!(SystemHotkeysDialog);

/// Edits Warcraft III's system and menu hotkeys. A variant of the `Dialog` base:
/// the hook holds the UI signals, and the body composes the shell with the
/// category breadcrumbs, the active category's editor, and the inventory drag
/// follower.
#[component]
pub fn SystemHotkeysDialog(props: SystemHotkeysDialogProps) -> Element {
    let model = use_system_hotkeys_dialog(&props);
    let loaded_keys = props.loaded_keys;
    rsx! {
        Dialog {
            open: model.open,
            title: "System Hotkeys",
            SystemHotkeysBreadcrumbs { active_category: model.active_category }
            SystemHotkeysBody {
                active_category: model.active_category,
                loaded_keys,
                editing_section: model.editing_section,
                drag_follower: model.drag_follower,
            }
            InventoryDragOverlay { drag_follower: model.drag_follower }
        }
    }
}
