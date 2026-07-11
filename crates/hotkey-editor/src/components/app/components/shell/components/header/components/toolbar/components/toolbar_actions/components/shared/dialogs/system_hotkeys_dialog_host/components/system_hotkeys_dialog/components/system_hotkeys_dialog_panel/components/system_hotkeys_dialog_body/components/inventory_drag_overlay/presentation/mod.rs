use super::model::InventoryDragOverlayModel;
use dioxus::prelude::*;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

/// The follower's shaped view: the inline placement style (position, size, and the
/// gold-frame CSS variable the border-image reads) and the key label it shows.
pub(super) struct DragFollowerView {
    pub(super) placement: String,
    pub(super) label: String,
}

/// Shapes the drag follower for rendering, or returns `None` when nothing is being
/// dragged (the overlay renders nothing). The placement is genuinely dynamic — it
/// tracks the cursor — so it stays an inline `style`, the one thing utilities can't
/// express.
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
