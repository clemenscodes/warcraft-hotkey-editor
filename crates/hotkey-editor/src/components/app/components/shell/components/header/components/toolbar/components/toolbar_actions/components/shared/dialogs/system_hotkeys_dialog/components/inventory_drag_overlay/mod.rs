pub mod components;
mod hooks;
mod props;
mod style;

use components::inventory_drag_key::{InventoryDragKey, InventoryDragKeyProps};
use dioxus::prelude::*;
use hooks::use_inventory_drag_overlay;
pub use props::InventoryDragOverlayProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(InventoryDragOverlay);

/// The card that follows the cursor while an inventory slot is dragged. Renders
/// nothing when no drag is in progress; otherwise a framed card at the cursor
/// showing the dragged slot's key.
#[component]
pub fn InventoryDragOverlay(props: InventoryDragOverlayProps) -> Element {
    let Some(view) = use_inventory_drag_overlay(&props) else {
        return rsx! {};
    };
    let key = InventoryDragKeyProps::from(&view);
    rsx! {
        div {
            class: CLASS,
            style: view.placement,
            InventoryDragKey { ..key }
        }
    }
}
