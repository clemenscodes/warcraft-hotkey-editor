use super::view::BlockedDropTargetOverlayView;
use dioxus::prelude::*;

/// Mounts only when a drop onto this empty slot is refused (another ability's off-state
/// reserves it); every other empty slot leaves `active` false and early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct BlockedDropTargetOverlayModel {
    pub active: bool,
}

impl From<&BlockedDropTargetOverlayView> for BlockedDropTargetOverlayModel {
    fn from(view: &BlockedDropTargetOverlayView) -> Self {
        let BlockedDropTargetOverlayView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Model for BlockedDropTargetOverlayModel {
    type View = BlockedDropTargetOverlayView;
}
