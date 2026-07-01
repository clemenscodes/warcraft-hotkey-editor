pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::inventory_drag_key::InventoryDragKey;
use hooks::use_inventory_drag_overlay;
use style::CLASS;

pub use props::InventoryDragOverlayProps;

assert_component!(InventoryDragOverlay);

/// The card that follows the cursor while an inventory slot is dragged. Renders
/// nothing when no drag is in progress; otherwise a framed card at the cursor
/// showing the dragged slot's key.
#[component]
pub fn InventoryDragOverlay(props: InventoryDragOverlayProps) -> Element {
    let Some(view) = use_inventory_drag_overlay(&props) else {
        return rsx! {};
    };
    rsx! {
        div {
            class: CLASS,
            style: view.placement,
            InventoryDragKey { label: view.label }
        }
    }
}
