use super::model::InventoryDragOverlayModel;
use dioxus::prelude::*;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

pub(super) struct DragFollowerView {
    pub(super) placement: String,
    pub(super) label: String,
}

pub(super) fn use_inventory_drag_overlay(
    props: &InventoryDragOverlayModel,
) -> Option<DragFollowerView> {
    let follower = props.drag_follower.read().clone()?;
    let frame_url = SLOT_FRAME_GOLD;
    let placement = format!(
        "left: {left}px; top: {top}px; width: {width}px; height: {height}px; \
         --wc3-slot-frame: url('{frame_url}');",
        left = follower.left(),
        top = follower.top(),
        width = follower.width(),
        height = follower.height(),
    );
    let label = follower.label().to_string();
    let view = DragFollowerView { placement, label };
    Some(view)
}
