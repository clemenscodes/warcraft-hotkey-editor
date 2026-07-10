mod hooks;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::shared::system_slot_key::{
    SystemSlotKey, SystemSlotKeyProps,
};
use dioxus::prelude::*;
use hooks::use_inventory_drag_overlay;
pub use props::InventoryDragOverlayProps;
use style::CLASS;
use tw_macro::assert_component;

/// The card that follows the cursor while an inventory slot is dragged. Renders
/// nothing when no drag is in progress; otherwise a framed card at the cursor
/// showing the dragged slot's key.
#[component]
pub fn InventoryDragOverlay(props: InventoryDragOverlayProps) -> Element {
    let Some(view) = use_inventory_drag_overlay(&props) else {
        return rsx! {};
    };
    let key = SystemSlotKeyProps::from(&view);
    rsx! {
        div {
            class: CLASS,
            style: view.placement,
            SystemSlotKey { ..key }
        }
    }
}

assert_component!(InventoryDragOverlay);
