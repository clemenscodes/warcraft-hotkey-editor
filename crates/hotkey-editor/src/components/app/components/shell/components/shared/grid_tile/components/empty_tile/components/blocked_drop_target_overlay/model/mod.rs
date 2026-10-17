use super::view::BlockedDropTargetOverlayView;
use dioxus::prelude::*;

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

impl ddd::Layered for BlockedDropTargetOverlayModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BlockedDropTargetOverlayModel {
    type View = BlockedDropTargetOverlayView;
}
