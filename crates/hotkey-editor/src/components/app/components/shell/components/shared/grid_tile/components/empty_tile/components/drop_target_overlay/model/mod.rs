use super::view::DropTargetOverlayView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DropTargetOverlayModel {
    pub active: bool,
}

impl From<&DropTargetOverlayView> for DropTargetOverlayModel {
    fn from(view: &DropTargetOverlayView) -> Self {
        let DropTargetOverlayView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Layered for DropTargetOverlayModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for DropTargetOverlayModel {
    type View = DropTargetOverlayView;
}
