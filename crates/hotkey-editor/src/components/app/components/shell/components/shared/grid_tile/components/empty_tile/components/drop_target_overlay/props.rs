use super::view::DropTargetOverlayView;
use dioxus::prelude::*;

/// Mounts only while the empty slot is the drag's drop-target candidate; every other
/// empty slot leaves `active` false and early-returns, so the overlay's presence is the
/// drop-target signal the tile root's border keys off.
#[derive(Props, Clone, PartialEq)]
pub struct DropTargetOverlayProps {
    pub active: bool,
}

impl From<&DropTargetOverlayView> for DropTargetOverlayProps {
    fn from(view: &DropTargetOverlayView) -> Self {
        let DropTargetOverlayView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Props for DropTargetOverlayProps {
    type View = DropTargetOverlayView;
}
