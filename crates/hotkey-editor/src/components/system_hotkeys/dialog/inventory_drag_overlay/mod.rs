use dioxus::prelude::*;

use crate::components::system_hotkeys::inventory_grid::InventoryDragFollower;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

#[derive(Props, Clone, PartialEq)]
pub(super) struct InventoryDragOverlayProps {
    pub(super) drag_follower: Signal<Option<InventoryDragFollower>>,
}

#[component]
pub(super) fn InventoryDragOverlay(props: InventoryDragOverlayProps) -> Element {
    let drag_follower = props.drag_follower;
    let follower_option = drag_follower.read().clone();
    let Some(follower) = follower_option else {
        return rsx! {};
    };
    let frame_url = SLOT_FRAME_GOLD;
    let style_value = format!(
        "left: {left}px; top: {top}px; width: {width}px; height: {height}px; \
         --wc3-slot-frame: url('{frame_url}');",
        left = follower.left(),
        top = follower.top(),
        width = follower.width(),
        height = follower.height(),
    );
    let label_text = follower.label().to_string();
    rsx! {
        div { class: "wc3-inventory-drag-follower", style: style_value,
            div { class: "wc3-slot-key", {label_text} }
        }
    }
}
