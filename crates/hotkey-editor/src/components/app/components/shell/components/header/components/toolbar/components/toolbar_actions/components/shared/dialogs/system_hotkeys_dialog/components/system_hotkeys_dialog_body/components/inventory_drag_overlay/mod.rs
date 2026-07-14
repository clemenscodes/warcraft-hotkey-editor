mod model;
mod presentation;
mod view;

pub use view::InventoryDragOverlayView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKey;
use dioxus::prelude::*;
use presentation::use_inventory_drag_overlay;
use model::InventoryDragOverlayModel;
use style::CLASS;
use tw_macro::assert_component;

/// The card that follows the cursor while an inventory slot is dragged. Renders
/// nothing when no drag is in progress; otherwise a framed card at the cursor
/// showing the dragged slot's key.
#[component]
pub fn InventoryDragOverlay(props: InventoryDragOverlayModel) -> Element {
    let Some(view) = use_inventory_drag_overlay(&props) else {
        return rsx! {};
    };
    let placement = view.placement;
    let label = view.label;
    let conflict = false;
    rsx! {
        div {
            class: CLASS,
            style: placement,
            SystemSlotKey {
                label,
                conflict,
            }
        }
    }
}

assert_component!(InventoryDragOverlay);
